use mint::Vector2;
use msdf_sys::*;
use std::ptr;

#[derive(Copy, Clone)]
/// Configuration for single-channel SDF calculation.
pub struct SDFConfig {
    /// Specifies whether to use the version of the algorithm that supports overlapping contours
    /// with the same winding. May be set to false to improve performance when no such contours
    /// are present.
    pub overlap_support: bool,
}

impl Default for SDFConfig {
    fn default() -> Self {
        SDFConfig {
            overlap_support: true,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Error correction mode of operation.
pub enum ErrorCorrectionMode {
    /// Skips error correction pass.
    Disabled = msdfgen_ErrorCorrectionConfig_Mode_DISABLED as isize,
    /// Corrects all discontinuities of the distance field regardless if edges are adversel
    /// affected.
    Indiscriminate = msdfgen_ErrorCorrectionConfig_Mode_INDISCRIMINATE as isize,
    /// Corrects artifacts at edges and other discontinuous distances only if it does not affect edges or corners.
    EdgePriority = msdfgen_ErrorCorrectionConfig_Mode_EDGE_PRIORITY as isize,
    /// Only corrects artifacts at edges.
    EdgeOnly = msdfgen_ErrorCorrectionConfig_Mode_EDGE_ONLY as isize,
}

impl Default for ErrorCorrectionMode {
    fn default() -> Self {
        ErrorCorrectionMode::EdgePriority
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// Configuration of whether to use an algorithm that computes the exact shape distance at the
/// positions of suspected artifacts.
pub enum DistanceCheckMode {
    /// Never computes exact shape distance.
    DoNotCheckDistance =
        msdfgen_ErrorCorrectionConfig_DistanceCheckMode_DO_NOT_CHECK_DISTANCE as isize,
    /// Only computes exact shape distance at edges. Provides a good balance between speed and
    /// precision.
    CheckDistanceAtEdge =
        msdfgen_ErrorCorrectionConfig_DistanceCheckMode_CHECK_DISTANCE_AT_EDGE as isize,
    /// Computes and compares the exact shape distance for each suspected artifact.
    AlwaysCheckDistance =
        msdfgen_ErrorCorrectionConfig_DistanceCheckMode_ALWAYS_CHECK_DISTANCE as isize,
}

impl Default for DistanceCheckMode {
    fn default() -> Self {
        DistanceCheckMode::CheckDistanceAtEdge
    }
}

#[derive(Copy, Clone)]
/// The configuration for the MSDF error correction pass.
pub struct ErrorCorrectionConfig {
    /// Error correction mode of operation.
    pub error_correction_mode: ErrorCorrectionMode,
    /// Configuration of whether to use an algorithm that computes the exact shape distance at the
    /// positions of suspected artifacts.
    pub distance_check_mode: DistanceCheckMode,
    /// The minimum ratio between the actual and maximum expected distance delta to be considered
    /// an error.
    pub min_deviation_ratio: f64,
    /// The minimum ratio between the pre-correction distance error and the post-correction
    /// distance error. Has no effect for [DistanceCheckMode::DoNotCheckDistance].
    pub min_improve_ratio: f64,
}

impl Default for ErrorCorrectionConfig {
    fn default() -> Self {
        ErrorCorrectionConfig {
            error_correction_mode: Default::default(),
            distance_check_mode: Default::default(),
            min_deviation_ratio: 1.111_111_111_111_111_2,
            min_improve_ratio: 1.111_111_111_111_111_2,
        }
    }
}

#[derive(Copy, Clone)]
/// Configuration for multi-channel SDF calculation.
pub struct MSDFConfig {
    /// Specifies whether to use the version of the algorithm that supports overlapping contours
    /// with the same winding. May be set to false to improve performance when no such contours
    /// are present.
    pub overlap_support: bool,
    /// The configuration for the MSDF error correction pass.
    pub error_correction_config: ErrorCorrectionConfig,
}

impl Default for MSDFConfig {
    fn default() -> Self {
        MSDFConfig {
            overlap_support: true,
            error_correction_config: Default::default(),
        }
    }
}

impl MSDFConfig {
    pub(super) fn as_msdfgen_config(&self) -> msdfgen_MSDFGeneratorConfig {
        msdfgen_MSDFGeneratorConfig {
            _base: msdfgen_GeneratorConfig {
                overlapSupport: self.overlap_support,
            },
            errorCorrection: msdfgen_ErrorCorrectionConfig {
                mode: self.error_correction_config.error_correction_mode
                    as msdfgen_ErrorCorrectionConfig_Mode,
                distanceCheckMode: self.error_correction_config.distance_check_mode
                    as msdfgen_ErrorCorrectionConfig_DistanceCheckMode,
                minDeviationRatio: self.error_correction_config.min_deviation_ratio,
                minImproveRatio: self.error_correction_config.min_improve_ratio,
                buffer: ptr::null_mut(),
            },
        }
    }
}

/// Specifies scale and translation for SDF generation.
pub struct Projection {
    /// Scale for SDF generation.
    pub scale: Vector2<f64>,
    /// Translation for SDF generation.
    pub translation: Vector2<f64>,
}

impl Projection {
    pub(super) fn as_msdfgen_projection(&self) -> msdfgen_Projection {
        msdfgen_Projection {
            scale: msdfgen_Vector2 {
                x: self.scale.x,
                y: self.scale.y,
            },
            translate: msdfgen_Vector2 {
                x: self.translation.x,
                y: self.translation.y,
            },
        }
    }
}

impl Default for Projection {
    fn default() -> Self {
        Projection {
            scale: Vector2 { x: 1.0, y: 1.0 },
            translation: Vector2 { x: 0.0, y: 0.0 },
        }
    }
}
