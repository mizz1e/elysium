#[inline]
pub const fn invalid_character() -> ! {
    panic!("invalid character encountered in pattern");
}

#[inline]
pub const fn expected_character_in_octal() -> ! {
    panic!("expected character in octal");
}

#[inline]
pub const fn unexpected_character_in_octal() -> ! {
    panic!("unexpected character in octal");
}

#[inline]
pub const fn expected_character_in_wildcard() -> ! {
    panic!("expected character in wildcard");
}

#[inline]
pub const fn unexpected_character_in_wildcard() -> ! {
    panic!("unexpected character in wildcard");
}

#[inline]
pub const fn expected_space() -> ! {
    panic!("expected space");
}

#[inline]
pub const fn unexpected_space() -> ! {
    panic!("expected a pattern not spaces");
}

#[inline]
pub const fn unexpected_trailing_space() -> ! {
    panic!("unexpected trailing spaces");
}

#[inline]
pub const fn decrease_size() -> ! {
    panic!("decrease the pattern size");
}

#[inline]
pub const fn increase_size() -> ! {
    panic!("increase the pattern size");
}
