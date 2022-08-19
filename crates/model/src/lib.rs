use core::ops;
use elysium_math::{Matrix3x4, Vec3};

const MAX_BONES: usize = 256;
const ZERO: Bones = Bones([Matrix3x4::zero(); MAX_BONES]);

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Bones([Matrix3x4; MAX_BONES]);

impl Bones {
    #[inline]
    pub const fn zero() -> Bones {
        ZERO
    }

    #[inline]
    pub unsafe fn from_ptr<'a>(data: *const Matrix3x4) -> &'a Bones {
        &*data.cast()
    }

    #[inline]
    pub unsafe fn from_ptr_mut<'a>(data: *mut Matrix3x4) -> &'a mut Bones {
        &mut *data.cast()
    }

    #[inline]
    pub fn get_origin(&self, index: usize) -> Option<Vec3> {
        self.get(index).map(|bone| bone.w_axis())
    }

    #[inline]
    pub unsafe fn get_origin_unchecked(&self, index: usize) -> Vec3 {
        self.get_unchecked(index).w_axis()
    }
}

impl ops::Deref for Bones {
    type Target = [Matrix3x4; MAX_BONES];

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for Bones {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
