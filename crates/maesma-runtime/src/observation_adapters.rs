//! Observation access adapters — extraction and alignment operators.
//!
//! These adapters bridge between model output (gridded `SimulationState`)
//! and observation products (point networks, swaths, polygons) so that
//! skill metrics can be computed on a like-for-like basis.
//!
//! Implements the extraction methods from `maesma_core::observations::ExtractionMethod`.

use maesma_core::observations::{BoundingBox, Cadence, ExtractionMethod};

// ---------------------------------------------------------------------------
// Point Extraction
// ---------------------------------------------------------------------------

/// Extract simulated values at specific observation station coordinates.
///
/// Uses bilinear interpolation on a regular grid for stations that fall
/// within the simulation domain.
pub struct PointExtractor {
    /// Grid origin (west, south) in degrees.
    pub origin: (f64, f64),
    /// Grid spacing (dx, dy) in degrees.
    pub spacing: (f64, f64),
    /// Grid dimensions (nx, ny).
    pub dims: (usize, usize),
}

impl PointExtractor {
    /// Create a new point extractor for a regular grid.
    pub fn new(origin: (f64, f64), spacing: (f64, f64), dims: (usize, usize)) -> Self {
        Self {
            origin,
            spacing,
            dims,
        }
    }

    /// Extract value at (lon, lat) from a flat 2D field (row-major, ny × nx).
    /// Returns `None` if the point is outside the grid.
    pub fn extract(&self, field: &[f64], lon: f64, lat: f64) -> Option<f64> {
        let (x0, y0) = self.origin;
        let (dx, dy) = self.spacing;
        let (nx, ny) = self.dims;

        // Fractional grid indices
        let fi = (lon - x0) / dx;
        let fj = (lat - y0) / dy;

        if fi < 0.0 || fj < 0.0 || fi >= (nx - 1) as f64 || fj >= (ny - 1) as f64 {
            return None;
        }

        let i0 = fi as usize;
        let j0 = fj as usize;
        let i1 = i0 + 1;
        let j1 = j0 + 1;

        let wi = fi - i0 as f64;
        let wj = fj - j0 as f64;

        // Bilinear interpolation
        let v00 = field.get(j0 * nx + i0)?;
        let v10 = field.get(j0 * nx + i1)?;
        let v01 = field.get(j1 * nx + i0)?;
        let v11 = field.get(j1 * nx + i1)?;

        let val = v00 * (1.0 - wi) * (1.0 - wj)
            + v10 * wi * (1.0 - wj)
            + v01 * (1.0 - wi) * wj
            + v11 * wi * wj;
        Some(val)
    }

    /// Extract values at multiple station locations.
    pub fn extract_multi(&self, field: &[f64], stations: &[(f64, f64)]) -> Vec<Option<f64>> {
        stations
            .iter()
            .map(|&(lon, lat)| self.extract(field, lon, lat))
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Spatial Averaging
// ---------------------------------------------------------------------------

/// Compute area-weighted spatial average over a bounding box.
pub struct SpatialAverager {
    /// Grid origin (west, south) in degrees.
    pub origin: (f64, f64),
    /// Grid spacing (dx, dy) in degrees.
    pub spacing: (f64, f64),
    /// Grid dimensions (nx, ny).
    pub dims: (usize, usize),
}

impl SpatialAverager {
    /// Create a new spatial averager for a regular grid.
    pub fn new(origin: (f64, f64), spacing: (f64, f64), dims: (usize, usize)) -> Self {
        Self {
            origin,
            spacing,
            dims,
        }
    }

    /// Compute the cosine-latitude-weighted average of a 2D field over a bounding box.
    pub fn average(&self, field: &[f64], bbox: &BoundingBox) -> Option<f64> {
        let (x0, y0) = self.origin;
        let (dx, dy) = self.spacing;
        let (nx, ny) = self.dims;

        if field.len() < nx * ny {
            return None;
        }

        let mut weighted_sum = 0.0;
        let mut weight_total = 0.0;

        for j in 0..ny {
            let lat = y0 + j as f64 * dy;
            if lat < bbox.south || lat > bbox.north {
                continue;
            }
            let cos_lat = (lat.to_radians()).cos().max(0.01);

            for i in 0..nx {
                let lon = x0 + i as f64 * dx;
                if lon < bbox.west || lon > bbox.east {
                    continue;
                }

                let val = field[j * nx + i];
                if val.is_finite() {
                    weighted_sum += val * cos_lat;
                    weight_total += cos_lat;
                }
            }
        }

        if weight_total > 0.0 {
            Some(weighted_sum / weight_total)
        } else {
            None
        }
    }
}

// ---------------------------------------------------------------------------
// Temporal Alignment
// ---------------------------------------------------------------------------

/// Align simulated time series to observation cadence by averaging.
pub struct TemporalAligner;

impl TemporalAligner {
    /// Downsample a simulation time series to a coarser cadence by averaging.
    ///
    /// `values` is the high-frequency simulation output.
    /// `steps_per_obs` is how many simulation steps per observation interval.
    pub fn downsample(values: &[f64], steps_per_obs: usize) -> Vec<f64> {
        if steps_per_obs == 0 {
            return values.to_vec();
        }
        values
            .chunks(steps_per_obs)
            .map(|chunk| {
                let (sum, count) = chunk.iter().fold((0.0, 0usize), |(s, c), &v| {
                    if v.is_finite() {
                        (s + v, c + 1)
                    } else {
                        (s, c)
                    }
                });
                if count > 0 {
                    sum / count as f64
                } else {
                    f64::NAN
                }
            })
            .collect()
    }

    /// Compute number of simulation steps per observation interval.
    pub fn steps_per_interval(sim_dt_s: f64, cadence: Cadence) -> usize {
        let obs_interval_s = match cadence {
            Cadence::SubHourly => 900.0, // 15 min
            Cadence::Hourly => 3600.0,
            Cadence::Daily => 86400.0,
            Cadence::Weekly => 604800.0,
            Cadence::Monthly => 2_592_000.0, // 30 days
            Cadence::Annual => 31_557_600.0, // 365.25 days
            Cadence::Irregular => 86400.0,   // default to daily
        };
        (obs_interval_s / sim_dt_s).round().max(1.0) as usize
    }
}

// ---------------------------------------------------------------------------
// Adapter Dispatcher
// ---------------------------------------------------------------------------

/// Dispatch observation extraction based on the configured method.
pub fn extract_for_method(
    method: ExtractionMethod,
    field: &[f64],
    grid_origin: (f64, f64),
    grid_spacing: (f64, f64),
    grid_dims: (usize, usize),
    stations: &[(f64, f64)],
    bbox: &BoundingBox,
) -> Vec<f64> {
    match method {
        ExtractionMethod::PointExtraction | ExtractionMethod::ProfileExtraction => {
            let extractor = PointExtractor::new(grid_origin, grid_spacing, grid_dims);
            extractor
                .extract_multi(field, stations)
                .into_iter()
                .flatten()
                .collect()
        }
        ExtractionMethod::SpatialAverage | ExtractionMethod::AreaWeighted => {
            let averager = SpatialAverager::new(grid_origin, grid_spacing, grid_dims);
            averager.average(field, bbox).into_iter().collect()
        }
        ExtractionMethod::TemporalAlignment => {
            // For temporal-only, return the field values directly
            field.iter().copied().filter(|v| v.is_finite()).collect()
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_extraction_center() {
        // 4×4 grid: lon [0,3], lat [0,3]
        let extractor = PointExtractor::new((0.0, 0.0), (1.0, 1.0), (4, 4));
        let field: Vec<f64> = (0..16).map(|i| i as f64).collect();

        // Exact grid point (1, 1) → index 1*4+1 = 5
        let val = extractor.extract(&field, 1.0, 1.0).unwrap();
        assert!((val - 5.0).abs() < 1e-10, "Grid point extraction: {val}");
    }

    #[test]
    fn test_point_extraction_interpolation() {
        let extractor = PointExtractor::new((0.0, 0.0), (1.0, 1.0), (4, 4));
        // Uniform field
        let field = vec![10.0; 16];
        let val = extractor.extract(&field, 0.5, 0.5).unwrap();
        assert!(
            (val - 10.0).abs() < 1e-10,
            "Uniform field interpolation: {val}"
        );
    }

    #[test]
    fn test_point_extraction_outside() {
        let extractor = PointExtractor::new((0.0, 0.0), (1.0, 1.0), (4, 4));
        let field = vec![1.0; 16];
        assert!(extractor.extract(&field, -1.0, 0.0).is_none());
        assert!(extractor.extract(&field, 5.0, 0.0).is_none());
    }

    #[test]
    fn test_spatial_average_uniform() {
        let averager = SpatialAverager::new((0.0, 0.0), (1.0, 1.0), (4, 4));
        let field = vec![5.0; 16];
        let avg = averager.average(&field, &BoundingBox::GLOBAL).unwrap();
        assert!((avg - 5.0).abs() < 1e-10, "Uniform field average: {avg}");
    }

    #[test]
    fn test_temporal_downsample() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let down = TemporalAligner::downsample(&values, 2);
        assert_eq!(down.len(), 3);
        assert!((down[0] - 1.5).abs() < 1e-10);
        assert!((down[1] - 3.5).abs() < 1e-10);
        assert!((down[2] - 5.5).abs() < 1e-10);
    }

    #[test]
    fn test_steps_per_interval() {
        assert_eq!(
            TemporalAligner::steps_per_interval(3600.0, Cadence::Daily),
            24
        );
        assert_eq!(
            TemporalAligner::steps_per_interval(86400.0, Cadence::Monthly),
            30
        );
    }
}
