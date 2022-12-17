use crate::{global, net_message::SignOnState, INetChannel, Ptr};
use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct GetBaseLocalClient(pub(crate) unsafe extern "C" fn() -> *mut u8);

/// `engine/client.h`
#[repr(C)]
pub struct ClientState {
    ptr: Ptr,
}

impl ClientState {
    pub fn get() -> Option<Self> {
        let method = global::with_resource::<GetBaseLocalClient, _>(|method| method.0);

        let ptr = unsafe { (method)() };
        let ptr = Ptr::new("CClientState", ptr)?;

        Some(Self { ptr })
    }

    unsafe fn read<T>(&self, offset: usize) -> T {
        self.ptr.byte_offset::<T>(offset).read_unaligned()
    }

    unsafe fn write<T>(&self, offset: usize, value: T) {
        self.ptr.byte_offset::<T>(offset).write_unaligned(value);
    }

    pub fn last_outgoing_command(&self) -> i32 {
        unsafe { self.read(0x8e3c) }
    }

    pub fn set_last_outgoing_command(&self, value: i32) {
        unsafe { self.write(0x8e3c, value) }
    }

    pub fn choked_commands(&self) -> i32 {
        unsafe { self.read(0x8e40) }
    }

    pub fn set_choked_commands(&self, value: i32) {
        unsafe { self.write(0x8e40, value) }
    }

    pub fn sign_on_state(&self) -> SignOnState {
        unsafe { self.read(0x1a0) }
    }

    pub fn next_command_time(&self) -> Duration {
        let time = unsafe { self.read::<f32>(0x1a8) };
        let time = if !time.is_finite() {
            0.0
        } else {
            time.max(0.0)
        };

        Duration::from_secs_f32(time)
    }

    pub fn set_next_command_time(&self, value: Duration) {
        unsafe { self.write(0x1a8, value.as_secs_f32()) }
    }

    pub fn net_channel(&self) -> Option<INetChannel> {
        let ptr: *mut u8 = unsafe { self.read(0x128) };
        let ptr = Ptr::new("INetChannel", ptr)?;

        Some(INetChannel { ptr })
    }
}
