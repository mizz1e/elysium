use bevy::prelude::*;
use bevy_source::prelude::*;
use leafwing_input_manager::prelude::*;
use std::f32::consts;

#[bevy_main]
fn main() {
    App::new()
        .add_default_schedules()
        .add_plugins(DefaultPlugins)
        .add_plugin(SourcePlugin)
        .add_plugin(InputPlugin)
        .run();
}

/// Extra engine camera data.
#[derive(Component, Default)]
pub struct Engine {
    pub rotation: Vec2,
}

/// The entity which the camera should follow.
#[derive(Component, Default)]
pub struct Followed {
    pub transform: Transform,
}

/// Engine input.
#[derive(Actionlike, Clone, Copy, Debug, Eq, PartialEq)]
pub enum Input {
    Look,
    Move,
}

/// Engine input plugin.
#[derive(Default)]
pub struct InputPlugin;

impl Input {
    /// Return the default input bindings.
    pub fn default_map() -> InputMap<Input> {
        let mut map = InputMap::default();

        map.insert(DualAxis::mouse_motion(), Input::Look);
        map.insert(VirtualDPad::wasd(), Input::Move);
        map
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Input>::default())
            .add_startup_system(setup_input_system)
            .add_system(input_system);
    }
}

impl Engine {
    /// Return the current rotation.
    pub fn rotation(&self) -> Quat {
        let Vec2 { x, y } = self.rotation;

        Quat::from_euler(EulerRot::YXZ, x, y, 0.0)
    }

    /// Return the current engine rotation as a source rotation (in degrees, right-handed z-up).
    pub fn source_angle(&self) -> Vec3 {
        let Vec2 { x: yaw, y: pitch } = self.rotation * 1.0_f32.to_degrees();

        // this is not the same as `value % 360.0`;
        let mut pitch = libm::remainderf(pitch, 360.0);
        let yaw = libm::remainderf(yaw, 360.0);

        // invert pitch
        if pitch != 0.0 {
            pitch = -pitch;
        }

        Vec3::new(pitch, yaw, 0.0)
    }

    /// Set the current rotation from a source rotation.
    pub fn set_source_rotation(&mut self, rotation: Vec3) {
        let Vec3 {
            x: mut pitch,
            y: yaw,
            z: _roll,
        } = rotation * 1.0_f32.to_radians();

        if pitch != 0.0 {
            pitch = -pitch;
        }

        self.rotation = Vec2::new(yaw, pitch);
    }

    /// Update the rotation with a mouse delta.
    pub fn update_rotation(&mut self, delta: Vec2) {
        let pitch_max = 89.0_f32.to_radians();

        // add the delta
        self.rotation -= delta * 0.005;

        // normalize
        self.rotation.x = libm::remainderf(self.rotation.x, consts::TAU);
        self.rotation.y = libm::remainderf(self.rotation.y, consts::TAU);

        // clamp
        self.rotation.y = self.rotation.y.clamp(-pitch_max, pitch_max);
    }
}

/// Setup engine input handling.
pub fn setup_input_system(mut commands: Commands) {
    commands.spawn((
        Engine::default(),
        InputManagerBundle::<Input> {
            input_map: Input::default_map(),
            ..default()
        },
    ));
}

/// Engine input handling.
pub fn input_system(mut query: Query<(&ActionState<Input>, &mut Engine, &mut Transform)>) {
    let Ok((input_state, mut engine, mut eye_transform)) = query.get_single_mut() else {
        return;
    };

    if let Some(axis) = input_state.axis_pair(Input::Look) {
        engine.update_rotation(axis.xy());

        println!("rotation = {:?}", engine.rotation());
    }

    if let Some(axis) = input_state.axis_pair(Input::Move) {
        let movement: Vec3 = axis.xy().extend(0.0);

        println!("movement = {movement:?}");
    }
}
