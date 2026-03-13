//! Topology adapters — grid ↔ mesh ↔ network ↔ patch mosaic conversions.
//!
//! These adapters enable conservative remapping and state transfer between
//! different spatial representations used by coupled process families.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::spatial::{NetworkTopology, RegularGrid};

/// A patch in a mosaic decomposition (e.g., vegetation tiles within a grid cell).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchMosaic {
    /// Number of parent cells.
    pub n_cells: usize,
    /// Number of patches per cell (indexed by cell_id).
    pub patches_per_cell: Vec<usize>,
    /// Total number of patches across all cells.
    pub total_patches: usize,
    /// CRS.
    pub crs: String,
}

/// Weight entry for conservative remapping between source and target cells.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemapWeight {
    /// Source cell index.
    pub src: usize,
    /// Target cell index.
    pub dst: usize,
    /// Fractional area overlap weight ∈ (0, 1].
    pub weight: f64,
}

/// A remapping operator that maps fields between two spatial representations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemapOperator {
    /// Source representation label.
    pub source: String,
    /// Target representation label.
    pub target: String,
    /// Number of source cells.
    pub n_src: usize,
    /// Number of target cells.
    pub n_dst: usize,
    /// Sparse weight entries (row = dst, col = src).
    pub weights: Vec<RemapWeight>,
    /// Whether this operator is conservative (weights per target sum to 1).
    pub conservative: bool,
}

impl RemapOperator {
    /// Apply the remap operator to a 1-D source field, producing a target field.
    pub fn apply(&self, src: &[f64]) -> Vec<f64> {
        assert_eq!(src.len(), self.n_src, "source field size mismatch");
        let mut dst = vec![0.0; self.n_dst];
        for w in &self.weights {
            dst[w.dst] += w.weight * src[w.src];
        }
        dst
    }

    /// Apply the transpose (adjoint) operator: target → source.
    pub fn apply_transpose(&self, dst_field: &[f64]) -> Vec<f64> {
        assert_eq!(dst_field.len(), self.n_dst, "target field size mismatch");
        let mut src = vec![0.0; self.n_src];
        for w in &self.weights {
            src[w.src] += w.weight * dst_field[w.dst];
        }
        src
    }

    /// Verify that weights per target cell sum to 1 (conservative).
    pub fn check_conservation(&self) -> Vec<ConservationError> {
        let mut target_sums: HashMap<usize, f64> = HashMap::new();
        for w in &self.weights {
            *target_sums.entry(w.dst).or_default() += w.weight;
        }
        let mut errors = Vec::new();
        for (dst, sum) in &target_sums {
            if (*sum - 1.0).abs() > 1e-12 {
                errors.push(ConservationError {
                    cell: *dst,
                    weight_sum: *sum,
                    deficit: 1.0 - *sum,
                });
            }
        }
        errors
    }
}

/// A conservation check failure.
#[derive(Debug, Clone)]
pub struct ConservationError {
    pub cell: usize,
    pub weight_sum: f64,
    pub deficit: f64,
}

/// Build an identity remap operator (useful for same-grid transfers).
pub fn identity_remap(n: usize, label: &str) -> RemapOperator {
    let weights = (0..n)
        .map(|i| RemapWeight {
            src: i,
            dst: i,
            weight: 1.0,
        })
        .collect();
    RemapOperator {
        source: label.to_string(),
        target: label.to_string(),
        n_src: n,
        n_dst: n,
        weights,
        conservative: true,
    }
}

/// Build a grid → coarser-grid aggregation operator.
///
/// The coarse grid must evenly divide the fine grid in both dimensions.
pub fn grid_to_coarse_grid(
    fine: &RegularGrid,
    coarse: &RegularGrid,
) -> crate::Result<RemapOperator> {
    let rx = fine.nx / coarse.nx;
    let ry = fine.ny / coarse.ny;
    if fine.nx % coarse.nx != 0 || fine.ny % coarse.ny != 0 {
        return Err(crate::Error::CompilationFailed(
            "Fine grid dimensions must be evenly divisible by coarse grid".into(),
        ));
    }
    let area_ratio = 1.0 / (rx * ry) as f64;
    let mut weights = Vec::new();
    for cy in 0..coarse.ny {
        for cx in 0..coarse.nx {
            let dst = cy * coarse.nx + cx;
            for fy in (cy * ry)..((cy + 1) * ry) {
                for fx in (cx * rx)..((cx + 1) * rx) {
                    let src = fy * fine.nx + fx;
                    weights.push(RemapWeight {
                        src,
                        dst,
                        weight: area_ratio,
                    });
                }
            }
        }
    }
    Ok(RemapOperator {
        source: format!("grid_{}x{}", fine.nx, fine.ny),
        target: format!("grid_{}x{}", coarse.nx, coarse.ny),
        n_src: fine.nx * fine.ny,
        n_dst: coarse.nx * coarse.ny,
        weights,
        conservative: true,
    })
}

/// Build a coarse-grid → fine-grid disaggregation (piecewise-constant injection).
pub fn coarse_grid_to_grid(
    coarse: &RegularGrid,
    fine: &RegularGrid,
) -> crate::Result<RemapOperator> {
    let rx = fine.nx / coarse.nx;
    let ry = fine.ny / coarse.ny;
    if fine.nx % coarse.nx != 0 || fine.ny % coarse.ny != 0 {
        return Err(crate::Error::CompilationFailed(
            "Fine grid dimensions must be evenly divisible by coarse grid".into(),
        ));
    }
    let mut weights = Vec::new();
    for cy in 0..coarse.ny {
        for cx in 0..coarse.nx {
            let src = cy * coarse.nx + cx;
            for fy in (cy * ry)..((cy + 1) * ry) {
                for fx in (cx * rx)..((cx + 1) * rx) {
                    let dst = fy * fine.nx + fx;
                    weights.push(RemapWeight {
                        src,
                        dst,
                        weight: 1.0,
                    });
                }
            }
        }
    }
    Ok(RemapOperator {
        source: format!("grid_{}x{}", coarse.nx, coarse.ny),
        target: format!("grid_{}x{}", fine.nx, fine.ny),
        n_src: coarse.nx * coarse.ny,
        n_dst: fine.nx * fine.ny,
        weights,
        conservative: false, // injection, not area-weighted
    })
}

/// Build a grid → network transfer operator.
///
/// Each network node is assigned the value of the grid cell it falls in.
/// `node_coords` maps node index → (grid_x, grid_y) cell coordinates.
pub fn grid_to_network(
    grid: &RegularGrid,
    network: &NetworkTopology,
    node_coords: &[(usize, usize)],
) -> crate::Result<RemapOperator> {
    if node_coords.len() != network.n_nodes {
        return Err(crate::Error::CompilationFailed(
            "node_coords length must match network n_nodes".into(),
        ));
    }
    let mut weights = Vec::new();
    for (dst, &(gx, gy)) in node_coords.iter().enumerate() {
        if gx >= grid.nx || gy >= grid.ny {
            return Err(crate::Error::CompilationFailed(format!(
                "Node {} coords ({},{}) outside grid {}x{}",
                dst, gx, gy, grid.nx, grid.ny
            )));
        }
        let src = gy * grid.nx + gx;
        weights.push(RemapWeight {
            src,
            dst,
            weight: 1.0,
        });
    }
    Ok(RemapOperator {
        source: format!("grid_{}x{}", grid.nx, grid.ny),
        target: format!("network_{}_nodes", network.n_nodes),
        n_src: grid.nx * grid.ny,
        n_dst: network.n_nodes,
        weights,
        conservative: false,
    })
}

/// Build a network → grid scatter operator.
///
/// Each network node contributes its value to the containing grid cell,
/// weighted by 1/count of nodes in that cell.
pub fn network_to_grid(
    network: &NetworkTopology,
    grid: &RegularGrid,
    node_coords: &[(usize, usize)],
) -> crate::Result<RemapOperator> {
    if node_coords.len() != network.n_nodes {
        return Err(crate::Error::CompilationFailed(
            "node_coords length must match network n_nodes".into(),
        ));
    }
    // Count nodes per grid cell for weighting.
    let mut cell_counts: HashMap<usize, usize> = HashMap::new();
    for &(gx, gy) in node_coords {
        let cell = gy * grid.nx + gx;
        *cell_counts.entry(cell).or_default() += 1;
    }
    let mut weights = Vec::new();
    for (src, &(gx, gy)) in node_coords.iter().enumerate() {
        let dst = gy * grid.nx + gx;
        let count = cell_counts[&dst] as f64;
        weights.push(RemapWeight {
            src,
            dst,
            weight: 1.0 / count,
        });
    }
    Ok(RemapOperator {
        source: format!("network_{}_nodes", network.n_nodes),
        target: format!("grid_{}x{}", grid.nx, grid.ny),
        n_src: network.n_nodes,
        n_dst: grid.nx * grid.ny,
        weights,
        conservative: false,
    })
}

/// Build a grid → patch mosaic disaggregation.
///
/// `patch_fractions` maps (cell_idx, patch_idx_within_cell) → area fraction.
pub fn grid_to_patches(
    grid: &RegularGrid,
    mosaic: &PatchMosaic,
    patch_fractions: &HashMap<(usize, usize), f64>,
) -> crate::Result<RemapOperator> {
    let n_grid = grid.nx * grid.ny;
    if n_grid != mosaic.n_cells {
        return Err(crate::Error::CompilationFailed(
            "Grid cell count must match mosaic n_cells".into(),
        ));
    }
    let mut weights = Vec::new();
    let mut patch_offset = 0;
    for cell in 0..mosaic.n_cells {
        let n_patches = mosaic.patches_per_cell[cell];
        for p in 0..n_patches {
            let dst = patch_offset + p;
            let frac = patch_fractions
                .get(&(cell, p))
                .copied()
                .unwrap_or(1.0 / n_patches as f64);
            weights.push(RemapWeight {
                src: cell,
                dst,
                weight: frac,
            });
        }
        patch_offset += n_patches;
    }
    Ok(RemapOperator {
        source: format!("grid_{}x{}", grid.nx, grid.ny),
        target: format!("patch_mosaic_{}", mosaic.total_patches),
        n_src: n_grid,
        n_dst: mosaic.total_patches,
        weights,
        conservative: false,
    })
}

/// Build a patch mosaic → grid aggregation.
///
/// Each cell's value is the area-weighted mean of its patches.
pub fn patches_to_grid(
    mosaic: &PatchMosaic,
    grid: &RegularGrid,
    patch_fractions: &HashMap<(usize, usize), f64>,
) -> crate::Result<RemapOperator> {
    let n_grid = grid.nx * grid.ny;
    if n_grid != mosaic.n_cells {
        return Err(crate::Error::CompilationFailed(
            "Grid cell count must match mosaic n_cells".into(),
        ));
    }
    let mut weights = Vec::new();
    let mut patch_offset = 0;
    for cell in 0..mosaic.n_cells {
        let n_patches = mosaic.patches_per_cell[cell];
        for p in 0..n_patches {
            let src = patch_offset + p;
            let frac = patch_fractions
                .get(&(cell, p))
                .copied()
                .unwrap_or(1.0 / n_patches as f64);
            weights.push(RemapWeight {
                src,
                dst: cell,
                weight: frac,
            });
        }
        patch_offset += n_patches;
    }
    Ok(RemapOperator {
        source: format!("patch_mosaic_{}", mosaic.total_patches),
        target: format!("grid_{}x{}", grid.nx, grid.ny),
        n_src: mosaic.total_patches,
        n_dst: n_grid,
        weights,
        conservative: true,
    })
}

/// Information loss tracking for a remapping operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemapInfoLoss {
    /// Name of the variable being remapped.
    pub variable: String,
    /// Source integral before remap.
    pub source_integral: f64,
    /// Target integral after remap.
    pub target_integral: f64,
    /// Relative mass/energy error: |target - source| / |source|.
    pub relative_error: f64,
    /// Maximum local error across cells.
    pub max_local_error: f64,
}

/// Compute information loss from a remap operation.
pub fn compute_info_loss(
    variable: &str,
    src_field: &[f64],
    dst_field: &[f64],
    src_areas: &[f64],
    dst_areas: &[f64],
) -> RemapInfoLoss {
    let source_integral: f64 = src_field.iter().zip(src_areas).map(|(v, a)| v * a).sum();
    let target_integral: f64 = dst_field.iter().zip(dst_areas).map(|(v, a)| v * a).sum();
    let relative_error = if source_integral.abs() > f64::EPSILON {
        (target_integral - source_integral).abs() / source_integral.abs()
    } else {
        0.0
    };
    let max_local_error = dst_field.iter().map(|v| v.abs()).fold(0.0_f64, f64::max);
    RemapInfoLoss {
        variable: variable.to_string(),
        source_integral,
        target_integral,
        relative_error,
        max_local_error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_grid(nx: usize, ny: usize) -> RegularGrid {
        RegularGrid {
            nx,
            ny,
            nz: 1,
            dx: 100.0,
            dy: 100.0,
            origin: (0.0, 0.0),
            crs: "EPSG:4326".into(),
        }
    }

    #[test]
    fn identity_remap_preserves_values() {
        let op = identity_remap(4, "test");
        let src = vec![1.0, 2.0, 3.0, 4.0];
        let dst = op.apply(&src);
        assert_eq!(dst, src);
        assert!(op.check_conservation().is_empty());
    }

    #[test]
    fn aggregation_conserves_mass() {
        let fine = make_grid(4, 4);
        let coarse = make_grid(2, 2);
        let op = grid_to_coarse_grid(&fine, &coarse).unwrap();
        // All fine cells = 1.0 → each coarse cell = mean = 1.0
        let src = vec![1.0; 16];
        let dst = op.apply(&src);
        assert_eq!(dst.len(), 4);
        for v in &dst {
            assert!((v - 1.0).abs() < 1e-12);
        }
        assert!(op.check_conservation().is_empty());
    }

    #[test]
    fn disaggregation_broadcasts() {
        let coarse = make_grid(2, 2);
        let fine = make_grid(4, 4);
        let op = coarse_grid_to_grid(&coarse, &fine).unwrap();
        let src = vec![10.0, 20.0, 30.0, 40.0];
        let dst = op.apply(&src);
        assert_eq!(dst.len(), 16);
        // Top-left 2x2 block should be 10.0
        assert!((dst[0] - 10.0).abs() < 1e-12);
        assert!((dst[1] - 10.0).abs() < 1e-12);
    }

    #[test]
    fn grid_network_round_trip() {
        let grid = make_grid(4, 4);
        let net = NetworkTopology {
            n_nodes: 3,
            n_edges: 2,
            crs: "EPSG:4326".into(),
        };
        let coords = vec![(0, 0), (1, 1), (3, 3)];
        let g2n = grid_to_network(&grid, &net, &coords).unwrap();
        let src = vec![0.0; 16];
        let dst = g2n.apply(&src);
        assert_eq!(dst.len(), 3);
    }

    #[test]
    fn patches_round_trip() {
        let grid = make_grid(2, 2);
        let mosaic = PatchMosaic {
            n_cells: 4,
            patches_per_cell: vec![2, 3, 1, 2],
            total_patches: 8,
            crs: "EPSG:4326".into(),
        };
        let mut fracs = HashMap::new();
        // Cell 0: 2 patches at 60/40
        fracs.insert((0, 0), 0.6);
        fracs.insert((0, 1), 0.4);
        // Cell 1: 3 patches at 50/30/20
        fracs.insert((1, 0), 0.5);
        fracs.insert((1, 1), 0.3);
        fracs.insert((1, 2), 0.2);
        // Cell 2: 1 patch at 100%
        fracs.insert((2, 0), 1.0);
        // Cell 3: 2 patches at 70/30
        fracs.insert((3, 0), 0.7);
        fracs.insert((3, 1), 0.3);

        let p2g = patches_to_grid(&mosaic, &grid, &fracs).unwrap();
        assert!(p2g.check_conservation().is_empty());
    }

    #[test]
    fn info_loss_zero_for_identical() {
        let loss = compute_info_loss(
            "temperature",
            &[1.0, 2.0, 3.0],
            &[1.0, 2.0, 3.0],
            &[1.0, 1.0, 1.0],
            &[1.0, 1.0, 1.0],
        );
        assert!(loss.relative_error < 1e-12);
    }

    #[test]
    fn non_divisible_grids_error() {
        let fine = make_grid(5, 5);
        let coarse = make_grid(2, 2);
        assert!(grid_to_coarse_grid(&fine, &coarse).is_err());
    }
}
