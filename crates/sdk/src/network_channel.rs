use crate::Ptr;
use bevy::prelude::*;
use std::ffi;
use std::ffi::CStr;
use std::net::{IpAddrV4, SocketAddrV4};
use std::time::Duration;
use ubyte::ByteUnit;

const DEFAULT_PORT: u16 = 27015;

const FLOW_OUTGOING: ffi::c_int = 0;
const FLOW_INCOMING: ffi::c_int = 1;

// https://github.com/HackerPolice/MissedIT/blob/master/src/SDK/INetChannel.h
#[derive(Resource)]
pub struct INetChannel {
    pub(crate) ptr: Ptr,
}

#[derive(Debug)]
pub struct Info {
    pub address: SocketAddrV4,
    pub latency: (Duration, Duration),
    pub packets: (u32, u32),
    pub data: (ByteUnit, ByteUnit),
}

impl INetChannel {
    #[inline]
    fn address(&self) -> SocketAddrV4 {
        if self.is_loopback() {
            return SocketAddrV4::new(IpAddrV4::LOCALHOST, DEFAULT_PORT);
        }

        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(1) };

        let address = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!address.is_null());

        let address = unsafe { CStr::from_ptr(address).to_bytes() };

        SocketAddrV4::parse_ascii(address)
            .or_else(|_| {
                IpAddrV4::parse_ascii(address).map(|ip| SocketAddrV4::new(ip, DEFAULT_PORT))
            })
            .unwrap_or_else(|_| panic!("invalid address from INetChannel"))
    }

    #[inline]
    fn is_loopback(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(6) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    #[inline]
    fn latency(&self) -> (Duration, Duration) {
        let method: unsafe extern "C" fn(this: *mut u8, flow: ffi::c_int) -> f32 =
            unsafe { self.ptr.vtable_entry(10) };

        let outgoing = unsafe { (method)(self.ptr.as_ptr(), FLOW_OUTGOING) };
        let incoming = unsafe { (method)(self.ptr.as_ptr(), FLOW_INCOMING) };

        let outgoing = Duration::from_secs_f32(outgoing);
        let incoming = Duration::from_secs_f32(incoming);

        (outgoing, incoming)
    }

    #[inline]
    fn packets(&self) -> (u32, u32) {
        let method: unsafe extern "C" fn(this: *mut u8, flow: ffi::c_int) -> f32 =
            unsafe { self.ptr.vtable_entry(14) };

        let outgoing = unsafe { (method)(self.ptr.as_ptr(), FLOW_OUTGOING) };
        let incoming = unsafe { (method)(self.ptr.as_ptr(), FLOW_INCOMING) };

        let outgoing = outgoing.trunc() as u32;
        let incoming = incoming.trunc() as u32;

        (outgoing, incoming)
    }

    #[inline]
    fn data(&self) -> (ByteUnit, ByteUnit) {
        let method: unsafe extern "C" fn(this: *mut u8, flow: ffi::c_int) -> f32 =
            unsafe { self.ptr.vtable_entry(14) };

        let outgoing = unsafe { (method)(self.ptr.as_ptr(), FLOW_OUTGOING) };
        let incoming = unsafe { (method)(self.ptr.as_ptr(), FLOW_INCOMING) };

        let outgoing = ByteUnit::Byte(outgoing.trunc() as u64);
        let incomimg = ByteUnit::Byte(incoming.trunc() as u64);

        (outgoing, incoming)
    }

    #[inline]
    pub fn info(&self) -> Info {
        let address = self.address();
        let latency = self.latency();
        let packets = self.packets();
        let data = self.data();

        Info {
            address,
            latency,
            packets,
            data,
        }
    }
}
