//! Trace interface

use crate::{HitGroup, Ptr};
use bevy::math::Vec3;
use std::mem::MaybeUninit;

/// Kind of trace to perform.
#[repr(i32)]
pub enum TraceKind {
    Everything = 0,
    /// this does not test static props
    WorldOnly = 1,
    /// this version will not test static props
    EntitiesOnly = 2,
    /// everything filter props
    EverythingFilterProps = 3,
}

/// A trait used to customize what a trace will yield.
pub trait ITraceFilter {
    fn should_hit(&self, entity: *const u8, mask: i32) -> bool;
    fn trace_kind(&self) -> TraceKind;
}

/// https://github.com/ValveSoftware/source-sdk-2013/blob/master/mp/src/public/engine/IEngineTrace.h
pub struct IEngineTrace {
    pub(crate) ptr: Ptr,
}

impl IEngineTrace {
    #[inline]
    pub fn contents(&self, position: Vec3, mask: u32, entity: *mut *mut u8) -> u32 {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            position: *const Vec3,
            mask: u32,
            entity: *mut *mut u8,
        ) -> u32 = unsafe { self.ptr.vtable_entry(0) };

        unsafe { (method)(self.ptr.as_ptr(), &position, mask, entity) }
    }

    #[inline]
    pub fn trace(&self, start: Vec3, end: Vec3, mask: u32) -> TraceResult {
        let method: unsafe extern "C" fn(
            this: *mut u8,
            ray: *const internal::Ray,
            mask: u32,
            filter: internal::ITraceFilter,
            trace: *mut internal::trace_t,
        ) = unsafe { self.ptr.vtable_entry(5) };

        let trace = unsafe {
            let ray = internal::Ray::new(start, end);
            let mut trace = MaybeUninit::uninit();

            (method)(self.ptr.as_ptr(), &ray, mask, filter, trace.as_mut_ptr());

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

#[derive(Debug)]
pub struct TraceResult {
    pub start: Vec3,
    pub end: Vec3,
    pub fraction: f32,
    pub contents: i32,
    pub displacement_flags: u32,
    pub plane: Option<Plane>,
    pub start_solid: bool,
    pub fraction_left_solid: bool,
    pub surface: Surface,
    pub hit_group: HitGroup,
    pub physics_bone: i16,
    pub world_surface_index: u16,
    pub entity_hit: *mut u8,
    pub hitbox: i32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Plane {
    pub normal: Vec3,
    pub dist: f32,
    pub kind: u8,
    pub sign_bits: u8,
    pub _pad0: [u8; 2],
}

#[derive(Debug)]
#[repr(C)]
pub struct Surface {
    pub name: *const u8,
    pub surface_props: i16,
    pub flags: u16,
}

mod internal {
    use super::{Plane, Surface};
    use crate::{HitGroup, Mat4x3};
    use bevy::math::{Vec3, Vec4};
    use std::mem::MaybeUninit;
    use std::ptr;

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

    impl Ray {
        #[inline]
        pub fn new(start: Vec3, end: Vec3) -> Self {
            let delta = end - start;
            let is_swept = delta.length() != 0.0;
            let delta = delta.extend(0.0);
            let extents = Vec4::ZERO;
            let world_axis_transform = ptr::null();
            let is_ray = true;
            let start_offset = Vec4::ZERO;
            let start = start.extend(0.0);

            Self {
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

    #[repr(C)]
    pub struct trace_t {
        pub start: Vec3,
        pub end: Vec3,
        pub plane: MaybeUninit<Plane>,
        pub fraction: f32,
        pub contents: i32,
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
}
