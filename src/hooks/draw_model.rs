use crate::entity::{Entity, Player, PlayerRef};
use crate::{state, State};
use elysium_math::Matrix3x4;
use elysium_sdk::material;
use elysium_sdk::model::{DrawModelState, ModelRender, ModelRenderInfo};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const ORANGE: [f32; 4] = [1.0, 0.5, 0.0, 0.5];
const ORANGE_2: [f32; 4] = [1.0, 0.3, 0.0, 0.5];
const ORANGE_3: [f32; 4] = [1.0, 0.1, 0.0, 0.5];

struct Layer {
    color: [f32; 4],
    material: Option<&'static material::Material>,
    ignore_z: bool,
    wireframe: bool,
}

impl Layer {
    pub fn new() -> Self {
        Self {
            color: BLACK,
            material: None,
            ignore_z: false,
            wireframe: false,
        }
    }
}

/// Model render layers.
pub struct Layers {
    layers: Vec<Layer>,
}

impl Layers {
    pub fn new() -> Self {
        let layers = vec![Layer::new()];

        Self { layers }
    }

    fn last(&mut self) -> &mut Layer {
        // There is always at least one item in layers.
        unsafe { self.layers.last_mut().unwrap_unchecked() }
    }

    pub fn color(&mut self, color: [f32; 4]) -> &mut Self {
        self.last().color = color;
        self
    }

    pub fn material(&mut self, material: Option<&'static material::Material>) -> &mut Self {
        self.last().material = material;
        self
    }

    pub fn ignore_z(&mut self, ignore_z: bool) -> &mut Self {
        self.last().ignore_z = ignore_z;
        self
    }

    pub fn wireframe(&mut self, wireframe: bool) -> &mut Self {
        self.last().wireframe = wireframe;
        self
    }

    pub fn layer(&mut self) -> &mut Self {
        self.layers.push(Layer::new());
        self
    }

    pub fn render(
        self,
        model_render: &mut ModelRender,
        context: *mut u8,
        draw_state: *mut DrawModelState,
        info: *const ModelRenderInfo,
        bone_to_world: *const Matrix3x4,
    ) {
        let state = State::get();
        let draw_model_original = state.hooks.draw_model.unwrap();

        for layer in self.layers {
            match layer.material {
                Some(material) => {
                    material.set_rgba(layer.color);
                    material.set_flag(material::Flag::IGNORE_Z, layer.ignore_z);
                    material.set_flag(material::Flag::WIREFRAME, layer.wireframe);

                    model_render.override_material(material);
                }
                None => {
                    model_render.reset_material();
                }
            }

            unsafe {
                (draw_model_original)(model_render, context, draw_state, info, bone_to_world);
            }

            model_render.reset_material();
        }
    }
}

#[inline]
unsafe fn draw_model_inner(
    model_render: &mut ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) -> Option<()> {
    let state = State::get();
    let interfaces = state.interfaces.as_ref()?;
    let entity_list = &interfaces.entity_list;
    let model_info = &interfaces.model_info;
    let local = PlayerRef::from_raw(state.local.player)?;

    let _flat = state::material::FLAT.load_unchecked();
    let glow = state::material::GLOW.load_unchecked();

    let info = info.as_ref()?;
    let name = info.name(&model_info)?;
    let mut layers = Layers::new();

    if name.starts_with("models/player") {
        let index = info.entity_index;
        let player = PlayerRef::from_raw(entity_list.entity(index))?;

        if index != local.index() {
            let is_enemy = player.is_enemy();
            let is_bot = player.flags().is_bot();

            match (is_enemy, is_bot) {
                // Our own team.
                (false, _) => {
                    layers.layer().color(ORANGE).material(Some(glow));
                }

                // Enemy players.
                (true, false) => {
                    layers
                        .layer()
                        .color(ORANGE_3)
                        .material(Some(glow))
                        .ignore_z(true);
                }

                // Enemy bots.
                (true, true) => {
                    layers
                        .layer()
                        .color(ORANGE_2)
                        .material(Some(glow))
                        .ignore_z(true);
                }
            };
        }
    } else {
        layers.layer().color(ORANGE).material(Some(glow));

        if name.starts_with("models/weapons/v_") {
            layers.ignore_z(true);
        }
    }

    layers.render(model_render, context, draw_state, info, bone_to_world);

    Some(())
}

/// `DrawModelExecute` hook.
pub unsafe extern "C" fn draw_model(
    model_render: &mut ModelRender,
    context: *mut u8,
    draw_state: *mut DrawModelState,
    info: *const ModelRenderInfo,
    bone_to_world: *const Matrix3x4,
) {
    draw_model_inner(model_render, context, draw_state, info, bone_to_world);
}
