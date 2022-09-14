use crate::model::{ModelInfo, ModelRender};
use crate::{Client, Engine, EntityList, InputSystem, Trace};
use crate::{Console, Debug, Effects, Events, Filesystem, InputInternal};
use crate::{Kinds, Localize, MaterialSystem, Movement, Panel, Panorama, Physics};
use crate::{Prediction, Sound, Surface, VGui};

macro_rules! libraries {
    ($($name:ident => $string:literal),*) => {
        /// library
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum LibraryKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                $name,
            )*
        }

        impl LibraryKind {
            /// Return's the path of this library.
            #[inline]
            pub const fn path(&self) -> &'static str {
                match self {
                    $(LibraryKind::$name => $string,)*
                }
            }
        }
    }
}

macro_rules! interfaces {
    ($(($ident:ident, $field:ident) => ($library:ident, $string:literal)),*) => {
        /// interface
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #[non_exhaustive]
        pub enum InterfaceKind {
            $(
                #[doc = "`"]
                #[doc = $string]
                #[doc = "`"]
                $ident,
            )*
        }

        impl InterfaceKind {
            /// Returns the interface's name.
            #[inline]
            pub const fn name(&self) -> &'static str {
                match self {
                    $(InterfaceKind::$ident => $string,)*
                }
            }

            /// Returns the library this interface is in.
            #[inline]
            pub const fn library(&self) -> LibraryKind {
                match self {
                    $(InterfaceKind::$ident => LibraryKind::$library,)*
                }
            }
        }

        //#[derive(Debug)]
        #[non_exhaustive]
        pub struct Interfaces {
            $(pub $field: &'static mut $ident,)*
        }

        impl Interfaces {
            #[inline]
            pub unsafe fn from_loader<L>(mut loader: L) -> Self
            where
                L: FnMut(InterfaceKind) -> *mut u8,
            {
                Self { $($field: &mut *loader(InterfaceKind::$ident).cast(),)* }
            }
        }
    }
}

libraries! {
    Client => "./csgo/bin/linux64/client_client.so",
    Engine => "./bin/linux64/engine_client.so",
    Filesystem => "./bin/linux64/filesystem_stdio_client.so",
    Input => "./bin/linux64/inputsystem_client.so",
    Localize => "./bin/linux64/localize_client.so",
    Matchmaking => "./csgo/bin/linux64/matchmaking_client.so",
    MaterialSystem => "./bin/linux64/materialsystem_client.so",
    Panorama => "./bin/linux64/panorama_gl_client.so",
    Physics => "./bin/linux64/vphysics_client.so",
    ServerBrowser => "./bin/linux64/serverbrowser_client.so",
    Surface => "./bin/linux64/vguimatsurface_client.so",
    Tier0 => "./bin/linux64/libtier0_client.so",
    VGui => "./bin/linux64/vgui2_client.so"
}

interfaces! {
    (Client, client) => (Client, "VClient"),
    (Console, console) => (MaterialSystem, "VEngineCvar"),
    (Debug, debug) => (Engine, "VDebugOverlay"),
    (Effects, effects) => (Engine, "VEngineEffects"),
    (Engine, engine) => (Engine, "VEngineClient"),
    (EntityList, entity_list) => (Client, "VClientEntityList"),
    (Events, events) => (Engine, "GAMEEVENTSMANAGER002"),
    (Filesystem, filesystem) => (Filesystem, "VFileSystem"),
    (InputInternal, input_internal) => (VGui, "VGUI_InputInternal"),
    (InputSystem, input_system) => (Input, "InputSystemVersion"),
    (Kinds, kinds) => (Matchmaking, "VENGINE_GAMETYPES_VERSION002"),
    (Localize, localize) => (Localize, "Localize_"),
    (MaterialSystem, material_system) => (MaterialSystem, "VMaterialSystem"),
    (ModelInfo, model_info) => (Engine, "VModelInfoClient"),
    (ModelRender, model_render) => (Engine, "VEngineModel"),
    (Movement, movement) => (Client, "GameMovement"),
    (Panel, panel) => (VGui, "VGUI_Panel"),
    (Panorama, panorama) => (Panorama, "PanoramaUIEngine001"),
    (Physics, physics) => (Physics, "VPhysicsSurfaceProps"),
    (Prediction, prediction) => (Client, "VClientPrediction001"),
    (Sound, sound) => (Engine, "IEngineSoundClient"),
    (Surface, surface) => (Surface, "VGUI_Surface"),
    (Trace, trace) => (Engine, "EngineTraceClient"),
    (VGui, vgui) => (Engine, "VEngineVGui")
}
