#[macro_export]
macro_rules! define_shape_family {
    (
        base: $base_fn:ident,
        svg: $svg_fn:ident,
        offset_rotations: $offset_rotations:expr,
        variants: {
            $(
                $variant_name:ident: $invert_base_invert:expr, $invert_svg_invert:expr => $transform:ident
            ),* $(,)?
        }
    ) => {
        $(
            pub fn $variant_name(invert: bool) -> String {
                let inv = if $invert_base_invert { !invert } else { invert };
                let (radii, pts) = $base_fn(inv);
                let transformed_pts = $transform(pts, invert, $offset_rotations);
                let svg_inv = if $invert_svg_invert { !invert } else { invert };
                $svg_fn(svg_inv, radii, transformed_pts)
            }
        )*
    };
}

/// Common transformation functions that can be used in shape families.
/// These are the standard transformations for 4-point paths.
pub mod transforms {
  use crate::types::path::Path;

  #[inline]
    pub fn nothing(_pts: Path, _invert: bool, _offset_rotations: &[usize]) -> Path {
        Path::new(vec![])
    }

    /// Identity transformation (no change)
    #[inline]
    pub fn identity(pts: Path, _invert: bool, _offset_rotations: &[usize]) -> Path {
        pts
    }

    /// Rotate counter-clockwise (120 degrees)
    #[inline]
    pub fn rotate_ccw(pts: Path, invert: bool, offset_rotations: &[usize]) -> Path {
        pts.rotate_ccw(invert, offset_rotations.to_vec())
    }

    /// Rotate clockwise (120 degrees)
    #[inline]
    pub fn rotate_cw(pts: Path, invert: bool, offset_rotations: &[usize]) -> Path {
        pts.rotate_cw(invert, offset_rotations.to_vec())
    }

    /// Reflect only
    #[inline]
    pub fn reflect(pts: Path, _invert: bool, _offset_rotations: &[usize]) -> Path {
        pts.reflect()
    }

    /// Reflect then rotate counter-clockwise
    #[inline]
    pub fn reflect_then_rotate_ccw(pts: Path, invert: bool, offset_rotations: &[usize]) -> Path {
        pts.reflect().rotate_ccw(invert, offset_rotations.to_vec())
    }

    /// Reflect then rotate clockwise
    #[inline]
    pub fn reflect_then_rotate_cw(pts: Path, invert: bool, offset_rotations: &[usize]) -> Path {
        pts.reflect().rotate_cw(invert, offset_rotations.to_vec())
    }
}

// Re-export transforms for easier access
pub use transforms::*;
