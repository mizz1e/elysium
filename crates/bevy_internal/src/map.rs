use {
    self::range::Ranges,
    std::{
        fmt, io,
        ops::Range,
        path::{Path, PathBuf},
        process, ptr, slice, vec,
    },
};

mod range;

bitflags::bitflags! {
    /// Permissions of a memory map.
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Permissions: u8 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXECUTE = 1 << 2;
    }
}

/// A memory map.
pub struct Map {
    pub(crate) range: Range<*mut u8>,
    path: Option<PathBuf>,
    permissions: Permissions,
}

/// Collection of memory maps.
pub struct Maps {
    pub(crate) maps: Vec<Map>,
}

impl Map {
    /// Returns the address range of this map.
    #[inline]
    pub fn range(&self) -> Range<*mut u8> {
        let Range { start, end } = &self.range;

        (*start)..(*end)
    }

    /// Returns the filesystem path of this map if it exists.
    #[inline]
    pub fn path(&self) -> Option<&Path> {
        self.path.as_deref()
    }

    /// Returns the permissions of this map.
    #[inline]
    pub fn permissions(&self) -> Permissions {
        self.permissions
    }
}

impl Maps {
    /// Memory maps of the current process.
    #[inline]
    pub fn current() -> io::Result<Self> {
        Self::of(process::id())
    }

    /// Memory maps of the given process.
    pub fn of(id: u32) -> io::Result<Self> {
        let mut maps = proc_maps::get_process_maps(id as proc_maps::Pid)?
            .into_iter()
            .flat_map(|map| {
                let read = map.is_read();

                // Skip unreadable maps ASAP.
                if !read {
                    return None;
                }

                let start = ptr::from_exposed_addr_mut(map.start());
                let bytes = unsafe { slice::from_raw_parts_mut(start, map.size()) };
                let range = bytes.as_mut_ptr_range();
                let path = map.filename().map(Into::into);
                let mut permissions = Permissions::empty();

                permissions.set(Permissions::READ, read);
                permissions.set(Permissions::WRITE, map.is_write());
                permissions.set(Permissions::EXECUTE, map.is_exec());

                Some(Map {
                    range,
                    path,
                    permissions,
                })
            })
            .collect::<Vec<_>>();

        // Ensure it is sorted by address.
        maps.sort_unstable_by_key(|map| map.range.start);

        Ok(Self { maps })
    }

    /// Returns an iterator of contiguous memory ranges.
    ///
    /// # Safety
    ///
    /// Caller must unsure the memory maps have not been invalidated.
    #[inline]
    pub unsafe fn ranges(&self) -> impl Iterator<Item = &'static mut [u8]> {
        Ranges::new(self)
    }

    /// Queries the address space of the permissions for `ptr`.
    ///
    /// Returns an empty set of permissions if this operation failed.
    #[inline]
    pub fn permissions_of(ptr: *const u8) -> Permissions {
        let ptr = ptr.cast_mut();

        Self::current()
            .ok()
            .and_then(|maps| {
                maps.maps
                    .iter()
                    .find(|map| map.range.contains(&ptr))
                    .map(|map| map.permissions())
            })
            .unwrap_or_default()
    }

    #[inline]
    pub(crate) fn assert_permissions(ptr: *const u8, permissions: Permissions) -> bool {
        Self::permissions_of(ptr).contains(permissions)
    }
}

impl fmt::Debug for Map {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = fmt.debug_struct("Map");

        debug.field("range", &self.range);

        if let Some(path) = &self.path {
            debug.field("path", &path);
        }

        debug.field("permissions", &self.permissions).finish()
    }
}

impl fmt::Debug for Maps {
    #[inline]
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.maps, fmt)
    }
}

impl IntoIterator for Maps {
    type Item = Map;
    type IntoIter = vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.maps.into_iter()
    }
}
