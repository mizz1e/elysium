// https://github.com/HackerPolice/MissedIT/blob/master/src/SDK/INetChannel.h

use crate::{object_validate, vtable_export, vtable_validate};
use cake::ffi::{BytePad, CUtf8Str};
use std::marker::PhantomData;
use std::mem::MaybeUninit;
//use std::net::{SocketAddr, IpAddr, Ipv4Addr};
use std::net::SocketAddr;
use std::time::Duration;

//const DEFAULT_IP: IpAddr = IpAddr::V4(Ipv4Addr::UNSPECIFIED);
//const DEFAULT_PORT: u16 = 27015;

#[repr(C)]
struct VTable {
    name2: unsafe extern "C" fn(this: *const NetworkChannel) -> *const libc::c_char,
    address: unsafe extern "C" fn(this: *const NetworkChannel) -> *const libc::c_char,
    time: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    time_connected: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    buffer_len: unsafe extern "C" fn(this: *const NetworkChannel) -> i32,
    data_rate: unsafe extern "C" fn(this: *const NetworkChannel) -> i32,
    is_loopback: unsafe extern "C" fn(this: *const NetworkChannel) -> bool,
    is_timing_out: unsafe extern "C" fn(this: *const NetworkChannel) -> bool,
    is_playback: unsafe extern "C" fn(this: *const NetworkChannel) -> bool,
    latency: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    average_latency: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    avg_loss: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    avg_choke: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    avg_data: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    avg_packets: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> f32,
    total_data: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> i32,
    sequence_number: unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow) -> i32,
    is_valid_packet:
        unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow, frame_number: i32) -> bool,
    packet_time:
        unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow, frame_number: i32) -> f32,
    packet_bytes: unsafe extern "C" fn(
        this: *const NetworkChannel,
        flow: Flow,
        frame_number: i32,
        group: i32,
    ) -> i32,
    stream_progress: unsafe extern "C" fn(
        this: *const NetworkChannel,
        flow: Flow,
        recieved: *mut i32,
        total: *mut i32,
    ) -> bool,
    time_since_last_received: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    command_interpolation_amount:
        unsafe extern "C" fn(this: *const NetworkChannel, flow: Flow, frame_number: i32) -> f32,
    packet_response_latency: unsafe extern "C" fn(
        this: *const NetworkChannel,
        flow: Flow,
        frame_number: i32,
        latency: *mut i32,
        choke: *mut i32,
    ),
    remote_frame_rate: unsafe extern "C" fn(
        this: *const NetworkChannel,
        frame_time: *mut f32,
        frame_time_standard_deviation: *mut f32,
    ),
    timeout_seconds: unsafe extern "C" fn(this: *const NetworkChannel) -> f32,
    name: unsafe extern "C" fn(this: *const NetworkChannel) -> *const libc::c_char,
}

vtable_validate! {
    address => 1,
    time => 2,
    time_connected => 3,
    buffer_len => 4,
    data_rate => 5,
    is_loopback => 6,
    is_timing_out => 7,
    is_playback => 8,
    latency => 9,
    average_latency => 10,
    avg_loss => 11,
    avg_choke => 12,
    avg_data => 13,
    avg_packets => 14,
    total_data => 15,
    sequence_number => 16,
    is_valid_packet => 17,
    packet_time => 18,
    packet_bytes => 19,
    stream_progress => 20,
    time_since_last_received => 21,
    command_interpolation_amount => 22,
    packet_response_latency => 23,
    remote_frame_rate => 24,
    timeout_seconds => 25,
    name => 26,
}

/// a network channel
#[repr(C)]
pub struct NetworkChannel {
    vtable: &'static VTable,
    _pad0: BytePad<36>,
    pub choked_packets: i32,
}

object_validate! {
    NetworkChannel;
    choked_packets => 44,
}

/// network channel flow
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Flow {
    Outgoing = 0,
    Incoming = 1,
    Both = 2,
}

impl NetworkChannel {
    vtable_export! {
        /// get current network time
        time() -> f32,

        /// get connection time in seconds
        time_connected() -> f32,

        /// get packet history size
        buffer_len() -> i32,

        /// outgoing data rate in bytes/second
        data_rate() -> i32,

        /// if loopback channel
        is_loopback() -> bool,

        /// if timing out
        is_timing_out() -> bool,

        /// if demo playback
        is_playback() -> bool,

        /// current latency (rtt), accurate but jittery
        latency(flow: Flow) -> f32,

        /// average packet loss (0 to 1)
        avg_loss(flow: Flow) -> f32,

        /// average packet choke (0 to 1)
        avg_choke(flow: Flow) -> f32,

        /// data flow in bytes/second
        avg_data(flow: Flow) -> f32,

        /// average packets/second
        avg_packets(flow: Flow) -> f32,

        /// total flow in bytes
        total_data(flow: Flow) -> i32,

        /// last sent sequence number
        sequence_number(flow: Flow) -> i32,

        /// if packet was not lost/dropped/choked/flushed
        is_valid_packet(flow: Flow, frame_number: i32) -> bool,

        /// time when packet was sent
        packet_time(flow: Flow, frame_number: i32) -> f32,

        /// group size of this packet
        packet_bytes(flow: Flow, frame_number: i32, group: i32) -> i32,

        /// get time since last recieved packet (in seconds)
        time_since_last_received() -> f32,

        /// ???
        command_interpolation_amount(flow: Flow, frame_number: i32) -> f32,

        /// ???
        packet_response_latency(
            flow: Flow,
            frame_number: i32,
            latency: &mut i32,
            choke: &mut i32
        ) -> (),

        /// ???
        timeout_seconds() -> f32,
    }

    /// Returns the average latency.
    #[inline]
    pub fn average_latency(&self, flow: Flow) -> Duration {
        let latency = unsafe { (self.vtable.average_latency)(self, flow) };

        Duration::from_secs_f32(latency)
    }

    /// Returns both incoming and outgoing average latency.
    #[inline]
    pub fn average_latency_pair(&self) -> (Duration, Duration) {
        let incoming = self.average_latency(Flow::Incoming);
        let outgoing = self.average_latency(Flow::Outgoing);

        (incoming, outgoing)
    }

    /// The IP address this channel is connected to.
    #[inline]
    pub fn address(&self) -> Option<SocketAddr> {
        unsafe {
            let pointer = (self.vtable.address)(self);

            if pointer.is_null() {
                return None;
            }

            let ip = CUtf8Str::from_ptr(pointer).as_str();

            ip.parse().ok()
        }
    }

    /// Returns the progress of a TCP transmit.
    #[inline]
    pub fn stream_progress(&self, flow: Flow) -> Option<(i32, i32)> {
        let mut received = MaybeUninit::uninit();
        let mut total = MaybeUninit::uninit();

        unsafe {
            let in_progress = (self.vtable.stream_progress)(
                self,
                flow,
                received.as_mut_ptr(),
                total.as_mut_ptr(),
            );

            in_progress.then(|| {
                let received = MaybeUninit::assume_init(received);
                let total = MaybeUninit::assume_init(total);

                (received, total)
            })
        }
    }

    /// Returns the remote frame rate.
    #[inline]
    pub fn remote_frame_rate(&self) -> (f32, f32) {
        let mut time = MaybeUninit::uninit();
        let mut time_standard_deviation = MaybeUninit::uninit();

        unsafe {
            (self.vtable.remote_frame_rate)(
                self,
                time.as_mut_ptr(),
                time_standard_deviation.as_mut_ptr(),
            );

            let time = MaybeUninit::assume_init(time);
            let time_standard_deviation = MaybeUninit::assume_init(time_standard_deviation);

            (time, time_standard_deviation)
        }
    }

    /// Get the network channel's name.
    #[inline]
    pub fn name(&self) -> Option<Box<str>> {
        unsafe {
            let pointer = (self.vtable.name)(self);

            if pointer.is_null() {
                return None;
            }

            let name = CUtf8Str::from_ptr(pointer).as_str();

            Some(Box::from(name))
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Message {
    /// valve: "true if message should be send reliable"
    /// english 100
    pub reliable: bool,
    /// if this message object owns it's own data
    pub own_data: bool,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct SendFile {
    pub message: Message,
    pub file_crc: i32,
}

impl SendFile {
    pub fn new(file_crc: i32) -> Self {
        let message = Message {
            reliable: true,
            own_data: false,
        };

        Self { message, file_crc }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Move<'a> {
    pub message: Message,
    pub backup_commands: i32,
    pub new_commands: i32,
    pub len: i32,
    pub data: *const u8,
    _phantom: PhantomData<&'a [u8]>,
}

impl<'a> Move<'a> {
    pub fn new(backup_commands: i32, new_commands: i32, data: &'a [u8]) -> Self {
        let message = Message {
            reliable: true,
            own_data: false,
        };

        Self {
            message,
            backup_commands,
            new_commands,
            len: data.len() as i32,
            data: data.as_ptr(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Command<'a> {
    pub message: Message,
    pub command: *const u8,
    _phantom: PhantomData<&'a [u8]>,
}

impl<'a> Command<'a> {
    pub fn new(command: &'a [u8]) -> Self {
        let message = Message {
            reliable: true,
            own_data: false,
        };

        Self {
            message,
            command: command.as_ptr(),
            _phantom: PhantomData,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Delta {
    pub message: Message,
    pub sequence_number: i32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VoiceData<'a> {
    pub message: Message,
    pub len: i32,
    pub data: *const u8,
    _phantom: PhantomData<&'a [u8]>,
}

impl<'a> VoiceData<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        let message = Message {
            reliable: false,
            own_data: false,
        };

        Self {
            message,
            len: data.len() as i32,
            data: data.as_ptr(),
            _phantom: PhantomData,
        }
    }
}
