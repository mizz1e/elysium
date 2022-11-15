use crate::entity::{Player, PlayerRef};
use crate::hooks::{EngineViewAngle, LocalViewAngle};
use bevy::prelude::*;
use std::ffi;

const FRAME_NET_UPDATE_END: ffi::c_int = 4;
const FRAME_RENDER_START: ffi::c_int = 5;

#[derive(Resource)]
pub struct FrameStageNotify(pub unsafe extern "C" fn(*const u8, ffi::c_int));

pub unsafe extern "C" fn frame_stage_notify(this: *const u8, frame: ffi::c_int) {
    debug_assert!(!this.is_null());

    let frame_stage_notify = elysium_sdk::with_app_mut(|app| {
        match frame {
            FRAME_RENDER_START => {
                let state = crate::State::get();
                let engine = &state.interfaces.as_ref().unwrap().engine;
                let entity_list = &state.interfaces.as_ref().unwrap().entity_list;
                let input = state.input.as_mut().unwrap();
                let engine_view_angle = engine.view_angle();

                state.local.player = entity_list.local_player(engine).cast();

                if state.local.player.is_null() {
                    state.local.reset();
                } else {
                    let mut local_player = PlayerRef::from_raw(state.local.player).unwrap();

                    state.location = Some(local_player.location_name());

                    // is it even enabled
                    let mut thirdperson = state.local.thirdperson.enabled;

                    // certain spectator modes alongside thirdperson break the camera.
                    thirdperson &= !local_player.observer_mode().breaks_thirdperson();

                    // use first person while scoped.
                    thirdperson &= !local_player.is_scoped();

                    // apply current toggle state.
                    thirdperson &= state.local.thirdperson.toggle;

                    input.thirdperson = thirdperson;

                    if thirdperson {
                        let local_view_angle = app.world.resource::<LocalViewAngle>().0;

                        local_player.set_view_angle(local_view_angle);
                    } else {
                        local_player.set_view_angle(engine_view_angle + Vec3::new(0.0, 0.0, 15.0));
                    }
                }

                app.insert_resource(EngineViewAngle(engine.view_angle()));
                app.update();
            }
            _ => {}
        }

        app.world.resource::<FrameStageNotify>().0
    });

    (frame_stage_notify)(this, frame)
}
