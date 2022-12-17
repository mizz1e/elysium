use crate::{global, material, Color, Config, IMaterial, Mat4x3, MaterialFlag, Ptr};
use bevy::prelude::*;
use std::mem::MaybeUninit;
use std::{ffi, ptr};

bitflags::bitflags! {
    /// `game/client/clientleafsystem.cpp`.
    #[repr(transparent)]
    pub struct RenderFlags: u16 {
        const DISABLE_RENDERING = 1 << 0;
        const HAS_CHANGED = 1 << 1;
        const ALTERNATIVE_SORTING = 1 << 2;
        const RENDER_WITH_VIEWMODELS = 1 << 3;
        const BLOAT_BOUNDS = 1 << 4;
        const BOUNDS_VALID = 1 << 5;
        const BOUNDS_ALWAYS_RECOMPUTE = 1 << 6;
        const IS_SPRITE = 1 << 7;
        const FORCE_OPAQUE_PASS = 1 << 8;
    }
}

/// `game/client/clientleafsystem.cpp`.
#[repr(C)]
pub struct RenderableInfo_t {
    pub renderable: *mut u8,
    pub alpha_property: *mut u8,
    pub enum_count: i32,
    pub render_frame: i32,
    pub first_shadow: u16,
    pub leaf_list: u16,
    pub area: i16,
    pub flags: RenderFlags,
    pub flags2: RenderFlags,
    pub bloated_abs_mins: Vec3,
    pub bloated_abs_maxs: Vec3,
    pub abs_mins: Vec3,
    pub abs_maxs: Vec3,
    _pad0: [MaybeUninit<u8>; 4],
}

#[derive(Resource)]
pub struct DrawModelExecute(
    pub(crate)  unsafe extern "C" fn(
        this: *mut u8,
        render_context: *const internal::IMatRenderContext,
        draw_model_state: *const internal::DrawModelState_t,
        model_render_info: *const internal::ModelRenderInfo_t,
        custom_bone_to_world: *const [Mat4x3; 256],
    ),
);

#[derive(Resource)]
pub struct IVModelRender {
    pub(crate) ptr: Ptr,
}

impl IVModelRender {
    pub(crate) unsafe fn setup(&self) {
        global::with_app_mut(|app| {
            app.insert_resource(DrawModelExecute(
                self.ptr.vtable_replace(21, draw_model_execute),
            ));
        });
    }

    fn override_material(&self, material: Option<&IMaterial>) {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            material: *mut u8,
            override_kind: ffi::c_int,
            material_index: ffi::c_int,
        ) = unsafe { self.ptr.vtable_entry(1) };

        let ptr = match material {
            Some(material) => material.ptr.as_ptr(),
            None => ptr::null_mut(),
        };

        unsafe { (method)(self.ptr.as_ptr(), ptr, 0, -1) }
    }

    fn draw_model_execute(
        &self,
        render_context: *const internal::IMatRenderContext,
        draw_model_state: *const internal::DrawModelState_t,
        model_render_info: *const internal::ModelRenderInfo_t,
        custom_bone_to_world: *const [Mat4x3; 256],
    ) {
        let method = global::with_resource::<DrawModelExecute, _>(|method| method.0);

        unsafe {
            (method)(
                self.ptr.as_ptr(),
                render_context,
                draw_model_state,
                model_render_info,
                custom_bone_to_world,
            )
        }
    }
}

unsafe extern "C" fn draw_model_execute(
    this: *mut u8,
    render_context: *const internal::IMatRenderContext,
    draw_model_state: *const internal::DrawModelState_t,
    model_render_info: *const internal::ModelRenderInfo_t,
    custom_bone_to_world: *const [Mat4x3; 256],
) {
    debug_assert!(!this.is_null());

    global::with_app(|app| {
        let config = app.world.resource::<Config>();
        let model_render = app.world.resource::<IVModelRender>();
        let glow = app.world.resource::<material::Glow>();
        let glow = &glow.0;
        let flat = app.world.resource::<material::Flat>();
        let flat = &flat.0;

        flat.set_flag(MaterialFlag::IGNORE_Z, true);
        flat.set_color(Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        });

        glow.set_flag(MaterialFlag::WIREFRAME, true);
        glow.set_flag(MaterialFlag::IGNORE_Z, true);
        glow.set_color(Color {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 0.1,
        });

        model_render.override_material(Some(flat));

        model_render.draw_model_execute(
            render_context,
            draw_model_state,
            model_render_info,
            custom_bone_to_world,
        );

        model_render.override_material(Some(glow));

        model_render.draw_model_execute(
            render_context,
            draw_model_state,
            model_render_info,
            custom_bone_to_world,
        );

        model_render.override_material(None);
    });
}

mod internal {
    #[repr(C)]
    pub struct IMatRenderContext;

    #[repr(C)]
    pub struct DrawModelState_t;

    #[repr(C)]
    pub struct ModelRenderInfo_t;
}
