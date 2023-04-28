use {crate::sys, bevy::prelude::*, std::ffi};

pub use self::command_line::command_line;

mod command_line;

pub unsafe fn log(
    format: *const ffi::c_char,
    severity: sys::LoggingSeverity_t,
    mut args: ffi::VaListImpl<'_>,
) -> sys::LoggingResponse_t {
    let mut message = String::new();

    printf_compat::format(
        format,
        args.as_va_list(),
        printf_compat::output::fmt_write(&mut message),
    );

    match severity {
        sys::LoggingSeverity_t_LS_ERROR => error!("{message}"),
        sys::LoggingSeverity_t_LS_WARNING => warn!("{message}"),
        sys::LoggingSeverity_t_LS_MESSAGE => info!("{message}"),
        _ => trace!("{message}"),
    }

    sys::LoggingResponse_t_LR_CONTINUE
}
