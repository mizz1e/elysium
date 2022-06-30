use findshlibs::{SharedLibrary, TargetSharedLibrary};
use std::collections::HashMap;
use std::ops::Range;
use std::path::Path;
use std::{ptr, slice};

/// currently loaded shared libraries
pub struct Shared {
    modules: HashMap<Box<str>, &'static [u8]>,
}

impl Shared {
    /// construct a new list of currently loaded shared libraries
    #[inline]
    pub fn new() -> Self {
        load()
    }

    #[inline]
    pub fn module_of<T>(&self, pointer: *const T) -> Option<&str> {
        self.modules
            .iter()
            .find(|(_name, bytes)| bytes_contain(bytes, pointer))
            .map(|(name, _bytes)| &**name)
    }
}

#[inline]
fn bytes_range<'a>(bytes: &'a [u8]) -> Range<usize> {
    let beg = bytes.as_ptr().addr();
    let end = beg + bytes.len();

    beg..end
}

#[inline]
fn bytes_contain<'a, T>(bytes: &'a [u8], pointer: *const T) -> bool {
    bytes_range(bytes).contains(&pointer.addr())
}

/// actually load the libraries
#[inline]
fn load() -> Shared {
    let mut modules = HashMap::new();

    TargetSharedLibrary::each(|library| {
        let name = Path::new(library.name());
        let name = match name.file_name() {
            Some(name) => name,
            None => return,
        };

        let name = Box::from(name.to_string_lossy());
        let address = ptr::from_exposed_addr(library.actual_load_addr().0);
        let len = library.len();
        let bytes = unsafe { slice::from_raw_parts(address, len) };

        modules.insert(name, bytes);
    });

    Shared { modules }
}
