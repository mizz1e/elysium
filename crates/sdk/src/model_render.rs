use crate::{global, Mat4x3, Ptr};
use bevy::prelude::*;
use std::ffi;

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
    #[inline]
    pub(crate) unsafe fn setup(&self) {
        global::with_app_mut(|app| {
            app.insert_resource(DrawModelExecute(
                self.ptr.vtable_replace(21, draw_model_execute),
            ));
        });
    }

    #[inline]
    fn override_material(&self, material: *mut u8) {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            material: *mut u8,
            override_kind: ffi::c_int,
            material_index: ffi::c_int,
        ) = unsafe { self.ptr.vtable_entry(1) };

        unsafe { (method)(self.ptr.as_ptr(), material, 0, -1) }
    }

    #[inline]
    fn draw_model_execute(
        &self,
        render_context: *const internal::IMatRenderContext,
        draw_model_state: *const internal::DrawModelState_t,
        model_render_info: *const internal::ModelRenderInfo_t,
        custom_bone_to_world: *const [Mat4x3; 256],
    ) {
        let method = global::with_app(|app| app.world.resource::<DrawModelExecute>().0);

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
        let model_render = app.world.resource::<IVModelRender>();

        model_render.draw_model_execute(
            render_context,
            draw_model_state,
            model_render_info,
            custom_bone_to_world,
        );
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
