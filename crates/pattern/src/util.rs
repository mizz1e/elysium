use regex::bytes::Regex;

#[inline]
pub const unsafe fn change_lifetime<'a, 'b, T>(a: &'a T) -> &'b T
where
    T: ?Sized,
{
    &*(a as *const T)
}

#[inline]
pub fn new_regex(pattern: &'static str) -> Regex {
    unsafe { Regex::new(pattern).unwrap_unchecked() }
}
