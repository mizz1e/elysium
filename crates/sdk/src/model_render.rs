use crate::{global, material, Color, Config, IMaterial, Mat4x3, MaterialFlag, Ptr};
use bevy::prelude::*;
use std::{ffi, ptr};

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

        model_render.draw_model_execute(
            render_context,
            draw_model_state,
            model_render_info,
            custom_bone_to_world,
        );

        glow.set_flag(MaterialFlag::IGNORE_Z, true);
        glow.set_color(config.cham_color);

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
