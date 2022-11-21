//! Trace interface

use crate::{HitGroup, IClientEntity, Ptr};
use bevy::prelude::*;
use std::mem::MaybeUninit;
use std::ptr;

/// `https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/engine/IEngineTrace.h`.
#[derive(Resource)]
pub struct IEngineTrace {
    pub(crate) ptr: Ptr,
}

impl IEngineTrace {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]

    pub fn contents(&self, position: Vec3, mask: u32) -> u32 {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            position: *const Vec3,
            mask: u32,
            entity: *mut *mut u8,
        ) -> u32 = unsafe { self.ptr.vtable_entry(0) };

        unsafe { (method)(self.ptr.as_ptr(), &position, mask, ptr::null_mut()) }
    }

    pub fn trace(&self, start: Vec3, end: Vec3, mask: u32) -> TraceResult {
        self.filtered_trace(start, end, mask, None)
    }

    pub fn filtered_trace<'a>(
        &self,
        start: Vec3,
        end: Vec3,
        mask: u32,
        filter: impl IntoIterator<Item = &'a IClientEntity>,
    ) -> TraceResult {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            ray: *const internal::Ray,
            mask: u32,
            filter: *const internal::ITraceFilter,
            trace: *mut internal::trace_t,
        ) = unsafe { self.ptr.vtable_entry(5) };

        let trace = unsafe {
            let ray = internal::ray(start, end);
            let filter = internal::filter(filter);
            let mut trace = MaybeUninit::zeroed();

            (method)(self.ptr.as_ptr(), &ray, mask, &filter, trace.as_mut_ptr());

            MaybeUninit::assume_init(trace)
        };

        let internal::trace_t {
            start,
            end,
            plane,
            fraction,
            contents,
            displacement_flags,
            all_solid,
            start_solid,
            fraction_left_solid,
            surface,
            hit_group,
            physics_bone,
            world_surface_index,
            entity_hit,
            hitbox,
        } = trace;

        // plane is not valid when all_solid is true
        let plane = if all_solid {
            None
        } else {
            Some(unsafe { MaybeUninit::assume_init(plane) })
        };

        let entity_hit = Ptr::new("IClientEntity", entity_hit).map(|ptr| IClientEntity { ptr });

        TraceResult {
            start,
            end,
            fraction,
            contents,
            displacement_flags,
            plane,
            start_solid,
            fraction_left_solid,
            surface,
            hit_group,
            physics_bone,
            world_surface_index,
            entity_hit,
            hitbox,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TraceResult {
    pub start: Vec3,
    pub end: Vec3,
    pub fraction: f32,
    pub contents: u32,
    pub displacement_flags: u32,
    pub plane: Option<Plane>,
    pub start_solid: bool,
    pub fraction_left_solid: bool,
    pub surface: Surface,
    pub hit_group: HitGroup,
    pub physics_bone: i16,
    pub world_surface_index: u16,
    pub entity_hit: Option<IClientEntity>,
    pub hitbox: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Plane {
    pub normal: Vec3,
    pub dist: f32,
    pub kind: u8,
    pub sign_bits: u8,
    pub _pad0: [u8; 2],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Surface {
    pub name: *const u8,
    pub surface_props: u16,
    pub flags: u16,
}

mod internal {
    use super::{Plane, Surface};
    use crate::{HitGroup, IClientEntity, Mat4x3};
    use bevy::math::{Vec3, Vec4};
    use std::collections::HashSet;
    use std::mem::MaybeUninit;
    use std::{ffi, ptr};

    const TRACE_EVERYTHING: ffi::c_int = 0;

    const ITRACEFILTER_VTABLE: ITraceFilterVTable = ITraceFilterVTable {
        should_hit_entity,
        trace_kind,
    };

    /// public/engine/IEngineTrace.h
    #[repr(C)]
    pub struct ITraceFilter {
        vtable: &'static ITraceFilterVTable,
        skip: HashSet<*mut u8>,
    }

    #[repr(C)]
    pub struct ITraceFilterVTable {
        should_hit_entity: unsafe extern "C" fn(
            this: *const ITraceFilter,
            entity: *mut u8,
            contents_mask: ffi::c_int,
        ) -> bool,
        trace_kind: unsafe extern "C" fn(this: *const ITraceFilter) -> ffi::c_int,
    }

    #[repr(C)]
    pub struct Ray {
        // Vec4 is already aligned to 16
        pub start: Vec4,
        pub delta: Vec4,
        pub start_offset: Vec4,
        pub extents: Vec4,
        pub world_axis_transform: *const Mat4x3,
        pub is_ray: bool,
        pub is_swept: bool,
    }

    #[repr(C)]
    pub struct trace_t {
        pub start: Vec3,
        pub end: Vec3,
        pub plane: MaybeUninit<Plane>,
        pub fraction: f32,
        pub contents: u32,
        pub displacement_flags: u32,
        pub all_solid: bool,
        pub start_solid: bool,
        pub fraction_left_solid: bool,
        pub surface: Surface,
        pub hit_group: HitGroup,
        pub physics_bone: i16,
        pub world_surface_index: u16,
        pub entity_hit: *mut u8,
        pub hitbox: i32,
    }

    unsafe extern "C" fn should_hit_entity(
        this: *const ITraceFilter,
        entity: *mut u8,
        contents_mask: ffi::c_int,
    ) -> bool {
        debug_assert!(!this.is_null());

        (*this).skip.contains(&entity)
    }

    unsafe extern "C" fn trace_kind(this: *const ITraceFilter) -> ffi::c_int {
        debug_assert!(!this.is_null());

        TRACE_EVERYTHING
    }

    pub fn filter<'a>(skip: impl IntoIterator<Item = &'a IClientEntity>) -> ITraceFilter {
        ITraceFilter {
            vtable: &ITRACEFILTER_VTABLE,
            skip: skip.into_iter().map(|entity| entity.ptr.as_ptr()).collect(),
        }
    }

    pub fn ray(start: Vec3, end: Vec3) -> Ray {
        let delta = end - start;
        let is_swept = delta.length() != 0.0;
        let delta = delta.extend(0.0);
        let extents = Vec4::ZERO;
        let world_axis_transform = ptr::null();
        let is_ray = true;
        let start_offset = Vec4::ZERO;
        let start = start.extend(0.0);

        Ray {
            start,
            delta,
            start_offset,
            extents,
            world_axis_transform,
            is_ray,
            is_swept,
        }
    }
}
