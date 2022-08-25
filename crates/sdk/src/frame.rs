use core::mem;

/// Argument passed to `FrameStageNotify`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Frame {
    #[doc(alias = "FRAME_UNDEFINED")]
    #[doc(alias = "UNDEFINED")]
    Undefined = -1,

    #[doc(alias = "FRAME_START")]
    #[doc(alias = "START")]
    Start = 0,

    #[doc(alias = "FRAME_NET_UPDATE_START")]
    #[doc(alias = "NET_UPDATE_START")]
    UpdateStart = 1,

    #[doc(alias = "FRAME_NET_UPDATE_END")]
    #[doc(alias = "NET_UPDATE_END")]
    UpdateEnd = 4,

    #[doc(alias = "FRAME_NET_UPDATE_POST_DATA_UPDATE_START")]
    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_START")]
    PostDataStart = 2,

    #[doc(alias = "FRAME_NET_UPDATE_POST_DATA_UPDATE_END")]
    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_END")]
    PostDataEnd = 3,

    #[doc(alias = "FRAME_RENDER_START")]
    #[doc(alias = "RENDER_START")]
    RenderStart = 5,

    #[doc(alias = "FRAME_RENDER_END")]
    #[doc(alias = "RENDER_END")]
    RenderEnd = 6,

    #[doc(alias = "FRAME_NET_FULL_FRAME_UPDATE_ON_REMOVE")]
    #[doc(alias = "NET_FULL_FRAME_UPDATE_ON_REMOVE")]
    FullFrameUpdateOnRemove = 7,
}

impl Frame {
    /// Convert an integer representation to the enum.
    #[inline]
    pub const fn from_raw(frame: i32) -> Option<Self> {
        const START: i32 = Frame::Undefined.to_i32();
        const END: i32 = Frame::FullFrameUpdateOnRemove.to_i32();

        if matches!(frame, START..=END) {
            Some(unsafe { Self::from_raw_unchecked(frame) })
        } else {
            None
        }
    }

    /// Convert an integer representation to the enum without checking.
    ///
    /// # Safety
    ///
    /// Caller must ensure `frame` is a valid variant.
    #[inline]
    pub const unsafe fn from_raw_unchecked(frame: i32) -> Self {
        mem::transmute(frame)
    }

    /// Returns the integer representation of the enum value.
    #[inline]
    pub const fn to_i32(self) -> i32 {
        self as i32
    }

    /// Is either `UpdateStart`, `UpdateEnd`, `PostDataStart`, or `PostDataEnd`.
    #[inline]
    pub const fn is_net(&self) -> bool {
        matches!(
            &self,
            Frame::UpdateStart | Frame::UpdateEnd | Frame::PostDataStart | Frame::PostDataEnd
        )
    }

    /// Is either `RenderStart` or RenderEnd`.
    #[inline]
    pub const fn is_render(&self) -> bool {
        matches!(*self, Frame::RenderStart | Frame::RenderEnd)
    }
}
