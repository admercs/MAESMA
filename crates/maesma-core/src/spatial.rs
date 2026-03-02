//! Spatial domain — grid, mesh, and network representations.

use serde::{Deserialize, Serialize};

/// Spatial domain description for a simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SpatialDomain {
    /// Regular structured grid.
    RegularGrid(RegularGrid),
    /// Unstructured mesh (e.g., MPAS-style Voronoi).
    UnstructuredMesh(UnstructuredMesh),
    /// Graph/network topology (e.g., river network).
    Network(NetworkTopology),
    /// Point-scale (single column, FLUXNET site).
    Point(PointDomain),
}

/// A regular lat/lon or projected grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegularGrid {
    /// Number of cells in x-direction.
    pub nx: usize,
    /// Number of cells in y-direction.
    pub ny: usize,
    /// Number of vertical layers.
    pub nz: usize,
    /// Cell size in x (meters).
    pub dx: f64,
    /// Cell size in y (meters).
    pub dy: f64,
    /// Origin coordinates (lon, lat) or (x, y).
    pub origin: (f64, f64),
    /// Coordinate reference system (e.g., "EPSG:4326").
    pub crs: String,
}

/// An unstructured mesh (Voronoi/Delaunay).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnstructuredMesh {
    /// Number of cells.
    pub n_cells: usize,
    /// Number of vertical layers.
    pub n_layers: usize,
    /// Typical cell area range (m²).
    pub area_range: (f64, f64),
    /// CRS.
    pub crs: String,
}

/// A network topology (e.g., river reaches).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkTopology {
    /// Number of nodes (junctions).
    pub n_nodes: usize,
    /// Number of edges (reaches/links).
    pub n_edges: usize,
    /// CRS.
    pub crs: String,
}

/// A single-point domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointDomain {
    /// Longitude.
    pub lon: f64,
    /// Latitude.
    pub lat: f64,
    /// Number of vertical layers.
    pub n_layers: usize,
    /// Site name.
    pub name: Option<String>,
}

impl SpatialDomain {
    /// Characteristic resolution in meters.
    pub fn characteristic_dx(&self) -> f64 {
        match self {
            SpatialDomain::RegularGrid(g) => g.dx.min(g.dy),
            SpatialDomain::UnstructuredMesh(m) => m.area_range.0.sqrt(),
            SpatialDomain::Network(_) => 0.0, // not grid-based
            SpatialDomain::Point(_) => 0.0,
        }
    }

    /// Total number of horizontal cells.
    pub fn n_cells(&self) -> usize {
        match self {
            SpatialDomain::RegularGrid(g) => g.nx * g.ny,
            SpatialDomain::UnstructuredMesh(m) => m.n_cells,
            SpatialDomain::Network(n) => n.n_nodes,
            SpatialDomain::Point(_) => 1,
        }
    }
}
