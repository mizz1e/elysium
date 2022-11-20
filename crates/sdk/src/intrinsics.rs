extern "C" {
    /// Returns a pointer indicating the frame address of the specified call frame, or nul if it
    /// cannot be identified. The value returned is likely incorrect or zero for any `level` that
    /// isn't zero.
    #[link_name = "llvm.frameaddress"]
    pub fn frame_addr(level: i32) -> *mut u8;

    /// Returns a pointer indicating the return address of the specified call frame, or nul if it
    /// cannot be identified. The value returned is likely incorrect or zero for any `level` that
    /// isn't zero.
    #[link_name = "llvm.returnaddress"]
    pub fn return_addr(level: i32) -> *mut u8;
}
