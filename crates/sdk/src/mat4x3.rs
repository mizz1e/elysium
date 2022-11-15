use bevy::math::Affine3A;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mat4x3 {
    matrix: [[f32; 4]; 3],
}

impl Mat4x3 {
    #[inline]
    pub fn from_affine(affine: Affine3A) -> Self {
        let cols_array_2d = affine.to_cols_array_2d();

        Self::from_cols_array_2d(&cols_array_2d)
    }

    #[inline]
    pub fn from_cols_array_2d(cols: &[[f32; 3]; 4]) -> Self {
        let [[x_x, x_y, x_z], [y_x, y_y, y_z], [z_x, z_y, z_z], [w_x, w_y, w_z]] = *cols;

        let matrix = [
            [x_x, y_x, z_x, w_x],
            [x_y, y_y, z_y, w_y],
            [x_z, y_z, z_z, w_z],
        ];

        Self { matrix }
    }

    #[inline]
    pub fn to_affine(self) -> Affine3A {
        let cols_array_2d = self.to_cols_array_2d();

        Affine3A::from_cols_array_2d(&cols_array_2d)
    }

    #[inline]
    pub fn to_cols_array_2d(self) -> [[f32; 3]; 4] {
        let [[x_x, y_x, z_x, w_x], [x_y, y_y, z_y, w_y], [x_z, y_z, z_z, w_z]] = self.matrix;

        let x_axis = [x_x, x_y, x_z];
        let y_axis = [y_x, y_y, y_z];
        let z_axis = [z_x, z_y, z_z];
        let w_axis = [w_x, w_y, w_z];

        [x_axis, y_axis, z_axis, w_axis]
    }
}
