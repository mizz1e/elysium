use super::{ffi, vtable_export, vtable_validate, NetworkChannel, SteamAPIContext, SteamId};
use cake::ffi::VTablePad;
use elysium_math::{Matrix3x4, Vec3};
use std::ffi::OsStr;
use std::mem::MaybeUninit;

/// player information
#[repr(C)]
pub struct PlayerInfo {
    pub version: u64,
    pub steam_id: u64,
    pub name: [u8; 128],
    pub user_id: i32,
    pub guid: [u8; 33],
    pub friends_id: u32,
    pub fake_player: bool,
    pub hltv: bool,
    pub custom_files: [i32; 4],
    pub files_downloaded: u8,
}

#[repr(C)]
struct VTable {
    _unknown0: VTablePad<5>,
    screen_size:
        unsafe extern "thiscall" fn(this: *const Engine, width: *mut f32, height: *mut f32),
    _unknown1: VTablePad<2>,
    player_info: unsafe extern "thiscall" fn(
        this: *const Engine,
        index: i32,
        player_info: *mut PlayerInfo,
    ) -> bool,
    player_for_user_id: unsafe extern "thiscall" fn(this: *const Engine, user_id: SteamId) -> i32,
    _unknown2: VTablePad<2>,
    local_player_index: unsafe extern "thiscall" fn(this: *const Engine) -> i32,
    _unknown3: VTablePad<5>,
    view_angle: unsafe extern "thiscall" fn(this: *const Engine, angle: *mut Vec3),
    set_view_angle: unsafe extern "thiscall" fn(this: *const Engine, angle: *const Vec3),
    max_clients: unsafe extern "thiscall" fn(this: *const Engine) -> i32,
    _unknown4: VTablePad<5>,
    is_in_game: unsafe extern "thiscall" fn(this: *const Engine) -> bool,
    is_connected: unsafe extern "thiscall" fn(this: *const Engine) -> bool,
    _unknown5: VTablePad<6>,
    set_cull_box: unsafe extern "thiscall" fn(
        this: *const Engine,
        min: *const Vec3,
        max: *const Vec3,
    ) -> bool,
    _unknown6: VTablePad<2>,
    world_to_screen_matrix: unsafe extern "thiscall" fn(this: *const Engine) -> *const Matrix3x4,
    _unknown7: VTablePad<5>,
    bsp_tree_query: unsafe extern "thiscall" fn(this: *const Engine) -> *const (),
    _unknown8: VTablePad<9>,
    level_name: unsafe extern "thiscall" fn(this: *const Engine) -> *const u8,
    _unknown9: VTablePad<24>,
    network_channel: unsafe extern "thiscall" fn(this: *const Engine) -> *const NetworkChannel,
    _unknown10: VTablePad<34>,
    execute_command: unsafe extern "thiscall" fn(
        this: *const Engine,
        command: *const u8,
        from_console_or_keybind: bool,
    ),
    _unknown11: VTablePad<72>,
    steam_api_context: unsafe extern "thiscall" fn(this: *const Engine) -> *const SteamAPIContext,
}

vtable_validate! {
    screen_size => 5,
    player_info => 8,
    player_for_user_id => 9,
    local_player_index => 12,
    view_angle => 18,
    set_view_angle => 19,
    max_clients => 20,
    is_in_game => 26,
    is_connected => 27,
    set_cull_box => 34,
    world_to_screen_matrix => 37,
    bsp_tree_query => 43,
    level_name => 53,
    network_channel => 78,
    execute_command => 113,
    steam_api_context => 186,
}

/// engine interface
#[repr(C)]
pub struct Engine {
    vtable: &'static VTable,
}

impl Engine {
    vtable_export! {
        /// returns the maximum amount of clients
        max_clients() -> i32,

        /// if in game
        is_in_game() -> bool,

        /// if connected
        is_connected() -> bool,

        /// returns the bsp tree
        bsp_tree_query() -> *const (),
    }

    /// returns the network channel
    #[inline]
    pub fn network_channel(&self) -> Option<&NetworkChannel> {
        unsafe { (self.vtable.network_channel)(self).as_ref() }
    }

    /// returns the local player's index
    #[inline]
    pub fn local_player_index(&self) -> i32 {
        unsafe { (self.vtable.local_player_index)(self) }
    }

    /// returns the screen size
    #[inline]
    pub fn screen_size(&self) -> (f32, f32) {
        unsafe {
            let mut width = MaybeUninit::uninit();
            let mut height = MaybeUninit::uninit();

            (self.vtable.screen_size)(self, width.as_mut_ptr(), height.as_mut_ptr());

            (width.assume_init(), height.assume_init())
        }
    }

    /// get player info for the player at `index`
    #[inline]
    pub fn player_info(&self, index: i32) -> Option<PlayerInfo> {
        unsafe {
            let mut player_info = MaybeUninit::uninit();
            let exists = (self.vtable.player_info)(self, index, player_info.as_mut_ptr());

            if exists {
                Some(player_info.assume_init())
            } else {
                None
            }
        }
    }

    /// get player index by `user_id`
    #[inline]
    pub fn player_for_user_id(&self, user_id: SteamId) -> Option<i32> {
        unsafe {
            let index = (self.vtable.player_for_user_id)(self, user_id);

            Some(index)
        }
    }

    /// get the view angle
    #[inline]
    pub fn view_angle(&self) -> Vec3 {
        unsafe {
            let mut view_angle = MaybeUninit::uninit();

            (self.vtable.view_angle)(self, view_angle.as_mut_ptr());

            view_angle.assume_init()
        }
    }

    /// set the view angle
    #[inline]
    pub fn set_view_angle(&self, angle: Vec3) {
        unsafe { (self.vtable.set_view_angle)(self, &angle) }
    }

    /// set the cull box
    #[inline]
    pub fn set_cull_box(&self, min: Vec3, max: Vec3) -> bool {
        unsafe { (self.vtable.set_cull_box)(self, &min, &max) }
    }

    /// returns the world to screen matrix
    #[inline]
    pub fn world_to_screen_matrix(&self) -> Matrix3x4 {
        unsafe { *(self.vtable.world_to_screen_matrix)(self) }
    }

    /// returns the current level name
    #[inline]
    pub fn level_name(&self) -> &str {
        unsafe {
            let address = (self.vtable.level_name)(self);

            ffi::str_from_ptr(address)
        }
    }

    /// executes a command
    #[inline]
    pub fn execute_command<C>(&self, command: C, from_console_or_keybind: bool)
    where
        C: AsRef<OsStr>,
    {
        unsafe {
            let maybe_cstr = ffi::osstr_to_cstr_cow(command);
            let ptr = ffi::cstr_cow_as_ptr(maybe_cstr.as_ref());

            (self.vtable.execute_command)(self, ptr, from_console_or_keybind);
        }
    }
}
