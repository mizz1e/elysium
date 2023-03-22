use super::{Map, Maps};
use std::{ops::Range, option, slice, vec};

/// An iterator of contiguous memory ranges.
pub enum Ranges {
    Option(option::IntoIter<Range<*mut u8>>),
    Vec(vec::IntoIter<Range<*mut u8>>),
}

impl Ranges {
    pub fn new(maps: &Maps) -> Self {
        match maps.maps.as_slice() {
            [] => option(None),
            [map] => option(Some(map.range())),
            [first, rest @ ..] => many(first, rest),
        }
    }
}

impl Iterator for Ranges {
    type Item = &'static mut [u8];

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let maybe_item = match self {
            Ranges::Option(iter) => iter.next(),
            Ranges::Vec(iter) => iter.next(),
        };

        maybe_item.map(|range| unsafe { slice::from_mut_ptr_range(range) })
    }
}

#[inline]
fn option(option: Option<Range<*mut u8>>) -> Ranges {
    Ranges::Option(option.into_iter())
}

#[inline]
fn many(first: &Map, rest: &[Map]) -> Ranges {
    let vec = rest.iter().fold(vec![first.range()], |mut ranges, map| {
        let range = map.range();
        // SAFETY: at least one item exists in `ranges`.
        let last_range = unsafe { ranges.last_mut().unwrap_unchecked() };

        if last_range.end == range.start {
            last_range.end = range.end;
        }

        ranges
    });

    Ranges::Vec(vec.into_iter())
}
