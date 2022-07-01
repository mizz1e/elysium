/// argument passed to `FrameStageNotify`
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Frame {
    #[doc(alias = "FRAME_UNDEFINED")]
    #[doc(alias = "UNDEFINED")]
    Undefined,

    #[doc(alias = "FRAME_START")]
    #[doc(alias = "START")]
    Start,

    #[doc(alias = "FRAME_NET_UPDATE_START")]
    #[doc(alias = "NET_UPDATE_START")]
    UpdateStart,

    #[doc(alias = "FRAME_NET_UPDATE_END")]
    #[doc(alias = "NET_UPDATE_END")]
    UpdateEnd,

    #[doc(alias = "FRAME_NET_UPDATE_POST_DATA_UPDATE_START")]
    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_START")]
    PostDataStart,

    #[doc(alias = "FRAME_NET_UPDATE_POST_DATA_UPDATE_END")]
    #[doc(alias = "NET_UPDATE_POST_DATA_UPDATE_END")]
    PostDataEnd,

    #[doc(alias = "FRAME_RENDER_START")]
    #[doc(alias = "RENDER_START")]
    RenderStart,

    #[doc(alias = "FRAME_RENDER_END")]
    #[doc(alias = "RENDER_END")]
    RenderEnd,

    #[doc(alias = "FRAME_NET_FULL_FRAME_UPDATE_ON_REMOVE")]
    #[doc(alias = "NET_FULL_FRAME_UPDATE_ON_REMOVE")]
    FullFrameUpdateOnRemove,
}

impl Frame {
    /// Obtain a frame from a raw value.
    #[inline]
    pub const fn from_raw(frame: i32) -> Option<Self> {
        let frame = match frame {
            -1 => Frame::Undefined,
            0 => Frame::Start,
            1 => Frame::UpdateStart,
            2 => Frame::PostDataStart,
            3 => Frame::PostDataEnd,
            4 => Frame::UpdateEnd,
            5 => Frame::RenderStart,
            6 => Frame::RenderEnd,
            7 => Frame::FullFrameUpdateOnRemove,
            _ => return None,
        };

        Some(frame)
    }

    /// Obtain a frame from a raw value. Without checking.
    ///
    /// # Safety
    ///
    /// Caller must ensure `frame` is a valid variant.
    #[inline]
    pub const unsafe fn from_raw_unchecked(frame: i32) -> Self {
        Self::from_raw(frame).unwrap_unchecked()
    }

    /// Turn this frame into it's raw value.
    #[inline]
    pub const fn into_raw(self) -> i32 {
        match self {
            Frame::Undefined => -1,
            Frame::Start => 0,
            Frame::UpdateStart => 1,
            Frame::PostDataStart => 2,
            Frame::PostDataEnd => 3,
            Frame::UpdateEnd => 4,
            Frame::RenderStart => 5,
            Frame::RenderEnd => 6,
            Frame::FullFrameUpdateOnRemove => 7,
        }
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
