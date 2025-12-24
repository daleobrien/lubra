/// Define a family of related shapes with automatic variant generation.
///
/// This macro reduces code duplication by generating all 6 node variants
/// (n1-n6) from a single base shape definition with transformations.
///
/// # Example
///
/// ```ignore
/// use common::define_shape_family;
///
/// define_shape_family! {
///     base: a1_base,
///     svg: a1_str,
///     variants: {
///         n1: false => identity,
///         n2: false => rotate_ccw,
///         n3: false => rotate_cw,
///         n4: true => reflect_then_rotate_cw,
///         n5: true => reflect_then_rotate_ccw,
///         n6: true => reflect,
///     }
/// }
/// ```
///
/// This generates 6 public functions: n1(bool) -> String, n2(bool) -> String, etc.
#[macro_export]
macro_rules! define_shape_family {
    (
        base: $base_fn:ident,
        svg: $svg_fn:ident,
        variants: {
            $(
                $variant_name:ident: $invert_flip:expr => $transform:ident
            ),* $(,)?
        }
    ) => {
        $(
            pub fn $variant_name(invert: bool) -> String {
                let inv = if $invert_flip { !invert } else { invert };
                let (r, pts) = $base_fn(inv);
                let transformed_pts = $transform(pts, invert);
                $svg_fn(invert, &transformed_pts, &r)
            }
        )*
    };
}

/// Common transformation functions that can be used in shape families.
/// These are the standard transformations for 4-point paths.
pub mod transforms {
  use crate::types::path::Path;

  #[inline]
  pub fn nothing(pts: Path, _invert: bool) -> Path {
    Path::new(vec![])
  }

  /// Identity transformation (no change)
    #[inline]
    pub fn identity(pts: Path, _invert: bool) -> Path {
        pts
    }

    /// Rotate counter-clockwise (120 degrees)
    #[inline]
    pub fn rotate_ccw(pts: Path, invert: bool) -> Path {
        pts.rotate_ccw(invert, vec![0, 1, 2, 3])
    }

    /// Rotate clockwise (120 degrees)
    #[inline]
    pub fn rotate_cw(pts: Path, invert: bool) -> Path {
        pts.rotate_cw(invert, vec![0, 1, 2, 3])
    }

    /// Reflect only
    #[inline]
    pub fn reflect(pts: Path, _invert: bool) -> Path {
        pts.reflect()
    }

    /// Reflect then rotate counter-clockwise
    #[inline]
    pub fn reflect_then_rotate_ccw(pts: Path, invert: bool) -> Path {
        pts.reflect().rotate_ccw(invert, vec![0, 1, 2, 3])
    }

    /// Reflect then rotate clockwise
    #[inline]
    pub fn reflect_then_rotate_cw(pts: Path, invert: bool) -> Path {
        pts.reflect().rotate_cw(invert, vec![0, 1, 2, 3])
    }

}

// Re-export transforms for easier access
pub use transforms::*;
