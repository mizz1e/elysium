use {
    crate::{internal::assert_non_null, sys},
    bevy::prelude::*,
    std::ffi::{self, CStr},
};

pub use self::command_line::command_line;

mod command_line;

pub unsafe fn log(
    format: *const ffi::c_char,
    severity: sys::LoggingSeverity_t,
    mut args: ffi::VaListImpl<'_>,
) -> sys::LoggingResponse_t {
    assert_non_null!(format);

    let format = CStr::from_ptr(format);
    let args = if format.to_string_lossy().trim() == "%s" {
        let arg: *const ffi::c_char = args.arg();

        assert_non_null!(arg);

        let arg = CStr::from_ptr(arg);

        arg.to_string_lossy()
    } else {
        format.to_string_lossy()
    };

    let args = args.trim();

    match severity {
        sys::LoggingSeverity_t_LS_ERROR => error!("{args}"),
        sys::LoggingSeverity_t_LS_WARNING => warn!("{args}"),
        sys::LoggingSeverity_t_LS_MESSAGE => info!("{args}"),
        _ => trace!("{args}"),
    }

    sys::LoggingResponse_t_LR_CONTINUE
}
