use crate::Ptr;
use bevy::prelude::*;
use std::ffi::CStr;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use std::{ffi, fmt, ptr};
use ubyte::ByteUnit;

const DEFAULT_PORT: u16 = 27015;

const FLOW_OUTGOING: ffi::c_int = 0;
const FLOW_INCOMING: ffi::c_int = 1;

// public/inetchannel.h
// https://github.com/HackerPolice/MissedIT/blob/master/src/SDK/INetChannel.h
#[derive(Resource)]
pub struct INetChannel {
    pub(crate) ptr: Ptr,
}

#[derive(Debug, Resource)]
pub struct Info {
    pub address: SocketAddrV4,
    pub latency: (Duration, Duration),
    pub packet_rate: (u32, u32),
    pub data_rate: (ByteUnit, ByteUnit),
}

impl Default for Info {
    fn default() -> Self {
        Self {
            address: SocketAddrV4::new(Ipv4Addr::LOCALHOST, DEFAULT_PORT),
            latency: Default::default(),
            packet_rate: Default::default(),
            data_rate: Default::default(),
        }
    }
}

pub struct InfoDisplay<'a>(&'a Info);

impl<'a> InfoDisplay<'a> {
    fn fmt_line(
        &self,
        fmt: &mut fmt::Formatter<'_>,
        direction: char,
        latency: Duration,
        packet_rate: u32,
        data_rate: ByteUnit,
    ) -> fmt::Result {
        let (whole, frac, suffix, _unit) = data_rate.repr();
        let rate = whole as f64 + frac;

        write!(
            fmt,
            " {direction} {latency:.2?} {packet_rate}pkt/s {data_rate:.2}{suffix}/s\n"
        )
    }
}

impl<'a> fmt::Display for InfoDisplay<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Info {
            latency,
            packet_rate,
            data_rate,
            ..
        } = self.0;

        self.fmt_line(fmt, '>', latency.0, packet_rate.0, data_rate.0)?;
        self.fmt_line(fmt, '<', latency.1, packet_rate.1, data_rate.1)?;

        Ok(())
    }
}

impl Info {
    pub fn display(&self) -> InfoDisplay<'_> {
        InfoDisplay(self)
    }
}

impl INetChannel {
    fn address(&self) -> SocketAddrV4 {
        if self.is_loopback() {
            return SocketAddrV4::new(Ipv4Addr::LOCALHOST, DEFAULT_PORT);
        }

        let method: unsafe extern "C" fn(this: *mut u8) -> *const ffi::c_char =
            unsafe { self.ptr.vtable_entry(1) };

        let address = unsafe { (method)(self.ptr.as_ptr()) };

        debug_assert!(!address.is_null());

        let address = unsafe { CStr::from_ptr(address).to_bytes() };

        SocketAddrV4::parse_ascii(address)
            .or_else(|_| {
                Ipv4Addr::parse_ascii(address).map(|ip| SocketAddrV4::new(ip, DEFAULT_PORT))
            })
            .unwrap_or_else(|_| panic!("invalid address from INetChannel"))
    }

    fn is_loopback(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(6) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    fn latency(&self) -> (Duration, Duration) {
        let method: unsafe extern "C" fn(this: *mut u8, flow: ffi::c_int) -> f32 =
            unsafe { self.ptr.vtable_entry(10) };

        let incoming = unsafe { (method)(self.ptr.as_ptr(), FLOW_INCOMING) };
        let outgoing = unsafe { (method)(self.ptr.as_ptr(), FLOW_OUTGOING) };

        let incoming = Duration::from_secs_f32(incoming);
        let outgoing = Duration::from_secs_f32(outgoing);

        (incoming, outgoing)
    }

    fn packet_rate(&self) -> (u32, u32) {
        let method: unsafe extern "C" fn(this: *mut u8, flow: ffi::c_int) -> f32 =
            unsafe { self.ptr.vtable_entry(14) };

        let incoming = unsafe { (method)(self.ptr.as_ptr(), FLOW_INCOMING) };
        let outgoing = unsafe { (method)(self.ptr.as_ptr(), FLOW_OUTGOING) };

        let incoming = incoming.trunc() as u32;
        let outgoing = outgoing.trunc() as u32;

        (incoming, outgoing)
    }

    fn data_rate(&self) -> (ByteUnit, ByteUnit) {
        let method: unsafe extern "C" fn(this: *mut u8, flow: ffi::c_int) -> f32 =
            unsafe { self.ptr.vtable_entry(13) };

        let incoming = unsafe { (method)(self.ptr.as_ptr(), FLOW_INCOMING) };
        let outgoing = unsafe { (method)(self.ptr.as_ptr(), FLOW_OUTGOING) };

        let incoming = ByteUnit::Byte(incoming.trunc() as u64);
        let outgoing = ByteUnit::Byte(outgoing.trunc() as u64);

        (incoming, outgoing)
    }

    pub fn info(&self) -> Info {
        let address = self.address();
        let latency = self.latency();
        let packet_rate = self.packet_rate();
        let data_rate = self.data_rate();

        Info {
            address,
            latency,
            packet_rate,
            data_rate,
        }
    }

    pub fn send_net_message(&self, message: &INetMessage, force_reliable: bool, voice: bool) {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            message: *mut u8,
            force_reliable: bool,
            voice: bool,
        ) = unsafe { self.ptr.vtable_entry(41) };

        unsafe {
            (method)(
                self.ptr.as_ptr(),
                message.ptr.as_ptr(),
                force_reliable,
                voice,
            );
        }
    }

    pub fn set_choked(&self) {
        let method: unsafe extern "C" fn(this: *mut u8) = unsafe { self.ptr.vtable_entry(46) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn can_packet(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(57) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn is_timing_out(&self) -> bool {
        let method: unsafe extern "C" fn(this: *mut u8) -> bool =
            unsafe { self.ptr.vtable_entry(7) };

        unsafe { (method)(self.ptr.as_ptr()) }
    }

    pub fn send_datagram(&self) -> i32 {
        let method: unsafe extern "C" fn(this: *mut u8, unknown0: *const u8) -> i32 =
            unsafe { self.ptr.vtable_entry(0x178 / 8) };

        unsafe { (method)(self.ptr.as_ptr(), ptr::null()) }
    }
}

pub struct INetMessage {
    pub(crate) ptr: Ptr,
}
