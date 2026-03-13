//! Observation Registry seed data — initial dataset catalog.
//!
//! Populates the observation registry with 30+ real-world datasets spanning
//! all 13 process families, as specified in ROADMAP Phase 0.6.

use maesma_core::families::ProcessFamily;
use maesma_core::observations::*;

/// Generate the full seed catalog of observation datasets.
pub fn generate_seed_observations() -> Vec<ObservationDataset> {
    let mut ds = vec![
        // ---- Hydrology ----
        usgs_nwis(),
        smap_l3(),
        grace_mascons(),
        // ---- Radiation & Energy ----
        fluxnet(),
        ceres_ebaf(),
        arm_sites(),
        // ---- Fire ----
        mtbs(),
        viirs_active_fire(),
        ravg(),
        // ---- Ecology / Vegetation ----
        modis_lai(),
        modis_et(),
    ];
    // Remaining datasets pushed individually below
    ds.push(modis_ndvi());
    ds.push(gedi_canopy());
    ds.push(fia_plots());
    ds.push(neon_surveys());

    // ---- Ocean ----
    ds.push(argo_floats());
    ds.push(satellite_altimetry());

    // ---- Cryosphere ----
    ds.push(nsidc_sea_ice());
    ds.push(grace_ice_sheets());

    // ---- Biogeochemistry ----
    ds.push(fluxnet_co2());

    // ---- Atmosphere ----
    ds.push(era5());

    // ---- Human Systems ----
    ds.push(eia_energy());

    // ---- Trophic Dynamics / Biodiversity ----
    ds.push(gbif());
    ds.push(ebird());
    ds.push(fao_fisheries());
    ds.push(try_traits());
    ds.push(bien_traits());

    // ---- Evolution / Paleo ----
    ds.push(pbdb());
    ds.push(timetree());
    ds.push(quaternary_pollen());

    // ---- Planetary Defense ----
    ds.push(cneos_sentry());
    ds.push(mpc_orbits());

    // ---- Geomorphology ----
    ds.push(copernicus_dem());

    // ---- Remote Sensing — Optical / Multispectral ----
    ds.push(landsat_c2());
    ds.push(sentinel2_msi());
    ds.push(sentinel1_sar());
    ds.push(avhrr_gimms_ndvi());

    // ---- Remote Sensing — MODIS Extended Products ----
    ds.push(modis_lst());
    ds.push(modis_burned_area());
    ds.push(modis_albedo());
    ds.push(modis_gpp());
    ds.push(modis_snow_cover());

    // ---- Remote Sensing — Precipitation ----
    ds.push(gpm_imerg());
    ds.push(chirps());

    // ---- Remote Sensing — Atmospheric Composition ----
    ds.push(oco2());
    ds.push(tropomi_s5p());
    ds.push(gosat());
    ds.push(calipso_caliop());
    ds.push(cloudsat());
    ds.push(goes_r_abi());
    ds.push(merra2());

    // ---- Remote Sensing — Ocean ----
    ds.push(ghrsst());
    ds.push(modis_ocean_color());
    ds.push(swot());
    ds.push(sentinel6());
    ds.push(sentinel3_olci());

    // ---- Remote Sensing — Cryosphere / Altimetry ----
    ds.push(icesat2());
    ds.push(cryosat2());
    ds.push(amsr2());

    // ---- Remote Sensing — Land Cover / Surface Water ----
    ds.push(esa_cci_land_cover());
    ds.push(jrc_global_surface_water());

    // ---- Remote Sensing — Elevation ----
    ds.push(srtm());

    // ---- Remote Sensing — Fire Emissions ----
    ds.push(gfed());

    // ---- Remote Sensing — Soil Moisture (extended) ----
    ds.push(esa_cci_soil_moisture());

    // ---- Remote Sensing — Gravity (GRACE-FO) ----
    ds.push(grace_fo_mascons());

    // ---- Remote Sensing — Vegetation / Biomass ----
    ds.push(smos_vod());

    // ---- Remote Sensing — Radiation (extended) ----
    ds.push(ceres_syn1deg());

    // ---- Remote Sensing — Night Lights ----
    ds.push(viirs_dnb());

    // ---- Remote Sensing — Evapotranspiration / Water Flux ----
    ds.push(ecostress());
    ds.push(gleam());

    // ---- Remote Sensing — Harmonised / Fused Products ----
    ds.push(hls());
    ds.push(dynamic_world());

    // ---- Remote Sensing — New-Generation Missions ----
    ds.push(pace_oci());
    ds.push(emit());
    ds.push(nisar());

    // ---- Remote Sensing — Forest / Biomass Change ----
    ds.push(hansen_gfc());
    ds.push(glad_alerts());
    ds.push(esa_cci_biomass());
    ds.push(global_mangrove_watch());

    // ---- Remote Sensing — SAR (additional) ----
    ds.push(alos_palsar());

    // ---- Remote Sensing — Precipitation (additional) ----
    ds.push(trmm());
    ds.push(persiann_cdr());

    // ---- Remote Sensing — Fire (additional) ----
    ds.push(firms());

    // ---- Reanalysis & Climate Grids (additional) ----
    ds.push(era5_land());
    ds.push(cru_ts());
    ds.push(terraclimate());
    ds.push(worldclim());
    ds.push(cams_global());

    // ---- Remote Sensing — Ocean (additional) ----
    ds.push(esa_cci_sst());
    ds.push(copernicus_marine());

    // ---- Gridded Soil / Subsurface ----
    ds.push(soilgrids());

    // ---- Population / Socioeconomic ----
    ds.push(worldpop());

    // ---- Remote Sensing — Flood / Disaster ----
    ds.push(global_flood_database());

    // ---- Remote Sensing — Gravity / Geoid ----
    ds.push(eigen6c4());

    // ---- Remote Sensing — Permafrost ----
    ds.push(esa_cci_permafrost());

    // ---- Remote Sensing — Ocean Biogeochemistry ----
    ds.push(esa_cci_ocean_colour());

    // ---- Carbon Flux Inversion ----
    ds.push(carbontracker());

    // ---- Groundwater ----
    ds.push(global_groundwater());

    ds
}

// ---------------------------------------------------------------------------
// Hydrology
// ---------------------------------------------------------------------------

fn usgs_nwis() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("USGS_NWIS"),
        name: "USGS_NWIS".into(),
        description: "USGS National Water Information System — streamflow gauges".into(),
        provider: "USGS".into(),
        observable: Observable {
            name: "streamflow".into(),
            unit: "m3 s-1".into(),
            families: vec![ProcessFamily::Hydrology],
            description: "Instantaneous and daily mean discharge at gauging stations".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::CONUS,
            temporal: TemporalExtent {
                start_year: 1900,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(13_000),
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec!["rating-curve extrapolation at flood stage".into()],
            quality_flag: Some("qualifiers".into()),
        },
        access: vec![AccessMethod::Api {
            base_url: "https://waterservices.usgs.gov/nwis/iv/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "kge".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec!["discharge".into(), "streamflow".into(), "conus".into()],
    }
}

fn smap_l3() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SMAP_L3_SM"),
        name: "SMAP_L3_SM".into(),
        description: "NASA SMAP Level-3 soil moisture (top 5 cm)".into(),
        provider: "NASA JPL".into(),
        observable: Observable {
            name: "soil_moisture".into(),
            unit: "m3 m-3".into(),
            families: vec![ProcessFamily::Hydrology],
            description: "Volumetric soil moisture from L-band passive microwave".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2015,
                end_year: None,
            },
            spatial_resolution_m: Some(36_000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.04),
            relative: None,
            known_biases: vec![
                "vegetation opacity in dense canopy".into(),
                "surface roughness".into(),
            ],
            quality_flag: Some("retrieval_qual_flag".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "SPL3SMP".into(),
            version: "008".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 0.5,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec!["soil_moisture".into(), "satellite".into(), "global".into()],
    }
}

fn grace_mascons() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GRACE_TWS"),
        name: "GRACE_TWS".into(),
        description: "GRACE/GRACE-FO terrestrial water storage anomalies (mascons)".into(),
        provider: "NASA JPL / CSR".into(),
        observable: Observable {
            name: "terrestrial_water_storage".into(),
            unit: "cm".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Cryosphere],
            description: "Monthly TWS anomalies from gravity field measurements".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2002,
                end_year: None,
            },
            spatial_resolution_m: Some(300_000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(2.0),
            relative: None,
            known_biases: vec![
                "GIA correction uncertainty".into(),
                "gap 2017-06 to 2018-05".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern:
                "https://podaac.jpl.nasa.gov/dataset/TELLUS_GRAC-GRFO_MASCON_CRI_GRID_RL06.1_V3"
                    .into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Cryosphere],
        tags: vec!["water_storage".into(), "gravity".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Radiation & Energy
// ---------------------------------------------------------------------------

fn fluxnet() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("FLUXNET2015"),
        name: "FLUXNET2015".into(),
        description: "FLUXNET eddy-covariance tower network — energy & water fluxes".into(),
        provider: "FLUXNET / AmeriFlux".into(),
        observable: Observable {
            name: "latent_heat_flux".into(),
            unit: "W m-2".into(),
            families: vec![ProcessFamily::Radiation, ProcessFamily::Hydrology],
            description: "Latent heat flux (evapotranspiration) from eddy covariance".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1996,
                end_year: Some(2020),
            },
            spatial_resolution_m: None,
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(212),
        },
        uncertainty: UncertaintySpec {
            absolute: Some(20.0),
            relative: Some(0.10),
            known_biases: vec![
                "energy balance non-closure".into(),
                "nighttime underestimate".into(),
            ],
            quality_flag: Some("NEE_VUT_REF_QC".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://fluxnet.org/data/fluxnet2015-dataset/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "kge".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 3.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![
            ProcessFamily::Radiation,
            ProcessFamily::Hydrology,
            ProcessFamily::Ecology,
        ],
        tags: vec![
            "flux_tower".into(),
            "eddy_covariance".into(),
            "energy_balance".into(),
        ],
    }
}

fn ceres_ebaf() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CERES_EBAF"),
        name: "CERES_EBAF".into(),
        description: "CERES Energy Balanced and Filled TOA and surface radiative fluxes".into(),
        provider: "NASA LaRC".into(),
        observable: Observable {
            name: "toa_net_radiation".into(),
            unit: "W m-2".into(),
            families: vec![ProcessFamily::Radiation, ProcessFamily::Atmosphere],
            description: "Top-of-atmosphere net radiation from CERES instruments".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(111_000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(2.5),
            relative: None,
            known_biases: vec!["diurnal sampling in early record".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "CERES_EBAF-TOA_Ed4.2".into(),
            version: "4.2".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "bias".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Radiation, ProcessFamily::Atmosphere],
        tags: vec!["radiation".into(), "toa".into(), "global".into()],
    }
}

fn arm_sites() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ARM_SITES"),
        name: "ARM_SITES".into(),
        description: "DOE Atmospheric Radiation Measurement — ground-based radiation & cloud"
            .into(),
        provider: "DOE ARM".into(),
        observable: Observable {
            name: "downwelling_shortwave".into(),
            unit: "W m-2".into(),
            families: vec![ProcessFamily::Radiation, ProcessFamily::Atmosphere],
            description: "Surface downwelling shortwave radiation from ARM facilities".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1993,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(6),
        },
        uncertainty: UncertaintySpec {
            absolute: Some(5.0),
            relative: Some(0.02),
            known_biases: vec![],
            quality_flag: Some("qc_flags".into()),
        },
        access: vec![AccessMethod::Api {
            base_url: "https://adc.arm.gov/discovery/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.5,
        },
        evaluates_families: vec![ProcessFamily::Radiation, ProcessFamily::Atmosphere],
        tags: vec!["radiation".into(), "cloud".into(), "surface".into()],
    }
}

// ---------------------------------------------------------------------------
// Fire
// ---------------------------------------------------------------------------

fn mtbs() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MTBS"),
        name: "MTBS".into(),
        description: "Monitoring Trends in Burn Severity — fire perimeters & severity".into(),
        provider: "USGS / USFS".into(),
        observable: Observable {
            name: "burn_severity".into(),
            unit: "dNBR".into(),
            families: vec![ProcessFamily::Fire],
            description: "Burn severity (dNBR) and fire perimeters for fires >1000 acres".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::CONUS,
            temporal: TemporalExtent {
                start_year: 1984,
                end_year: None,
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Polygon,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "omits fires <1000 acres".into(),
                "mapping date vs. fire date lag".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.mtbs.gov/direct-download".into(),
        }],
        formats: vec![DataFormat::GeoTiff, DataFormat::Shapefile],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "kappa".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Fire],
        tags: vec![
            "fire".into(),
            "burn_severity".into(),
            "perimeter".into(),
            "conus".into(),
        ],
    }
}

fn viirs_active_fire() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("VIIRS_AF"),
        name: "VIIRS_AF".into(),
        description: "VIIRS Active Fire — near-real-time thermal detections (375 m)".into(),
        provider: "NASA FIRMS".into(),
        observable: Observable {
            name: "fire_radiative_power".into(),
            unit: "MW".into(),
            families: vec![ProcessFamily::Fire],
            description: "Active fire detections with fire radiative power from VIIRS I-band"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2012,
                end_year: None,
            },
            spatial_resolution_m: Some(375.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.20),
            known_biases: vec!["cloud obscuration".into(), "sub-pixel fire mixing".into()],
            quality_flag: Some("confidence".into()),
        },
        access: vec![AccessMethod::Api {
            base_url: "https://firms.modaps.eosdis.nasa.gov/api/".into(),
        }],
        formats: vec![DataFormat::Csv, DataFormat::Shapefile],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "f1_score".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Fire],
        tags: vec![
            "fire".into(),
            "active_fire".into(),
            "nrt".into(),
            "global".into(),
        ],
    }
}

fn ravg() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("RAVG"),
        name: "RAVG".into(),
        description: "Rapid Assessment of Vegetation Condition after Wildfire — BARC severity"
            .into(),
        provider: "USDA Forest Service".into(),
        observable: Observable {
            name: "burn_severity".into(),
            unit: "class".into(),
            families: vec![ProcessFamily::Fire, ProcessFamily::Ecology],
            description: "Categorical soil-burn and canopy-burn severity (BARC4)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::CONUS,
            temporal: TemporalExtent {
                start_year: 2006,
                end_year: None,
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Polygon,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.20),
            known_biases: vec!["timing-dependent reflectance change".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://burnseverity.cr.usgs.gov/ravg/data-access".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::UsGov,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "kappa".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Fire, ProcessFamily::Ecology],
        tags: vec![
            "fire".into(),
            "burn_severity".into(),
            "rapid_assessment".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Ecology / Vegetation
// ---------------------------------------------------------------------------

fn modis_lai() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_LAI"),
        name: "MODIS_LAI".into(),
        description: "MODIS MCD15A2H — 8-day leaf area index (500 m)".into(),
        provider: "NASA LP DAAC".into(),
        observable: Observable {
            name: "lai".into(),
            unit: "m2 m-2".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Radiation],
            description: "Leaf area index from MODIS Terra+Aqua combined".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2002,
                end_year: None,
            },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.5),
            relative: Some(0.15),
            known_biases: vec!["saturation in dense canopy (LAI > 6)".into()],
            quality_flag: Some("FparLai_QC".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MCD15A2H".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Radiation],
        tags: vec![
            "lai".into(),
            "vegetation".into(),
            "satellite".into(),
            "global".into(),
        ],
    }
}

fn modis_et() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_ET"),
        name: "MODIS_ET".into(),
        description: "MODIS MOD16A2 — 8-day evapotranspiration (500 m)".into(),
        provider: "NASA LP DAAC".into(),
        observable: Observable {
            name: "evapotranspiration".into(),
            unit: "kg m-2 d-1".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Ecology],
            description: "ET from Penman-Monteith driven by MODIS land products".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2001,
                end_year: None,
            },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: Some(0.25),
            known_biases: vec!["overestimation in arid regions".into()],
            quality_flag: Some("ET_QC".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MOD16A2".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "kge".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Ecology],
        tags: vec!["et".into(), "evapotranspiration".into(), "satellite".into()],
    }
}

fn modis_ndvi() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_NDVI"),
        name: "MODIS_NDVI".into(),
        description: "MODIS MOD13A2 — 16-day NDVI (1 km)".into(),
        provider: "NASA LP DAAC".into(),
        observable: Observable {
            name: "ndvi".into(),
            unit: "1".into(),
            families: vec![ProcessFamily::Ecology],
            description: "Normalized Difference Vegetation Index from MODIS Terra".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(1000.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.02),
            relative: None,
            known_biases: vec!["atmospheric contamination residual".into()],
            quality_flag: Some("pixel_reliability".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MOD13A2".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology],
        tags: vec![
            "ndvi".into(),
            "vegetation".into(),
            "satellite".into(),
            "global".into(),
        ],
    }
}

fn gedi_canopy() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GEDI_L2A"),
        name: "GEDI_L2A".into(),
        description: "GEDI L2A — spaceborne lidar canopy height (25 m footprint)".into(),
        provider: "NASA GSFC".into(),
        observable: Observable {
            name: "canopy_height".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Fire],
            description: "Relative height metrics from GEDI full-waveform lidar".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0,
                south: -51.5,
                east: 180.0,
                north: 51.5,
            },
            temporal: TemporalExtent {
                start_year: 2019,
                end_year: None,
            },
            spatial_resolution_m: Some(25.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(3.0),
            relative: Some(0.10),
            known_biases: vec!["slope-induced geolocation error".into()],
            quality_flag: Some("quality_flag".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "GEDI02_A".into(),
            version: "002".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Fire],
        tags: vec!["canopy_height".into(), "lidar".into(), "structure".into()],
    }
}

fn fia_plots() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("FIA_PLOTS"),
        name: "FIA_PLOTS".into(),
        description: "USDA Forest Service Forest Inventory and Analysis — plot data".into(),
        provider: "USDA Forest Service".into(),
        observable: Observable {
            name: "aboveground_biomass".into(),
            unit: "kg m-2".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
            description:
                "Tree-level measurements aggregated to plot biomass, volume, growth, mortality"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::CONUS,
            temporal: TemporalExtent {
                start_year: 1999,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Annual,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(125_000),
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec!["allometric model uncertainty".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://apps.fs.usda.gov/fia/datamart/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::UsGov,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
        tags: vec![
            "forest".into(),
            "biomass".into(),
            "inventory".into(),
            "conus".into(),
        ],
    }
}

fn neon_surveys() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("NEON_SURVEYS"),
        name: "NEON_SURVEYS".into(),
        description: "NEON terrestrial observation network — multi-trophic ecological surveys".into(),
        provider: "NSF NEON / Battelle".into(),
        observable: Observable {
            name: "species_abundance".into(),
            unit: "count".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::TrophicDynamics],
            description: "Standardised biodiversity surveys: plants, birds, mammals, beetles, ticks, soil microbes".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::CONUS,
            temporal: TemporalExtent { start_year: 2014, end_year: None },
            spatial_resolution_m: None,
            cadence: Cadence::Annual,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(81),
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec!["detection probability varies by taxon".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api { base_url: "https://data.neonscience.org/data-api/".into() }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 3.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::TrophicDynamics],
        tags: vec!["biodiversity".into(), "neon".into(), "multi_trophic".into()],
    }
}

// ---------------------------------------------------------------------------
// Ocean
// ---------------------------------------------------------------------------

fn argo_floats() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ARGO_FLOATS"),
        name: "ARGO_FLOATS".into(),
        description: "Argo profiling float network — temperature & salinity profiles (0–2000 m)"
            .into(),
        provider: "Argo / IFREMER / NOAA".into(),
        observable: Observable {
            name: "ocean_temperature".into(),
            unit: "K".into(),
            families: vec![ProcessFamily::Ocean],
            description: "In-situ temperature profiles from autonomous profiling floats".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Profile,
            station_count: Some(3900),
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.002),
            relative: None,
            known_biases: vec!["pressure sensor drift in early floats".into()],
            quality_flag: Some("QC_flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://data-argo.ifremer.fr/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::ProfileExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean],
        tags: vec![
            "ocean".into(),
            "temperature".into(),
            "salinity".into(),
            "profile".into(),
        ],
    }
}

fn satellite_altimetry() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SAT_ALTIMETRY"),
        name: "SAT_ALTIMETRY".into(),
        description: "Multi-mission satellite altimetry — sea surface height anomalies".into(),
        provider: "AVISO / Copernicus Marine".into(),
        observable: Observable {
            name: "sea_surface_height".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Ocean],
            description: "Gridded sea surface height anomalies from merged altimeter missions"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1993,
                end_year: None,
            },
            spatial_resolution_m: Some(25_000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.03),
            relative: None,
            known_biases: vec!["near-coast contamination".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://data.marine.copernicus.eu/product/SEALEVEL_GLO_PHY_L4_MY_008_047"
                .into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean],
        tags: vec![
            "ocean".into(),
            "sea_level".into(),
            "altimetry".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Cryosphere
// ---------------------------------------------------------------------------

fn nsidc_sea_ice() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("NSIDC_SEA_ICE"),
        name: "NSIDC_SEA_ICE".into(),
        description: "NSIDC Sea Ice Index — concentration & extent from passive microwave".into(),
        provider: "NSIDC".into(),
        observable: Observable {
            name: "sea_ice_concentration".into(),
            unit: "fraction".into(),
            families: vec![ProcessFamily::Cryosphere],
            description: "Sea ice concentration from SMMR, SSM/I, SSMIS passive microwave".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1979,
                end_year: None,
            },
            spatial_resolution_m: Some(25_000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.05),
            relative: None,
            known_biases: vec!["melt-pond contamination in summer Arctic".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://nsidc.org/data/nsidc-0051".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Cryosphere],
        tags: vec![
            "sea_ice".into(),
            "cryosphere".into(),
            "arctic".into(),
            "antarctic".into(),
        ],
    }
}

fn grace_ice_sheets() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GRACE_ICE"),
        name: "GRACE_ICE".into(),
        description: "GRACE/GRACE-FO ice-sheet mass balance — Greenland & Antarctica".into(),
        provider: "NASA JPL".into(),
        observable: Observable {
            name: "ice_mass_change".into(),
            unit: "Gt yr-1".into(),
            families: vec![ProcessFamily::Cryosphere],
            description: "Monthly ice-sheet mass anomalies from GRACE gravity".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2002,
                end_year: None,
            },
            spatial_resolution_m: Some(300_000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(20.0),
            relative: None,
            known_biases: vec!["GIA model dependency".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern:
                "https://podaac.jpl.nasa.gov/dataset/TELLUS_GRAC-GRFO_MASCON_CRI_GRID_RL06.1_V3"
                    .into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "bias".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Cryosphere],
        tags: vec!["ice_sheet".into(), "mass_balance".into(), "gravity".into()],
    }
}

// ---------------------------------------------------------------------------
// Biogeochemistry
// ---------------------------------------------------------------------------

fn fluxnet_co2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("FLUXNET_CO2"),
        name: "FLUXNET_CO2".into(),
        description: "FLUXNET — net ecosystem exchange of CO₂ (eddy covariance)".into(),
        provider: "FLUXNET / AmeriFlux".into(),
        observable: Observable {
            name: "nee".into(),
            unit: "g C m-2 d-1".into(),
            families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Ecology],
            description: "Net ecosystem exchange from eddy-covariance towers".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1996,
                end_year: Some(2020),
            },
            spatial_resolution_m: None,
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(212),
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: Some(0.15),
            known_biases: vec![
                "u* filtering uncertainty".into(),
                "gap-filling method".into(),
            ],
            quality_flag: Some("NEE_VUT_REF_QC".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://fluxnet.org/data/fluxnet2015-dataset/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "kge".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 3.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Ecology],
        tags: vec!["carbon".into(), "nee".into(), "flux_tower".into()],
    }
}

// ---------------------------------------------------------------------------
// Atmosphere
// ---------------------------------------------------------------------------

fn era5() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ERA5"),
        name: "ERA5".into(),
        description: "ECMWF ERA5 reanalysis — global atmospheric/land forcing".into(),
        provider: "ECMWF / Copernicus CDS".into(),
        observable: Observable {
            name: "atmospheric_state".into(),
            unit: "multi".into(),
            families: vec![ProcessFamily::Atmosphere],
            description: "Hourly reanalysis: T, q, u, v, P, radiation, pressure levels".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1940,
                end_year: None,
            },
            spatial_resolution_m: Some(31_000.0),
            cadence: Cadence::Hourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec![
                "precipitation over-estimation in tropics".into(),
                "observing-system transitions".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://cds.climate.copernicus.eu/api/v2".into(),
        }],
        formats: vec![DataFormat::Grib2, DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "bias".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere],
        tags: vec![
            "reanalysis".into(),
            "forcing".into(),
            "global".into(),
            "atmosphere".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Human Systems
// ---------------------------------------------------------------------------

fn eia_energy() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("EIA_ENERGY"),
        name: "EIA_ENERGY".into(),
        description: "EIA — U.S. energy generation, consumption, and emissions data".into(),
        provider: "U.S. Energy Information Administration".into(),
        observable: Observable {
            name: "energy_generation".into(),
            unit: "GWh".into(),
            families: vec![ProcessFamily::HumanSystems],
            description: "Monthly electricity generation by source, state-level".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::CONUS,
            temporal: TemporalExtent {
                start_year: 2001,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Polygon,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.02),
            known_biases: vec![],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://api.eia.gov/v2/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::HumanSystems],
        tags: vec!["energy".into(), "emissions".into(), "infrastructure".into()],
    }
}

// ---------------------------------------------------------------------------
// Trophic Dynamics / Biodiversity
// ---------------------------------------------------------------------------

fn gbif() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GBIF"),
        name: "GBIF".into(),
        description: "Global Biodiversity Information Facility — species occurrence records".into(),
        provider: "GBIF Secretariat".into(),
        observable: Observable {
            name: "species_occurrence".into(),
            unit: "count".into(),
            families: vec![
                ProcessFamily::Ecology,
                ProcessFamily::TrophicDynamics,
                ProcessFamily::Evolution,
            ],
            description:
                "Georeferenced species occurrence records from museums, surveys, citizen science"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1600,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec![
                "sampling bias toward roads/cities".into(),
                "taxonomic bias toward vertebrates".into(),
            ],
            quality_flag: Some("coordinateUncertaintyInMeters".into()),
        },
        access: vec![AccessMethod::Api {
            base_url: "https://api.gbif.org/v1/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![
            ProcessFamily::Ecology,
            ProcessFamily::TrophicDynamics,
            ProcessFamily::Evolution,
        ],
        tags: vec!["biodiversity".into(), "occurrence".into(), "global".into()],
    }
}

fn ebird() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("EBIRD"),
        name: "EBIRD".into(),
        description: "eBird — citizen-science bird observations with effort data".into(),
        provider: "Cornell Lab of Ornithology".into(),
        observable: Observable {
            name: "bird_abundance".into(),
            unit: "count".into(),
            families: vec![ProcessFamily::TrophicDynamics, ProcessFamily::Ecology],
            description: "Bird checklist observations with survey effort and detection probability"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2002,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec![
                "observer skill variation".into(),
                "spatial clustering near population centers".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://api.ebird.org/v2/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::TrophicDynamics, ProcessFamily::Ecology],
        tags: vec![
            "birds".into(),
            "citizen_science".into(),
            "biodiversity".into(),
        ],
    }
}

fn fao_fisheries() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("FAO_FISHERIES"),
        name: "FAO_FISHERIES".into(),
        description: "FAO Global Capture Production — marine & freshwater fisheries landings"
            .into(),
        provider: "FAO".into(),
        observable: Observable {
            name: "fisheries_catch".into(),
            unit: "t yr-1".into(),
            families: vec![ProcessFamily::TrophicDynamics, ProcessFamily::Ocean],
            description: "Annual capture production by species, FAO area, country".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1950,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Annual,
            topology: SpatialTopology::Polygon,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.30),
            known_biases: vec![
                "unreported/illegal catch".into(),
                "discards underestimated".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.fao.org/fishery/topic/16140/en".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::OpenData,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::TrophicDynamics, ProcessFamily::Ocean],
        tags: vec!["fisheries".into(), "marine".into(), "trophic".into()],
    }
}

fn try_traits() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("TRY_TRAITS"),
        name: "TRY_TRAITS".into(),
        description: "TRY Plant Trait Database — global plant functional traits".into(),
        provider: "TRY Initiative / MPI-BGC".into(),
        observable: Observable {
            name: "plant_traits".into(),
            unit: "multi".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Evolution],
            description: "SLA, wood density, seed mass, N content, photosynthetic rate, etc."
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1900,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.20),
            known_biases: vec!["temperate/boreal over-representation".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://www.try-db.org/TryWeb/dp.php".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::RestrictedAcademic,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Evolution],
        tags: vec![
            "traits".into(),
            "functional_ecology".into(),
            "global".into(),
        ],
    }
}

fn bien_traits() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("BIEN_TRAITS"),
        name: "BIEN_TRAITS".into(),
        description: "BIEN — Botanical Information and Ecology Network species & traits".into(),
        provider: "BIEN Working Group".into(),
        observable: Observable {
            name: "plant_traits".into(),
            unit: "multi".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Evolution],
            description: "Occurrence + functional trait data for New World plants".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0,
                south: -60.0,
                east: -30.0,
                north: 80.0,
            },
            temporal: TemporalExtent {
                start_year: 1800,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec!["New World focused".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://bien.nceas.ucsb.edu/bien/biendata/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Evolution],
        tags: vec!["traits".into(), "new_world".into(), "plants".into()],
    }
}

// ---------------------------------------------------------------------------
// Evolution / Paleo
// ---------------------------------------------------------------------------

fn pbdb() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("PBDB"),
        name: "PBDB".into(),
        description: "Paleobiology Database — fossil occurrence and stratigraphic ranges".into(),
        provider: "PBDB".into(),
        observable: Observable {
            name: "fossil_occurrence".into(),
            unit: "count".into(),
            families: vec![ProcessFamily::Evolution, ProcessFamily::Ecology],
            description:
                "Fossil occurrences: taxon, locality, age range, paleocoordinates, environment"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: -542_000_000,
                end_year: Some(0),
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec![
                "pull-of-the-Recent".into(),
                "Lagerstätten effects".into(),
                "marine-biased".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://paleobiodb.org/data1.2/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Evolution, ProcessFamily::Ecology],
        tags: vec![
            "fossil".into(),
            "paleo".into(),
            "extinction".into(),
            "deep_time".into(),
        ],
    }
}

fn timetree() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("TIMETREE"),
        name: "TIMETREE".into(),
        description: "TimeTree — molecular divergence times (phylogenetic calibration)".into(),
        provider: "Temple University".into(),
        observable: Observable {
            name: "divergence_time".into(),
            unit: "Myr".into(),
            families: vec![ProcessFamily::Evolution],
            description: "Molecular clock-calibrated divergence times between taxa".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: -2_000_000_000,
                end_year: Some(0),
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec!["calibration fossil selection".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "http://www.timetree.org/api/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::PublicDomain,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Evolution],
        tags: vec![
            "phylogenetics".into(),
            "molecular_clock".into(),
            "divergence".into(),
        ],
    }
}

fn quaternary_pollen() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("NEOTOMA_POLLEN"),
        name: "NEOTOMA_POLLEN".into(),
        description: "Neotoma Paleoecology Database — Quaternary pollen & plant macrofossils"
            .into(),
        provider: "Neotoma / PAGES".into(),
        observable: Observable {
            name: "pollen_abundance".into(),
            unit: "percent".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Evolution],
            description: "Pollen percentages and concentrations from lake and bog sediment cores"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: -2_600_000,
                end_year: Some(0),
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "taphonomic bias (wind-pollinated over-represented)".into(),
                "chronology uncertainty".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://api.neotomadb.org/v2.0/".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Evolution],
        tags: vec!["pollen".into(), "quaternary".into(), "paleoecology".into()],
    }
}

// ---------------------------------------------------------------------------
// Planetary Defense
// ---------------------------------------------------------------------------

fn cneos_sentry() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CNEOS_SENTRY"),
        name: "CNEOS_SENTRY".into(),
        description: "CNEOS Sentry — near-Earth object impact risk monitoring".into(),
        provider: "NASA JPL CNEOS".into(),
        observable: Observable {
            name: "impact_probability".into(),
            unit: "probability".into(),
            families: vec![ProcessFamily::Geology],
            description: "Impact probability estimates for potentially hazardous asteroids".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2002,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Daily,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec!["orbit determination quality varies by object".into()],
            quality_flag: Some("ip_flag".into()),
        },
        access: vec![AccessMethod::Api {
            base_url: "https://ssd-api.jpl.nasa.gov/sentry.api".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "palermo_scale".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geology],
        tags: vec![
            "neo".into(),
            "asteroid".into(),
            "impact_risk".into(),
            "planetary_defense".into(),
        ],
    }
}

fn mpc_orbits() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MPC_ORBITS"),
        name: "MPC_ORBITS".into(),
        description: "Minor Planet Center — orbital elements for all known minor planets".into(),
        provider: "IAU Minor Planet Center".into(),
        observable: Observable {
            name: "orbital_elements".into(),
            unit: "multi".into(),
            families: vec![ProcessFamily::Geology],
            description: "Keplerian orbital elements (a, e, i, Ω, ω, M) for asteroids & comets"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1801,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Daily,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: None,
            known_biases: vec!["discovery bias toward large/bright objects".into()],
            quality_flag: Some("uncertainty_parameter".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://minorplanetcenter.net/iau/MPCORB.html".into(),
        }],
        formats: vec![DataFormat::Csv],
        license: License::PublicDomain,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geology],
        tags: vec![
            "orbits".into(),
            "asteroids".into(),
            "comets".into(),
            "neo".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Geomorphology / DEM
// ---------------------------------------------------------------------------

fn copernicus_dem() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("COP_DEM_30"),
        name: "COP_DEM_30".into(),
        description: "Copernicus DEM GLO-30 — global digital elevation model (30 m)".into(),
        provider: "ESA / Airbus".into(),
        observable: Observable {
            name: "elevation".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Geomorphology, ProcessFamily::Hydrology],
            description: "Surface elevation from TanDEM-X interferometric SAR".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2021,
                end_year: Some(2021),
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(4.0),
            relative: None,
            known_biases: vec!["vegetation canopy bias in forested areas".into()],
            quality_flag: Some("EDM".into()),
        },
        access: vec![AccessMethod::Stac {
            catalog_url: "https://planetarycomputer.microsoft.com/api/stac/v1".into(),
            collection: "cop-dem-glo-30".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::OpenData,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geomorphology, ProcessFamily::Hydrology],
        tags: vec![
            "dem".into(),
            "elevation".into(),
            "topography".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Optical / Multispectral
// ---------------------------------------------------------------------------

fn landsat_c2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("LANDSAT_C2"),
        name: "LANDSAT_C2".into(),
        description:
            "USGS Landsat Collection 2 — calibrated surface reflectance (L4–L9, 1982–present)"
                .into(),
        provider: "USGS / NASA".into(),
        observable: Observable {
            name: "surface_reflectance".into(),
            unit: "dimensionless".into(),
            families: vec![
                ProcessFamily::Ecology,
                ProcessFamily::Hydrology,
                ProcessFamily::HumanSystems,
            ],
            description: "Multispectral surface reflectance (6–11 bands, 30 m, 16-day revisit)"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1982,
                end_year: None,
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec!["atmospheric correction residuals in steep terrain".into()],
            quality_flag: Some("QA_PIXEL".into()),
        },
        access: vec![
            AccessMethod::Stac {
                catalog_url: "https://planetarycomputer.microsoft.com/api/stac/v1".into(),
                collection: "landsat-c2-l2".into(),
            },
            AccessMethod::HttpDownload {
                url_pattern: "https://earthexplorer.usgs.gov/".into(),
            },
        ],
        formats: vec![DataFormat::GeoTiff],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "spectral_rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![
            ProcessFamily::Ecology,
            ProcessFamily::Hydrology,
            ProcessFamily::HumanSystems,
        ],
        tags: vec![
            "landsat".into(),
            "reflectance".into(),
            "multispectral".into(),
            "land_cover".into(),
            "global".into(),
        ],
    }
}

fn sentinel2_msi() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SENTINEL2_MSI"),
        name: "SENTINEL2_MSI".into(),
        description: "ESA Sentinel-2 MultiSpectral Instrument — 10 m surface reflectance".into(),
        provider: "ESA / Copernicus".into(),
        observable: Observable {
            name: "surface_reflectance".into(),
            unit: "dimensionless".into(),
            families: vec![
                ProcessFamily::Ecology,
                ProcessFamily::Hydrology,
                ProcessFamily::HumanSystems,
            ],
            description:
                "13-band multispectral imagery (10–60 m, 5-day revisit with twin satellites)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2015,
                end_year: None,
            },
            spatial_resolution_m: Some(10.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.03),
            known_biases: vec!["cirrus contamination in Band 10".into()],
            quality_flag: Some("SCL".into()),
        },
        access: vec![
            AccessMethod::Stac {
                catalog_url: "https://planetarycomputer.microsoft.com/api/stac/v1".into(),
                collection: "sentinel-2-l2a".into(),
            },
            AccessMethod::HttpDownload {
                url_pattern: "https://scihub.copernicus.eu/dhus/".into(),
            },
        ],
        formats: vec![DataFormat::GeoTiff],
        license: License::OpenData,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "spectral_rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![
            ProcessFamily::Ecology,
            ProcessFamily::Hydrology,
            ProcessFamily::HumanSystems,
        ],
        tags: vec![
            "sentinel-2".into(),
            "reflectance".into(),
            "multispectral".into(),
            "high_resolution".into(),
            "global".into(),
        ],
    }
}

fn sentinel1_sar() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SENTINEL1_SAR"),
        name: "SENTINEL1_SAR".into(),
        description: "ESA Sentinel-1 C-band Synthetic Aperture Radar — all-weather imaging".into(),
        provider: "ESA / Copernicus".into(),
        observable: Observable {
            name: "radar_backscatter".into(),
            unit: "dB".into(),
            families: vec![
                ProcessFamily::Hydrology,
                ProcessFamily::Cryosphere,
                ProcessFamily::Ecology,
            ],
            description: "C-band SAR backscatter (VV/VH polarisation, 10 m, 6–12 day revisit)"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2014,
                end_year: None,
            },
            spatial_resolution_m: Some(10.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: None,
            known_biases: vec!["speckle noise requires multi-look averaging".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Stac {
            catalog_url: "https://planetarycomputer.microsoft.com/api/stac/v1".into(),
            collection: "sentinel-1-grd".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::OpenData,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "backscatter_rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![
            ProcessFamily::Hydrology,
            ProcessFamily::Cryosphere,
            ProcessFamily::Ecology,
        ],
        tags: vec![
            "sentinel-1".into(),
            "sar".into(),
            "radar".into(),
            "all_weather".into(),
            "global".into(),
        ],
    }
}

fn avhrr_gimms_ndvi() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("AVHRR_GIMMS_NDVI3G"),
        name: "AVHRR_GIMMS_NDVI3G".into(),
        description: "AVHRR GIMMS NDVI3g — 40-year global vegetation index record".into(),
        provider: "NASA GSFC".into(),
        observable: Observable {
            name: "ndvi".into(),
            unit: "dimensionless".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
            description: "Biweekly maximum-value composite NDVI from AVHRR (1/12° resolution)"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1981,
                end_year: Some(2015),
            },
            spatial_resolution_m: Some(8000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.05),
            relative: None,
            known_biases: vec![
                "orbital drift correction residuals in early NOAA satellites".into(),
                "volcanic aerosol contamination (Pinatubo 1991)".into(),
            ],
            quality_flag: Some("QA_flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://ecocast.arc.nasa.gov/data/pub/gimms/3g.v1/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "r_squared".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
        tags: vec![
            "avhrr".into(),
            "ndvi".into(),
            "vegetation".into(),
            "long_record".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — MODIS Extended Products
// ---------------------------------------------------------------------------

fn modis_lst() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_LST"),
        name: "MODIS_LST".into(),
        description: "MODIS Land Surface Temperature (MOD11A1/MYD11A1) — daily 1 km".into(),
        provider: "NASA LP DAAC".into(),
        observable: Observable {
            name: "land_surface_temperature".into(),
            unit: "K".into(),
            families: vec![ProcessFamily::Radiation, ProcessFamily::Atmosphere],
            description: "Day/night land surface temperature and emissivity from thermal IR".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(1000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: None,
            known_biases: vec!["clear-sky bias (no retrievals under cloud)".into()],
            quality_flag: Some("QC_Day / QC_Night".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MOD11A1".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Radiation, ProcessFamily::Atmosphere],
        tags: vec![
            "modis".into(),
            "lst".into(),
            "temperature".into(),
            "thermal_ir".into(),
            "global".into(),
        ],
    }
}

fn modis_burned_area() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_MCD64A1"),
        name: "MODIS_MCD64A1".into(),
        description: "MODIS Burned Area (MCD64A1) — monthly 500 m global fire extent".into(),
        provider: "NASA LP DAAC".into(),
        observable: Observable {
            name: "burned_area".into(),
            unit: "binary (burn date)".into(),
            families: vec![ProcessFamily::Fire, ProcessFamily::Ecology],
            description:
                "Monthly burned-area mapping using surface reflectance change and active-fire data"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "omission of small fires in cropland regions".into(),
                "commission errors in spectrally variable surfaces".into(),
            ],
            quality_flag: Some("BA_QA".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MCD64A1".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "dice_coefficient".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Fire, ProcessFamily::Ecology],
        tags: vec![
            "modis".into(),
            "burned_area".into(),
            "fire".into(),
            "global".into(),
        ],
    }
}

fn modis_albedo() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_MCD43A3"),
        name: "MODIS_MCD43A3".into(),
        description: "MODIS BRDF/Albedo (MCD43A3) — daily 500 m surface albedo".into(),
        provider: "NASA LP DAAC".into(),
        observable: Observable {
            name: "surface_albedo".into(),
            unit: "dimensionless".into(),
            families: vec![ProcessFamily::Radiation],
            description:
                "Black-sky and white-sky albedo from BRDF model inversion (16-day sliding window)"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.02),
            relative: None,
            known_biases: vec!["sub-pixel snow contamination in transition seasons".into()],
            quality_flag: Some("BRDF_Albedo_Band_Mandatory_Quality".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MCD43A3".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Radiation],
        tags: vec![
            "modis".into(),
            "albedo".into(),
            "brdf".into(),
            "radiation".into(),
            "global".into(),
        ],
    }
}

fn modis_gpp() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_MOD17A2H"),
        name: "MODIS_MOD17A2H".into(),
        description: "MODIS GPP (MOD17A2H) — 8-day 500 m gross primary productivity".into(),
        provider: "NASA LP DAAC / UMT NTSG".into(),
        observable: Observable {
            name: "gross_primary_productivity".into(),
            unit: "kg C m-2 8day-1".into(),
            families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Ecology],
            description: "Light-use efficiency model driven by MODIS fPAR and GMAO meteorology"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.25),
            known_biases: vec![
                "systematic underestimate in tropical forests".into(),
                "meteorology driver errors propagate to GPP".into(),
            ],
            quality_flag: Some("Psn_QC".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MOD17A2H".into(),
            version: "061".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Ecology],
        tags: vec![
            "modis".into(),
            "gpp".into(),
            "productivity".into(),
            "carbon".into(),
            "global".into(),
        ],
    }
}

fn modis_snow_cover() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_MOD10A1"),
        name: "MODIS_MOD10A1".into(),
        description: "MODIS Snow Cover (MOD10A1) — daily 500 m fractional snow cover".into(),
        provider: "NASA NSIDC DAAC".into(),
        observable: Observable {
            name: "snow_cover_fraction".into(),
            unit: "fraction".into(),
            families: vec![ProcessFamily::Cryosphere, ProcessFamily::Hydrology],
            description: "NDSI-based fractional snow cover and snow albedo".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.10),
            relative: None,
            known_biases: vec![
                "cloud obscuration reduces effective temporal coverage".into(),
                "forest canopy masks underlying snow".into(),
            ],
            quality_flag: Some("NDSI_Snow_Cover_Basic_QA".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "MOD10A1".into(),
            version: "61".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "accuracy".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Cryosphere, ProcessFamily::Hydrology],
        tags: vec![
            "modis".into(),
            "snow".into(),
            "cryosphere".into(),
            "ndsi".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Precipitation
// ---------------------------------------------------------------------------

fn gpm_imerg() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GPM_IMERG"),
        name: "GPM_IMERG".into(),
        description: "NASA GPM IMERG — global half-hourly 0.1° merged precipitation".into(),
        provider: "NASA GES DISC".into(),
        observable: Observable {
            name: "precipitation_rate".into(),
            unit: "mm hr-1".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere],
            description: "Multi-satellite merged precipitation estimate (passive MW + IR calibration + gauge)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -60.0, east: 180.0, north: 60.0,
            },
            temporal: TemporalExtent { start_year: 2000, end_year: None },
            spatial_resolution_m: Some(11_000.0),
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.30),
            known_biases: vec![
                "underestimate of orographic precipitation".into(),
                "overestimate of light warm-rain events".into(),
            ],
            quality_flag: Some("precipitationQualityIndex".into()),
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://gpm1.gesdisc.eosdis.nasa.gov/opendap/GPM_L3/".into(),
        }],
        formats: vec![DataFormat::NetCdf, DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere],
        tags: vec!["gpm".into(), "imerg".into(), "precipitation".into(), "satellite".into(), "global".into()],
    }
}

fn chirps() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CHIRPS"),
        name: "CHIRPS".into(),
        description: "CHIRPS v2.0 — quasi-global 0.05° daily/pentadal rainfall (1981–present)"
            .into(),
        provider: "UCSB Climate Hazards Center".into(),
        observable: Observable {
            name: "precipitation".into(),
            unit: "mm day-1".into(),
            families: vec![ProcessFamily::Hydrology],
            description: "Blended satellite/gauge gridded precipitation for drought monitoring"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0,
                south: -50.0,
                east: 180.0,
                north: 50.0,
            },
            temporal: TemporalExtent {
                start_year: 1981,
                end_year: None,
            },
            spatial_resolution_m: Some(5500.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.20),
            known_biases: vec![
                "gauge density limitations in central Africa and high-latitude regions".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://data.chc.ucsb.edu/products/CHIRPS-2.0/".into(),
        }],
        formats: vec![DataFormat::GeoTiff, DataFormat::NetCdf],
        license: License::PublicDomain,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec![
            "chirps".into(),
            "precipitation".into(),
            "rainfall".into(),
            "drought".into(),
            "quasi_global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Atmospheric Composition
// ---------------------------------------------------------------------------

fn oco2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("OCO2_CO2"),
        name: "OCO2_CO2".into(),
        description: "NASA OCO-2/OCO-3 — column-averaged dry-air CO₂ mole fraction (XCO₂)".into(),
        provider: "NASA JPL".into(),
        observable: Observable {
            name: "xco2".into(),
            unit: "ppm".into(),
            families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere],
            description: "Column-averaged CO₂ from high-resolution grating spectrometer (1.29 × 2.25 km footprint)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2014, end_year: None },
            spatial_resolution_m: Some(2250.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: None,
            known_biases: vec![
                "land–ocean bias of ~0.3 ppm".into(),
                "aerosol-related systematic errors over bright surfaces".into(),
            ],
            quality_flag: Some("xco2_quality_flag".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "OCO2_L2_Lite_FP".into(),
            version: "11r".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere],
        tags: vec!["oco-2".into(), "co2".into(), "carbon".into(), "greenhouse_gas".into(), "column".into()],
    }
}

fn tropomi_s5p() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("TROPOMI_S5P"),
        name: "TROPOMI_S5P".into(),
        description: "Sentinel-5P TROPOMI — global trace gas and aerosol columns".into(),
        provider: "ESA / KNMI".into(),
        observable: Observable {
            name: "trace_gas_columns".into(),
            unit: "mol m-2".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Biogeochemistry],
            description: "NO₂, O₃, CH₄, CO, SO₂, HCHO tropospheric/total columns (3.5 × 5.5 km)"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2017,
                end_year: None,
            },
            spatial_resolution_m: Some(5500.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "surface albedo sensitivity in NO₂ retrievals".into(),
                "a-priori profile shape dependency".into(),
            ],
            quality_flag: Some("qa_value".into()),
        },
        access: vec![AccessMethod::Stac {
            catalog_url: "https://planetarycomputer.microsoft.com/api/stac/v1".into(),
            collection: "sentinel-5p-l2-no2".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "nrmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 2.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Biogeochemistry],
        tags: vec![
            "tropomi".into(),
            "sentinel-5p".into(),
            "no2".into(),
            "methane".into(),
            "ozone".into(),
            "trace_gas".into(),
        ],
    }
}

fn gosat() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GOSAT_GHG"),
        name: "GOSAT_GHG".into(),
        description: "GOSAT/IBUKI — column-averaged CO₂ and CH₄ (2009–present)".into(),
        provider: "JAXA / NIES / MOE".into(),
        observable: Observable {
            name: "xco2_xch4".into(),
            unit: "ppm / ppb".into(),
            families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere],
            description:
                "Column-averaged greenhouse gas (CO₂, CH₄) from TANSO-FTS SWIR spectrometer".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2009,
                end_year: None,
            },
            spatial_resolution_m: Some(10_500.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.5),
            relative: None,
            known_biases: vec!["glint-mode vs nadir-mode systematic offset".into()],
            quality_flag: Some("xco2_quality_flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://data2.gosat.nies.go.jp/".into(),
        }],
        formats: vec![DataFormat::NetCdf, DataFormat::Hdf5],
        license: License::OpenData,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 3.0,
        },
        evaluates_families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere],
        tags: vec![
            "gosat".into(),
            "co2".into(),
            "methane".into(),
            "greenhouse_gas".into(),
            "global".into(),
        ],
    }
}

fn calipso_caliop() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CALIPSO_CALIOP"),
        name: "CALIPSO_CALIOP".into(),
        description: "CALIPSO CALIOP — global spaceborne lidar aerosol/cloud profiles".into(),
        provider: "NASA LaRC / CNES".into(),
        observable: Observable {
            name: "aerosol_cloud_profile".into(),
            unit: "km-1 sr-1".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation],
            description: "Attenuated backscatter profiles (532/1064 nm) resolving aerosol and cloud vertical structure".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2006, end_year: Some(2023) },
            spatial_resolution_m: Some(333.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Profile,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec![
                "signal attenuation below optically thick clouds".into(),
                "daytime noise floor higher than nighttime".into(),
            ],
            quality_flag: Some("CAD_Score".into()),
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://opendap.larc.nasa.gov/opendap/CALIPSO/".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "profile_rmse".into(),
            extraction: ExtractionMethod::ProfileExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation],
        tags: vec!["calipso".into(), "lidar".into(), "aerosol".into(), "cloud_profile".into(), "global".into()],
    }
}

fn cloudsat() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CLOUDSAT_CPR"),
        name: "CLOUDSAT_CPR".into(),
        description: "CloudSat Cloud Profiling Radar — 94 GHz vertical cloud structure".into(),
        provider: "NASA JPL / CSA".into(),
        observable: Observable {
            name: "cloud_radar_reflectivity".into(),
            unit: "dBZ".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation],
            description:
                "94 GHz W-band radar profiles of cloud liquid/ice water content and precipitation"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2006,
                end_year: None,
            },
            spatial_resolution_m: Some(1400.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Profile,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.5),
            relative: None,
            known_biases: vec![
                "ground clutter in lowest 1 km".into(),
                "attenuation in heavy precipitation".into(),
            ],
            quality_flag: Some("Data_quality".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.cloudsat.cira.colostate.edu/dataSpecs.php".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "profile_rmse".into(),
            extraction: ExtractionMethod::ProfileExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation],
        tags: vec![
            "cloudsat".into(),
            "radar".into(),
            "cloud".into(),
            "precipitation".into(),
            "profile".into(),
        ],
    }
}

fn goes_r_abi() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GOES_R_ABI"),
        name: "GOES_R_ABI".into(),
        description: "GOES-R Advanced Baseline Imager — geostationary 16-band imagery".into(),
        provider: "NOAA NESDIS".into(),
        observable: Observable {
            name: "radiance_reflectance".into(),
            unit: "W m-2 sr-1 / dimensionless".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation, ProcessFamily::Fire],
            description: "16-band geostationary imagery (0.5–2 km, 5–15 min) for weather, fire, and radiation".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -160.0, south: -60.0, east: -20.0, north: 60.0,
            },
            temporal: TemporalExtent { start_year: 2017, end_year: None },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec!["navigation error at scan edges".into()],
            quality_flag: Some("DQF".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.avl.class.noaa.gov/saa/products/welcome".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation, ProcessFamily::Fire],
        tags: vec!["goes".into(), "geostationary".into(), "weather".into(), "fire_detection".into(), "nrt".into()],
    }
}

fn merra2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MERRA2"),
        name: "MERRA2".into(),
        description: "NASA MERRA-2 — Modern-Era Retrospective analysis for Research (1980–present)".into(),
        provider: "NASA GMAO".into(),
        observable: Observable {
            name: "aerosol_meteorology".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation],
            description: "Assimilated aerosol optical depth, meteorological fields, and radiation budgets (0.5° × 0.625°)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1980, end_year: None },
            spatial_resolution_m: Some(55_000.0),
            cadence: Cadence::Hourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "pre-satellite era (before 1979) less constrained".into(),
                "AOD assimilation introduces discontinuities at instrument transitions".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://goldsmr5.gesdisc.eosdis.nasa.gov/opendap/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Radiation],
        tags: vec!["merra-2".into(), "reanalysis".into(), "aerosol".into(), "meteorology".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Ocean
// ---------------------------------------------------------------------------

fn ghrsst() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GHRSST_L4"),
        name: "GHRSST_L4".into(),
        description: "GHRSST L4 — multi-sensor merged sea surface temperature (1981–present)"
            .into(),
        provider: "GHRSST / NOAA / UKMO".into(),
        observable: Observable {
            name: "sea_surface_temperature".into(),
            unit: "K".into(),
            families: vec![ProcessFamily::Ocean],
            description:
                "Gap-free daily SST from merged IR + MW satellite sensors (OSTIA, MUR, etc.)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1981,
                end_year: None,
            },
            spatial_resolution_m: Some(1000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.3),
            relative: None,
            known_biases: vec![
                "diurnal warming not fully resolved in daily means".into(),
                "sea-ice edge SST artifacts".into(),
            ],
            quality_flag: Some("quality_level".into()),
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://podaac-opendap.jpl.nasa.gov/opendap/allData/ghrsst/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean],
        tags: vec![
            "ghrsst".into(),
            "sst".into(),
            "ocean_temperature".into(),
            "global".into(),
        ],
    }
}

fn modis_ocean_color() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("MODIS_OCEAN_COLOR"),
        name: "MODIS_OCEAN_COLOR".into(),
        description: "MODIS/Aqua Ocean Color — global chlorophyll-a and ocean productivity".into(),
        provider: "NASA OB.DAAC".into(),
        observable: Observable {
            name: "chlorophyll_a".into(),
            unit: "mg m-3".into(),
            families: vec![ProcessFamily::Ocean, ProcessFamily::Biogeochemistry],
            description: "Chlorophyll-a concentration from OC3M algorithm (4 km, 8-day composite)"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2002,
                end_year: None,
            },
            spatial_resolution_m: Some(4_000.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.35),
            known_biases: vec![
                "overestimate in turbid coastal/river plume waters".into(),
                "sun-glint contamination in tropical latitudes".into(),
            ],
            quality_flag: Some("l2_flags".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://oceancolor.gsfc.nasa.gov/l3/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "log_rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean, ProcessFamily::Biogeochemistry],
        tags: vec![
            "modis".into(),
            "ocean_color".into(),
            "chlorophyll".into(),
            "productivity".into(),
            "global".into(),
        ],
    }
}

fn swot() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SWOT"),
        name: "SWOT".into(),
        description: "SWOT — Surface Water and Ocean Topography (wide-swath altimetry)".into(),
        provider: "NASA / CNES".into(),
        observable: Observable {
            name: "water_surface_elevation".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Ocean],
            description: "KaRIn interferometric radar measures river/lake/reservoir water levels and ocean SSH".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -78.0, east: 180.0, north: 78.0,
            },
            temporal: TemporalExtent { start_year: 2022, end_year: None },
            spatial_resolution_m: Some(100.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.10),
            relative: None,
            known_biases: vec!["layover effects in narrow river valleys".into()],
            quality_flag: Some("quality_f".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "SWOT_L2_HR_RiverSP".into(),
            version: "2.0".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 1.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Ocean],
        tags: vec!["swot".into(), "altimetry".into(), "river".into(), "lake".into(), "ocean_ssh".into()],
    }
}

fn sentinel6() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SENTINEL6_MF"),
        name: "SENTINEL6_MF".into(),
        description:
            "Sentinel-6 Michael Freilich — precision radar altimetry (sea level reference)".into(),
        provider: "ESA / EUMETSAT / NASA".into(),
        observable: Observable {
            name: "sea_surface_height".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Ocean],
            description:
                "Ku/C-band dual-frequency radar altimeter continuing Jason series reference orbit"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0,
                south: -66.0,
                east: 180.0,
                north: 66.0,
            },
            temporal: TemporalExtent {
                start_year: 2020,
                end_year: None,
            },
            spatial_resolution_m: Some(300.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Transect,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.035),
            relative: None,
            known_biases: vec!["wet troposphere correction residuals in coastal zones".into()],
            quality_flag: Some("surface_type_flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://podaac.jpl.nasa.gov/Sentinel-6".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean],
        tags: vec![
            "sentinel-6".into(),
            "altimetry".into(),
            "sea_level".into(),
            "ocean".into(),
            "reference_mission".into(),
        ],
    }
}

fn sentinel3_olci() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SENTINEL3_OLCI"),
        name: "SENTINEL3_OLCI".into(),
        description: "Sentinel-3 OLCI — Ocean and Land Colour Instrument (21-band, 300 m)".into(),
        provider: "ESA / EUMETSAT".into(),
        observable: Observable {
            name: "water_leaving_reflectance".into(),
            unit: "dimensionless".into(),
            families: vec![ProcessFamily::Ocean, ProcessFamily::Ecology],
            description:
                "21-band push-broom imager for ocean colour, vegetation, and fire monitoring".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2016,
                end_year: None,
            },
            spatial_resolution_m: Some(300.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec!["adjacency effects in coastal waters".into()],
            quality_flag: Some("WQSF".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://scihub.copernicus.eu/dhus/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean, ProcessFamily::Ecology],
        tags: vec![
            "sentinel-3".into(),
            "olci".into(),
            "ocean_color".into(),
            "vegetation".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Cryosphere / Altimetry
// ---------------------------------------------------------------------------

fn icesat2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ICESAT2_ATL"),
        name: "ICESAT2_ATL".into(),
        description: "NASA ICESat-2 ATLAS — photon-counting lidar elevation (ice, land, canopy, water)".into(),
        provider: "NASA NSIDC DAAC".into(),
        observable: Observable {
            name: "surface_elevation".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Cryosphere, ProcessFamily::Ecology, ProcessFamily::Hydrology],
            description: "Photon-counting 532 nm lidar providing surface elevation, canopy height, and bathymetry".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2018, end_year: None },
            spatial_resolution_m: Some(11.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Transect,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.03),
            relative: None,
            known_biases: vec![
                "forward scattering bias in cloud/aerosol layers".into(),
                "canopy-top bias in dense tropical forests".into(),
            ],
            quality_flag: Some("atl06_quality_summary".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "ATL06".into(),
            version: "006".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Cryosphere, ProcessFamily::Ecology, ProcessFamily::Hydrology],
        tags: vec!["icesat-2".into(), "lidar".into(), "elevation".into(), "ice_sheet".into(), "canopy_height".into()],
    }
}

fn cryosat2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CRYOSAT2"),
        name: "CRYOSAT2".into(),
        description: "ESA CryoSat-2 — radar altimetry for ice sheet and sea-ice freeboard".into(),
        provider: "ESA".into(),
        observable: Observable {
            name: "ice_elevation_freeboard".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Cryosphere, ProcessFamily::Ocean],
            description: "SIRAL interferometric/SAR radar altimeter measuring ice sheet elevation and sea-ice freeboard".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2010, end_year: None },
            spatial_resolution_m: Some(380.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Transect,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.10),
            relative: None,
            known_biases: vec![
                "snow loading on sea ice biases freeboard high".into(),
                "radar penetration into firn on ice sheets".into(),
            ],
            quality_flag: Some("Product_confidence_data".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://science-pds.cryosat.esa.int/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Cryosphere, ProcessFamily::Ocean],
        tags: vec!["cryosat-2".into(), "radar_altimetry".into(), "ice_sheet".into(), "sea_ice".into(), "freeboard".into()],
    }
}

fn amsr2() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("AMSR2"),
        name: "AMSR2".into(),
        description: "GCOM-W AMSR2 — passive microwave soil moisture, snow, SST, and sea ice".into(),
        provider: "JAXA".into(),
        observable: Observable {
            name: "brightness_temperature".into(),
            unit: "K".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Cryosphere, ProcessFamily::Ocean],
            description: "Multi-frequency passive microwave (6.9–89 GHz) brightness temperatures for geophysical retrievals".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2012, end_year: None },
            spatial_resolution_m: Some(10_000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: None,
            known_biases: vec![
                "RFI contamination in C-band channels over land".into(),
                "wind-roughness dependency in ocean retrievals".into(),
            ],
            quality_flag: Some("Pixel_Data_Quality".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://gportal.jaxa.jp/gpr/search?tab=1".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::OpenData,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Cryosphere, ProcessFamily::Ocean],
        tags: vec!["amsr2".into(), "passive_microwave".into(), "soil_moisture".into(), "snow".into(), "sea_ice".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Land Cover / Surface Water
// ---------------------------------------------------------------------------

fn esa_cci_land_cover() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ESA_CCI_LC"),
        name: "ESA_CCI_LC".into(),
        description: "ESA CCI Land Cover — annual 300 m global land cover maps (1992–present)"
            .into(),
        provider: "ESA Climate Change Initiative".into(),
        observable: Observable {
            name: "land_cover_class".into(),
            unit: "class".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::HumanSystems],
            description:
                "Annual land cover classification (37 classes, UN-LCCS) from multi-sensor fusion"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1992,
                end_year: None,
            },
            spatial_resolution_m: Some(300.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "confusion between cropland and grassland in semi-arid zones".into(),
                "temporal consistency artifacts at sensor transitions".into(),
            ],
            quality_flag: Some("processed_flag".into()),
        },
        access: vec![AccessMethod::Stac {
            catalog_url: "https://planetarycomputer.microsoft.com/api/stac/v1".into(),
            collection: "esa-cci-lc".into(),
        }],
        formats: vec![DataFormat::GeoTiff, DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "overall_accuracy".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::HumanSystems],
        tags: vec![
            "esa_cci".into(),
            "land_cover".into(),
            "classification".into(),
            "change_detection".into(),
            "global".into(),
        ],
    }
}

fn jrc_global_surface_water() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("JRC_GSW"),
        name: "JRC_GSW".into(),
        description:
            "JRC Global Surface Water — 30 m Landsat-derived water dynamics (1984–present)".into(),
        provider: "EC Joint Research Centre".into(),
        observable: Observable {
            name: "water_occurrence".into(),
            unit: "fraction".into(),
            families: vec![ProcessFamily::Hydrology],
            description:
                "Per-pixel water occurrence, seasonality, and transitions from 40+ years of Landsat"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1984,
                end_year: None,
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec![
                "cloud-shadow misclassification as water".into(),
                "reservoir filling events may lag in monthly composites".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://global-surface-water.appspot.com/download".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "accuracy".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec![
            "jrc".into(),
            "surface_water".into(),
            "landsat".into(),
            "occurrence".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Elevation / Topography
// ---------------------------------------------------------------------------

fn srtm() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SRTM_V3"),
        name: "SRTM_V3".into(),
        description: "NASA SRTM v3 — Shuttle Radar Topography Mission 30 m DEM".into(),
        provider: "NASA / NGA".into(),
        observable: Observable {
            name: "elevation".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Geomorphology, ProcessFamily::Hydrology],
            description:
                "C-band InSAR DEM from the 2000 Shuttle mission (void-filled, 1 arc-second)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0,
                south: -56.0,
                east: 180.0,
                north: 60.0,
            },
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: Some(2000),
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(9.0),
            relative: None,
            known_biases: vec![
                "radar shadow and layover in steep terrain".into(),
                "canopy-top surface in forested areas (not bare earth)".into(),
            ],
            quality_flag: Some("NUM".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://e4ftl01.cr.usgs.gov/MEASURES/SRTMGL1.003/".into(),
        }],
        formats: vec![DataFormat::GeoTiff, DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geomorphology, ProcessFamily::Hydrology],
        tags: vec![
            "srtm".into(),
            "dem".into(),
            "elevation".into(),
            "topography".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Fire Emissions
// ---------------------------------------------------------------------------

fn gfed() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GFED4S"),
        name: "GFED4S".into(),
        description: "GFED4s — Global Fire Emissions Database with small fires (1997–present)".into(),
        provider: "Vrije Universiteit / NASA".into(),
        observable: Observable {
            name: "fire_carbon_emissions".into(),
            unit: "g C m-2 month-1".into(),
            families: vec![ProcessFamily::Fire, ProcessFamily::Biogeochemistry],
            description: "Monthly burned area, fire emissions (C, CO₂, CO, CH₄, etc.) at 0.25° from satellite BA + biogeochemical model".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1997, end_year: None },
            spatial_resolution_m: Some(27_750.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.30),
            known_biases: vec![
                "small-fire supplement may double-count with MCD64A1".into(),
                "emission factors vary regionally".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.globalfiredata.org/data.html".into(),
        }],
        formats: vec![DataFormat::Hdf5],
        license: License::CcBy4,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "nrmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Fire, ProcessFamily::Biogeochemistry],
        tags: vec!["gfed".into(), "fire_emissions".into(), "burned_area".into(), "carbon".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Soil Moisture / Passive MW (additional)
// ---------------------------------------------------------------------------

fn esa_cci_soil_moisture() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ESA_CCI_SM"),
        name: "ESA_CCI_SM".into(),
        description:
            "ESA CCI Soil Moisture — multi-decadal merged active/passive microwave (1978–present)"
                .into(),
        provider: "ESA Climate Change Initiative / TU Wien".into(),
        observable: Observable {
            name: "soil_moisture".into(),
            unit: "m3 m-3".into(),
            families: vec![ProcessFamily::Hydrology],
            description:
                "Combined active/passive microwave volumetric soil moisture (0.25°, daily)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1978,
                end_year: None,
            },
            spatial_resolution_m: Some(27_750.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.04),
            relative: None,
            known_biases: vec![
                "frozen-soil gap in boreal winter".into(),
                "signal saturation in dense vegetation (VOD > 0.8)".into(),
            ],
            quality_flag: Some("flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.esa-soilmoisture-cci.org/node/145".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "ubrmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec![
            "esa_cci".into(),
            "soil_moisture".into(),
            "microwave".into(),
            "long_record".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Gravity / Mass Change (additional)
// ---------------------------------------------------------------------------

fn grace_fo_mascons() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GRACE_FO_MASCONS"),
        name: "GRACE_FO_MASCONS".into(),
        description: "GRACE-FO JPL Mascon RL06.1 — monthly mass change continuation (2018–present)".into(),
        provider: "NASA JPL".into(),
        observable: Observable {
            name: "liquid_water_equivalent_thickness".into(),
            unit: "cm".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Cryosphere],
            description: "Monthly 0.5° mascon gravity solutions from GRACE Follow-On laser ranging interferometer".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2018, end_year: None },
            spatial_resolution_m: Some(55_000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.5),
            relative: None,
            known_biases: vec![
                "glacial isostatic adjustment correction uncertainty".into(),
                "leakage between adjacent mascons".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://podaac.jpl.nasa.gov/dataset/TELLUS_GRAC-GRFO_MASCON_CRI_GRID_RL06.1_V3".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 3.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Cryosphere],
        tags: vec!["grace-fo".into(), "gravity".into(), "mass_change".into(), "tws".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Vegetation Optical Depth / Biomass
// ---------------------------------------------------------------------------

fn smos_vod() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SMOS_IC_VOD"),
        name: "SMOS_IC_VOD".into(),
        description: "SMOS-IC L-band Vegetation Optical Depth — above-ground biomass proxy (2010–present)".into(),
        provider: "CESBIO / ESA".into(),
        observable: Observable {
            name: "vegetation_optical_depth".into(),
            unit: "dimensionless".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
            description: "L-band (1.4 GHz) vegetation optical depth sensitive to vegetation water content and biomass".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2010, end_year: None },
            spatial_resolution_m: Some(25_000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.05),
            relative: None,
            known_biases: vec![
                "RFI contamination over Europe and Asia".into(),
                "sensitivity saturation above ~300 Mg/ha biomass".into(),
            ],
            quality_flag: Some("Quality_Flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://ib.remote-sensing.inrae.fr/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "r_squared".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 5.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
        tags: vec!["smos".into(), "vod".into(), "biomass".into(), "vegetation".into(), "l_band".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Solar Radiation
// ---------------------------------------------------------------------------

fn ceres_syn1deg() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CERES_SYN1DEG"),
        name: "CERES_SYN1DEG".into(),
        description: "CERES SYN1deg — hourly 1° surface and TOA radiation fluxes with diurnal cycle".into(),
        provider: "NASA LaRC".into(),
        observable: Observable {
            name: "surface_radiation_flux".into(),
            unit: "W m-2".into(),
            families: vec![ProcessFamily::Radiation],
            description: "Hourly SW/LW surface downwelling and TOA fluxes using CERES + geostationary diurnal correction".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2000, end_year: None },
            spatial_resolution_m: Some(111_000.0),
            cadence: Cadence::Hourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(10.0),
            relative: None,
            known_biases: vec!["surface flux uncertainty larger than TOA (cloud representation)".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://opendap.larc.nasa.gov/opendap/CERES/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 5.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Radiation],
        tags: vec!["ceres".into(), "radiation".into(), "surface_flux".into(), "toa_flux".into(), "diurnal".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Night Lights / Anthropogenic
// ---------------------------------------------------------------------------

fn viirs_dnb() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("VIIRS_DNB_NTL"),
        name: "VIIRS_DNB_NTL".into(),
        description: "VIIRS Day/Night Band — monthly nighttime lights composites (2012–present)".into(),
        provider: "NOAA NCEI / Colorado School of Mines".into(),
        observable: Observable {
            name: "nighttime_radiance".into(),
            unit: "nW cm-2 sr-1".into(),
            families: vec![ProcessFamily::HumanSystems],
            description: "Monthly cloud-free nighttime light composites tracking urbanisation, electrification, and economic activity".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2012, end_year: None },
            spatial_resolution_m: Some(500.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec![
                "stray-light contamination near terminator".into(),
                "biomass burning and gas flares inflate signal".into(),
            ],
            quality_flag: Some("cf_cvg".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://eogdata.mines.edu/nighttime_light/monthly/v10/".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::PublicDomain,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::HumanSystems],
        tags: vec!["viirs".into(), "nighttime_lights".into(), "urbanisation".into(), "energy".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Evapotranspiration / Water Flux
// ---------------------------------------------------------------------------

fn ecostress() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ECOSTRESS"),
        name: "ECOSTRESS".into(),
        description: "NASA ECOSTRESS — ISS-based high-resolution evapotranspiration and LST".into(),
        provider: "NASA JPL".into(),
        observable: Observable {
            name: "evapotranspiration".into(),
            unit: "W m-2".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Ecology],
            description: "Latent heat flux / ET and water stress from ISS thermal-IR (38 × 69 m, diurnal sampling)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -52.0, east: 180.0, north: 52.0,
            },
            temporal: TemporalExtent { start_year: 2018, end_year: None },
            spatial_resolution_m: Some(70.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(30.0),
            relative: None,
            known_biases: vec![
                "ISS orbit precession gives non-repeating overpass times".into(),
                "cloud masking in tropical convective regions".into(),
            ],
            quality_flag: Some("QC".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "ECO3ETPTJPL".into(),
            version: "002".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 2.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Ecology],
        tags: vec!["ecostress".into(), "evapotranspiration".into(), "water_stress".into(), "thermal".into(), "iss".into()],
    }
}

fn gleam() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GLEAM_V3"),
        name: "GLEAM_V3".into(),
        description: "GLEAM v3 — Global Land Evaporation Amsterdam Model (1980–present)".into(),
        provider: "Vrije Universiteit Amsterdam / Ghent University".into(),
        observable: Observable {
            name: "evapotranspiration".into(),
            unit: "mm day-1".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Ecology],
            description:
                "Satellite-based ET, root-zone soil moisture, and evaporative stress (0.25°, daily)"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1980,
                end_year: None,
            },
            spatial_resolution_m: Some(27_750.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.20),
            known_biases: vec![
                "interception loss may be overestimated in deciduous forests".into(),
                "forcing data quality degrades before satellite era".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.gleam.eu/#downloads".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Ecology],
        tags: vec![
            "gleam".into(),
            "evapotranspiration".into(),
            "soil_moisture".into(),
            "long_record".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Harmonised / Fused Products
// ---------------------------------------------------------------------------

fn hls() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("HLS_V2"),
        name: "HLS_V2".into(),
        description: "NASA Harmonized Landsat Sentinel-2 (HLS) — 30 m fused surface reflectance".into(),
        provider: "NASA GSFC".into(),
        observable: Observable {
            name: "surface_reflectance".into(),
            unit: "dimensionless".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Hydrology, ProcessFamily::HumanSystems],
            description: "Harmonised BRDF-normalised reflectance from Landsat 8/9 and Sentinel-2A/B (2–4 day revisit)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2013, end_year: None },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.04),
            known_biases: vec!["residual BRDF normalisation error at high view angles".into()],
            quality_flag: Some("Fmask".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "HLSS30".into(),
            version: "002".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::UsGov,
        latency: LatencyClass::Rapid,
        scoring: ScoringProtocol {
            primary_metric: "spectral_rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Hydrology, ProcessFamily::HumanSystems],
        tags: vec!["hls".into(), "harmonized".into(), "landsat".into(), "sentinel-2".into(), "reflectance".into()],
    }
}

fn dynamic_world() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("DYNAMIC_WORLD"),
        name: "DYNAMIC_WORLD".into(),
        description: "Google Dynamic World — near-real-time 10 m land cover from Sentinel-2".into(),
        provider: "Google / World Resources Institute".into(),
        observable: Observable {
            name: "land_cover_probability".into(),
            unit: "probability".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::HumanSystems],
            description: "Per-pixel 9-class land cover probabilities from deep learning on every Sentinel-2 image".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2015, end_year: None },
            spatial_resolution_m: Some(10.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.12),
            known_biases: vec![
                "temporal noise in class assignments between revisits".into(),
                "cloud-shadow misclassification in built-up areas".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://earthengine.google.com/".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "overall_accuracy".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::HumanSystems],
        tags: vec!["dynamic_world".into(), "land_cover".into(), "deep_learning".into(), "nrt".into(), "sentinel-2".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — New-Generation Missions (PACE, EMIT, NISAR)
// ---------------------------------------------------------------------------

fn pace_oci() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("PACE_OCI"),
        name: "PACE_OCI".into(),
        description: "NASA PACE OCI — hyperspectral ocean colour and aerosol (2024–present)".into(),
        provider: "NASA GSFC".into(),
        observable: Observable {
            name: "ocean_color_hyperspectral".into(),
            unit: "mW cm-2 um-1 sr-1".into(),
            families: vec![ProcessFamily::Ocean, ProcessFamily::Atmosphere],
            description: "Hyper-spectral (340–890 nm, 5 nm) ocean colour + multi-angle polarimetry for aerosol/cloud".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2024, end_year: None },
            spatial_resolution_m: Some(1000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec!["stray light from adjacent bright targets (land–ocean boundary)".into()],
            quality_flag: Some("l2_flags".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "PACE_OCI_L2_AOP".into(),
            version: "2.0".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 1.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean, ProcessFamily::Atmosphere],
        tags: vec!["pace".into(), "ocean_color".into(), "hyperspectral".into(), "aerosol".into(), "polarimetry".into()],
    }
}

fn emit() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("EMIT"),
        name: "EMIT".into(),
        description: "NASA EMIT — ISS imaging spectrometer for surface mineral dust and methane".into(),
        provider: "NASA JPL".into(),
        observable: Observable {
            name: "surface_mineralogy".into(),
            unit: "dimensionless".into(),
            families: vec![ProcessFamily::Geomorphology, ProcessFamily::Atmosphere],
            description: "VSWIR imaging spectrometer (380–2500 nm, 7.4 nm res, 60 m) mapping surface mineral composition and CH₄/CO₂ point sources".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -52.0, east: 180.0, north: 52.0,
            },
            temporal: TemporalExtent { start_year: 2022, end_year: None },
            spatial_resolution_m: Some(60.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.05),
            known_biases: vec![
                "vegetation spectral mixing with exposed mineral surfaces".into(),
                "ISS orbit gives non-uniform revisit frequency".into(),
            ],
            quality_flag: Some("quality_flag".into()),
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "EMITL2ARFL".into(),
            version: "001".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "spectral_angle".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 1.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geomorphology, ProcessFamily::Atmosphere],
        tags: vec!["emit".into(), "mineral_dust".into(), "imaging_spectrometer".into(), "methane".into(), "iss".into()],
    }
}

fn nisar() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("NISAR"),
        name: "NISAR".into(),
        description: "NASA-ISRO NISAR — L/S-band SAR for deformation, biomass, and cryosphere".into(),
        provider: "NASA JPL / ISRO".into(),
        observable: Observable {
            name: "surface_deformation".into(),
            unit: "mm yr-1".into(),
            families: vec![ProcessFamily::Geology, ProcessFamily::Cryosphere, ProcessFamily::Ecology],
            description: "Dual-frequency (L+S band) SAR interferometry at 3–10 m for crustal deformation, ice velocity, and biomass".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2025, end_year: None },
            spatial_resolution_m: Some(10.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Swath,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: None,
            known_biases: vec![
                "ionospheric phase delay at L-band in equatorial regions".into(),
                "tropospheric water vapour artefacts in interferograms".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::NasaCmr {
            short_name: "NISAR_L2_GUNW".into(),
            version: "1.0".into(),
        }],
        formats: vec![DataFormat::GeoTiff, DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 1.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geology, ProcessFamily::Cryosphere, ProcessFamily::Ecology],
        tags: vec!["nisar".into(), "sar".into(), "insar".into(), "deformation".into(), "biomass".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Forest / Biomass Change
// ---------------------------------------------------------------------------

fn hansen_gfc() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("HANSEN_GFC"),
        name: "HANSEN_GFC".into(),
        description:
            "Hansen Global Forest Change — annual 30 m tree cover loss/gain (2000–present)".into(),
        provider: "University of Maryland / Google".into(),
        observable: Observable {
            name: "tree_cover_change".into(),
            unit: "fraction / year".into(),
            families: vec![
                ProcessFamily::Ecology,
                ProcessFamily::Biogeochemistry,
                ProcessFamily::HumanSystems,
            ],
            description: "Annual tree cover loss, gain, and year-of-loss from Landsat time series"
                .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: None,
            },
            spatial_resolution_m: Some(30.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec![
                "commission errors in boreal fire-scars".into(),
                "does not distinguish plantation from natural forest".into(),
            ],
            quality_flag: Some("datamask".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern:
                "https://storage.googleapis.com/earthenginepartners-hansen/GFC-2023-v1.11/".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "accuracy".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![
            ProcessFamily::Ecology,
            ProcessFamily::Biogeochemistry,
            ProcessFamily::HumanSystems,
        ],
        tags: vec![
            "hansen".into(),
            "forest_change".into(),
            "tree_cover".into(),
            "deforestation".into(),
            "global".into(),
        ],
    }
}

fn glad_alerts() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GLAD_ALERTS"),
        name: "GLAD_ALERTS".into(),
        description: "GLAD Forest Disturbance Alerts — weekly Landsat/Sentinel-2 deforestation alerts".into(),
        provider: "University of Maryland / GLAD".into(),
        observable: Observable {
            name: "forest_disturbance".into(),
            unit: "binary (date)".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::HumanSystems],
            description: "Near-real-time forest disturbance alerts (confirmed + provisional) from Landsat and Sentinel-2".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -30.0, east: 180.0, north: 30.0,
            },
            temporal: TemporalExtent { start_year: 2018, end_year: None },
            spatial_resolution_m: Some(10.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec!["false alerts in persistent cloud regions (equatorial Africa)".into()],
            quality_flag: Some("confidence".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://glad.umd.edu/dataset/glad-forest-alerts".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "precision_recall".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 2.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::HumanSystems],
        tags: vec!["glad".into(), "deforestation".into(), "alert".into(), "nrt".into(), "tropics".into()],
    }
}

fn esa_cci_biomass() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ESA_CCI_BIOMASS"),
        name: "ESA_CCI_BIOMASS".into(),
        description: "ESA CCI Above-Ground Biomass — global 100 m maps (2010, 2017–2020)".into(),
        provider: "ESA Climate Change Initiative".into(),
        observable: Observable {
            name: "above_ground_biomass".into(),
            unit: "Mg ha-1".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
            description: "Above-ground biomass density from merged SAR (Sentinel-1, ALOS PALSAR-2) and spaceborne lidar".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2010, end_year: Some(2020) },
            spatial_resolution_m: Some(100.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.30),
            known_biases: vec![
                "SAR backscatter saturation above ~150 Mg/ha".into(),
                "sparse lidar calibration in central Africa".into(),
            ],
            quality_flag: Some("quality_flag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://catalogue.ceda.ac.uk/uuid/af60720c1e404a9e9d2c145d2b2ead4e".into(),
        }],
        formats: vec![DataFormat::GeoTiff, DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 1.0,
            spinup_discard_years: 10.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Biogeochemistry],
        tags: vec!["esa_cci".into(), "biomass".into(), "agb".into(), "forest".into(), "global".into()],
    }
}

fn global_mangrove_watch() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GMW"),
        name: "GMW".into(),
        description: "Global Mangrove Watch — annual mangrove extent from ALOS PALSAR + Landsat (1996–present)".into(),
        provider: "JAXA / Aberystwyth University / solo Earth Observation".into(),
        observable: Observable {
            name: "mangrove_extent".into(),
            unit: "binary".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Ocean],
            description: "Annual mangrove forest extent mapping from L-band SAR and optical multi-sensor fusion".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -40.0, east: 180.0, north: 33.0,
            },
            temporal: TemporalExtent { start_year: 1996, end_year: None },
            spatial_resolution_m: Some(25.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.08),
            known_biases: vec!["exclusion of small isolated mangrove patches (<0.5 ha)".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.globalmangrovewatch.org/datasets".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "accuracy".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Ocean],
        tags: vec!["mangrove".into(), "coastal".into(), "sar".into(), "blue_carbon".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — SAR (additional)
// ---------------------------------------------------------------------------

fn alos_palsar() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ALOS_PALSAR_MOSAIC"),
        name: "ALOS_PALSAR_MOSAIC".into(),
        description: "JAXA ALOS PALSAR/PALSAR-2 — annual L-band SAR mosaic (25 m, 2007–present)"
            .into(),
        provider: "JAXA EORC".into(),
        observable: Observable {
            name: "l_band_backscatter".into(),
            unit: "dB".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Cryosphere],
            description:
                "L-band HH/HV polarisation SAR mosaics sensitive to forest structure and biomass"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2007,
                end_year: None,
            },
            spatial_resolution_m: Some(25.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.8),
            relative: None,
            known_biases: vec![
                "ortho-rectification artefacts in mountainous terrain".into(),
                "moisture-driven backscatter variability within mosaic windows".into(),
            ],
            quality_flag: Some("mask".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.eorc.jaxa.jp/ALOS/en/dataset/fnf_e.htm".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::OpenData,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 3.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Cryosphere],
        tags: vec![
            "alos".into(),
            "palsar".into(),
            "l_band".into(),
            "sar".into(),
            "biomass".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Precipitation (additional)
// ---------------------------------------------------------------------------

fn trmm() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("TRMM_3B42"),
        name: "TRMM_3B42".into(),
        description: "NASA TRMM 3B42 — tropical 3-hourly merged precipitation (1998–2019)".into(),
        provider: "NASA GES DISC".into(),
        observable: Observable {
            name: "precipitation_rate".into(),
            unit: "mm hr-1".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere],
            description: "Multi-satellite merged precipitation from TRMM Precipitation Radar + TMI + IR (0.25°)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -50.0, east: 180.0, north: 50.0,
            },
            temporal: TemporalExtent { start_year: 1998, end_year: Some(2019) },
            spatial_resolution_m: Some(27_750.0),
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.25),
            known_biases: vec![
                "underestimate of orographic precipitation over the Andes".into(),
                "warm-rain bias in shallow convective systems".into(),
            ],
            quality_flag: Some("HQprecipQualityIndex".into()),
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://disc2.gesdisc.eosdis.nasa.gov/opendap/TRMM_L3/".into(),
        }],
        formats: vec![DataFormat::NetCdf, DataFormat::Hdf5],
        license: License::UsGov,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "correlation".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 10.0,
            spinup_discard_years: 1.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere],
        tags: vec!["trmm".into(), "precipitation".into(), "tropical".into(), "radar".into(), "archive".into()],
    }
}

fn persiann_cdr() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("PERSIANN_CDR"),
        name: "PERSIANN_CDR".into(),
        description: "PERSIANN-CDR — daily 0.25° precipitation climate data record (1983–present)".into(),
        provider: "UC Irvine CHRS".into(),
        observable: Observable {
            name: "precipitation".into(),
            unit: "mm day-1".into(),
            families: vec![ProcessFamily::Hydrology],
            description: "Neural-network based IR precipitation estimates bias-corrected with GPCP (60°S–60°N)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: -60.0, east: 180.0, north: 60.0,
            },
            temporal: TemporalExtent { start_year: 1983, end_year: None },
            spatial_resolution_m: Some(27_750.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.25),
            known_biases: vec!["cold-season precipitation underestimate at high latitudes".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://chrsdata.eng.uci.edu/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::PublicDomain,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec!["persiann".into(), "precipitation".into(), "cdr".into(), "neural_network".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Fire (additional)
// ---------------------------------------------------------------------------

fn firms() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("FIRMS"),
        name: "FIRMS".into(),
        description: "NASA FIRMS — global NRT active fire detections (MODIS + VIIRS combined)".into(),
        provider: "NASA LANCE".into(),
        observable: Observable {
            name: "active_fire".into(),
            unit: "fire radiative power (MW)".into(),
            families: vec![ProcessFamily::Fire],
            description: "Combined MODIS (1 km) and VIIRS (375 m) active fire detections within 3 hours of overpass".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2000, end_year: None },
            spatial_resolution_m: Some(375.0),
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::PointNetwork,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "sub-pixel false alarms from industrial heat sources".into(),
                "cloud obscuration leads to omission errors".into(),
            ],
            quality_flag: Some("confidence".into()),
        },
        access: vec![AccessMethod::Api {
            base_url: "https://firms.modaps.eosdis.nasa.gov/api/".into(),
        }],
        formats: vec![DataFormat::Csv, DataFormat::GeoJson],
        license: License::UsGov,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "detection_efficiency".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Fire],
        tags: vec!["firms".into(), "active_fire".into(), "nrt".into(), "frp".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Reanalysis & Climate Grids (additional)
// ---------------------------------------------------------------------------

fn era5_land() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ERA5_LAND"),
        name: "ERA5_LAND".into(),
        description: "ECMWF ERA5-Land — enhanced-resolution land surface reanalysis (1950–present)".into(),
        provider: "ECMWF / Copernicus C3S".into(),
        observable: Observable {
            name: "land_surface_met".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere, ProcessFamily::Cryosphere],
            description: "Hourly 0.1° T2m, precipitation, soil temperature/moisture, snow, runoff from HTESSEL land model".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1950, end_year: None },
            spatial_resolution_m: Some(11_000.0),
            cadence: Cadence::Hourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "cold bias in skin temperature over snow-covered regions".into(),
                "precipitation over-estimation in mountainous areas".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://cds.climate.copernicus.eu/api/v2".into(),
        }],
        formats: vec![DataFormat::NetCdf, DataFormat::Grib2],
        license: License::CcBy4,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere, ProcessFamily::Cryosphere],
        tags: vec!["era5-land".into(), "reanalysis".into(), "high_resolution".into(), "land_surface".into(), "global".into()],
    }
}

fn cru_ts() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CRU_TS4"),
        name: "CRU_TS4".into(),
        description: "CRU TS v4 — monthly 0.5° station-interpolated climate grids (1901–present)".into(),
        provider: "University of East Anglia CRU".into(),
        observable: Observable {
            name: "surface_climate".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Hydrology],
            description: "Monthly mean temperature, precipitation, vapour pressure, cloud cover, and wet days from station interpolation".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1901, end_year: None },
            spatial_resolution_m: Some(55_000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec![
                "sparse station coverage before 1950 in tropics and southern hemisphere".into(),
                "urban heat island not removed from temperature".into(),
            ],
            quality_flag: Some("stn".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://crudata.uea.ac.uk/cru/data/hrg/cru_ts_4.07/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 20.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Hydrology],
        tags: vec!["cru".into(), "temperature".into(), "precipitation".into(), "station_based".into(), "long_record".into()],
    }
}

fn terraclimate() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("TERRACLIMATE"),
        name: "TERRACLIMATE".into(),
        description: "TerraClimate — monthly 4 km climate and water balance (1958–present)".into(),
        provider: "University of Idaho / Climatology Lab".into(),
        observable: Observable {
            name: "climate_water_balance".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere, ProcessFamily::Ecology],
            description: "Monthly 1/24° temperature, precipitation, PET, AET, soil moisture, snow, VPD, and climatic water deficit".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1958, end_year: None },
            spatial_resolution_m: Some(4_000.0),
            cadence: Cadence::Monthly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec!["water balance closure forced by simple bucket model".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::Opendap {
            endpoint: "https://climate.northwestknowledge.net/TERRACLIMATE-DATA/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::PublicDomain,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Atmosphere, ProcessFamily::Ecology],
        tags: vec!["terraclimate".into(), "water_balance".into(), "vpd".into(), "climate".into(), "global".into()],
    }
}

fn worldclim() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("WORLDCLIM_V2"),
        name: "WORLDCLIM_V2".into(),
        description: "WorldClim v2.1 — 1 km bioclimatic variables and monthly climatologies (1970–2000)".into(),
        provider: "WorldClim.org".into(),
        observable: Observable {
            name: "bioclimatic_variables".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Ecology, ProcessFamily::Atmosphere],
            description: "19 bioclimatic variables + monthly Tmin/Tmax/precip from station interpolation with covariates".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1970, end_year: Some(2000) },
            spatial_resolution_m: Some(1000.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec![
                "interpolation artefacts in data-sparse mountainous regions".into(),
                "no diurnal or seasonal variability captured".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://www.worldclim.org/data/worldclim21.html".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 10.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Ecology, ProcessFamily::Atmosphere],
        tags: vec!["worldclim".into(), "bioclim".into(), "climatology".into(), "species_distribution".into(), "global".into()],
    }
}

fn cams_global() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CAMS_GLOBAL"),
        name: "CAMS_GLOBAL".into(),
        description: "Copernicus Atmosphere Monitoring Service — global atmospheric composition reanalysis".into(),
        provider: "ECMWF / Copernicus".into(),
        observable: Observable {
            name: "atmospheric_composition".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Atmosphere, ProcessFamily::Biogeochemistry],
            description: "3D fields of O₃, CO, NO₂, SO₂, aerosols, GHGs from data assimilation (0.4°, 3-hourly)".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2003, end_year: None },
            spatial_resolution_m: Some(44_000.0),
            cadence: Cadence::SubHourly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "biomass burning emission injection height uncertainty".into(),
                "limited assimilation of surface in-situ CO₂".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://ads.atmosphere.copernicus.eu/api/v2".into(),
        }],
        formats: vec![DataFormat::NetCdf, DataFormat::Grib2],
        license: License::CcBy4,
        latency: LatencyClass::Nrt,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 5.0,
            spinup_discard_years: 2.0,
        },
        evaluates_families: vec![ProcessFamily::Atmosphere, ProcessFamily::Biogeochemistry],
        tags: vec!["cams".into(), "atmospheric_composition".into(), "aerosol".into(), "reanalysis".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Ocean (additional)
// ---------------------------------------------------------------------------

fn esa_cci_sst() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ESA_CCI_SST"),
        name: "ESA_CCI_SST".into(),
        description:
            "ESA CCI SST — multi-decadal climate-quality sea surface temperature (1981–present)"
                .into(),
        provider: "ESA Climate Change Initiative / U. Reading".into(),
        observable: Observable {
            name: "sea_surface_temperature".into(),
            unit: "K".into(),
            families: vec![ProcessFamily::Ocean],
            description:
                "Daily 0.05° SST from merged AVHRR + (A)ATSR with strict inter-sensor harmonisation"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1981,
                end_year: None,
            },
            spatial_resolution_m: Some(5500.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.2),
            relative: None,
            known_biases: vec![
                "residual inter-sensor calibration offset between AVHRR generations".into(),
                "depth mismatch: satellite skin vs bulk SST".into(),
            ],
            quality_flag: Some("quality_level".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://catalogue.ceda.ac.uk/uuid/62c0f97b1eac4e0197a674870afe1ee6"
                .into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 15.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean],
        tags: vec![
            "esa_cci".into(),
            "sst".into(),
            "ocean_temperature".into(),
            "climate_record".into(),
            "global".into(),
        ],
    }
}

fn copernicus_marine() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CMEMS_GLOBAL_PHYS"),
        name: "CMEMS_GLOBAL_PHYS".into(),
        description: "Copernicus Marine GLORYS12 — global 1/12° ocean reanalysis (1993–present)".into(),
        provider: "Mercator Ocean / Copernicus".into(),
        observable: Observable {
            name: "ocean_state".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Ocean],
            description: "Daily/monthly 3D ocean temperature, salinity, currents, SSH, and mixed-layer depth at 1/12°".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1993, end_year: None },
            spatial_resolution_m: Some(9_000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.10),
            known_biases: vec![
                "mesoscale eddy positions may be misplaced by O(50 km)".into(),
                "Arctic coverage limited by sparse Argo float sampling".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://data.marine.copernicus.eu/api/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 3.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean],
        tags: vec!["copernicus_marine".into(), "glorys".into(), "ocean_reanalysis".into(), "3d_ocean".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Gridded Soil / Subsurface
// ---------------------------------------------------------------------------

fn soilgrids() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("SOILGRIDS250M"),
        name: "SOILGRIDS250M".into(),
        description: "SoilGrids 250 m — global machine-learning predicted soil properties".into(),
        provider: "ISRIC World Soil Information".into(),
        observable: Observable {
            name: "soil_properties".into(),
            unit: "various".into(),
            families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Hydrology, ProcessFamily::Geology],
            description: "Predicted SOC, sand/silt/clay, pH, bulk density, CEC at 6 depths (0–200 cm) from 250k+ profiles".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2017, end_year: Some(2017) },
            spatial_resolution_m: Some(250.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.25),
            known_biases: vec![
                "prediction intervals widen in under-sampled tropical soils".into(),
                "SOC underestimate in peatlands".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://rest.isric.org/soilgrids/v2.0/".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Hydrology, ProcessFamily::Geology],
        tags: vec!["soilgrids".into(), "soil".into(), "carbon".into(), "texture".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Population / Socioeconomic
// ---------------------------------------------------------------------------

fn worldpop() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("WORLDPOP"),
        name: "WORLDPOP".into(),
        description: "WorldPop — annual 100 m gridded population estimates (2000–2020)".into(),
        provider: "University of Southampton / WorldPop".into(),
        observable: Observable {
            name: "population_density".into(),
            unit: "people per pixel".into(),
            families: vec![ProcessFamily::HumanSystems],
            description: "Dasymetric population mapping using census, building footprints, and satellite covariates".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2000, end_year: Some(2020) },
            spatial_resolution_m: Some(100.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.15),
            known_biases: vec![
                "census boundary misalignment causes edge artefacts".into(),
                "nomadic/mobile populations under-represented".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://hub.worldpop.org/geodata/listing?id=29".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "mae".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::HumanSystems],
        tags: vec!["worldpop".into(), "population".into(), "demographics".into(), "gridded".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Flood / Disaster
// ---------------------------------------------------------------------------

fn global_flood_database() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GFD"),
        name: "GFD".into(),
        description: "Global Flood Database — MODIS-derived flood extent maps (2000–2018)".into(),
        provider: "Cloud to Street / Columbia University".into(),
        observable: Observable {
            name: "flood_extent".into(),
            unit: "binary".into(),
            families: vec![ProcessFamily::Hydrology],
            description:
                "Event-level inundation maps for 913 large floods derived from MODIS NRT data"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 2000,
                end_year: Some(2018),
            },
            spatial_resolution_m: Some(250.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.20),
            known_biases: vec![
                "cloud contamination limits coverage during peak flood events".into(),
                "250 m resolution misses small urban floods".into(),
            ],
            quality_flag: Some("flooded".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://global-flood-database.cloudtostreet.ai/".into(),
        }],
        formats: vec![DataFormat::GeoTiff],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "critical_success_index".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 5.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology],
        tags: vec![
            "flood".into(),
            "inundation".into(),
            "disaster".into(),
            "modis".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Gravity / Geoid
// ---------------------------------------------------------------------------

fn eigen6c4() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("EIGEN6C4"),
        name: "EIGEN6C4".into(),
        description: "EIGEN-6C4 — combined static gravity field model to degree/order 2190".into(),
        provider: "GFZ / GRGS".into(),
        observable: Observable {
            name: "gravity_geoid".into(),
            unit: "mGal / m".into(),
            families: vec![ProcessFamily::Geology, ProcessFamily::Geomorphology],
            description: "Ultra-high resolution static gravity anomalies and geoid undulations from GOCE + GRACE + satellite altimetry + surface data".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2014, end_year: Some(2014) },
            spatial_resolution_m: Some(9000.0),
            cadence: Cadence::Irregular,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(1.0),
            relative: None,
            known_biases: vec!["surface gravity data gaps in Antarctica and ocean trenches".into()],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "http://icgem.gfz-potsdam.de/tom_longtime".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::OpenData,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::SpatialAverage,
            min_overlap_years: 0.0,
            spinup_discard_years: 0.0,
        },
        evaluates_families: vec![ProcessFamily::Geology, ProcessFamily::Geomorphology],
        tags: vec!["gravity".into(), "geoid".into(), "goce".into(), "static_field".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Permafrost
// ---------------------------------------------------------------------------

fn esa_cci_permafrost() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ESA_CCI_PERMAFROST"),
        name: "ESA_CCI_PERMAFROST".into(),
        description: "ESA CCI Permafrost — global 1 km ground temperature and active layer thickness (1997–present)".into(),
        provider: "ESA CCI / AWI / b.geos".into(),
        observable: Observable {
            name: "permafrost_state".into(),
            unit: "°C / m".into(),
            families: vec![ProcessFamily::Cryosphere, ProcessFamily::Geology],
            description: "Annual mean ground temperature (MAGT) and active layer thickness (ALT) from satellite LST + modelling".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox {
                west: -180.0, south: 30.0, east: 180.0, north: 90.0,
            },
            temporal: TemporalExtent { start_year: 1997, end_year: None },
            spatial_resolution_m: Some(1000.0),
            cadence: Cadence::Annual,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: Some(2.0),
            relative: None,
            known_biases: vec![
                "snow insulation effect poorly resolved in sparse weather data".into(),
                "sub-grid organic layer thickness unknown".into(),
            ],
            quality_flag: Some("qflag".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://catalogue.ceda.ac.uk/uuid/1f88068e86184b9e8de960ac1b57ad42".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Cryosphere, ProcessFamily::Geology],
        tags: vec!["permafrost".into(), "ground_temperature".into(), "active_layer".into(), "esa_cci".into(), "arctic".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Ocean Biogeochemistry
// ---------------------------------------------------------------------------

fn esa_cci_ocean_colour() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("ESA_CCI_OC"),
        name: "ESA_CCI_OC".into(),
        description: "ESA CCI Ocean Colour — multi-sensor merged chlorophyll-a CDR (1997–present)".into(),
        provider: "ESA CCI / Plymouth Marine Laboratory".into(),
        observable: Observable {
            name: "chlorophyll_a".into(),
            unit: "mg m-3".into(),
            families: vec![ProcessFamily::Ocean, ProcessFamily::Biogeochemistry],
            description: "Daily/monthly 4 km Chl-a from merged SeaWiFS + MODIS + MERIS + VIIRS + OLCI with bias correction".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 1997, end_year: None },
            spatial_resolution_m: Some(4000.0),
            cadence: Cadence::Daily,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.30),
            known_biases: vec![
                "per-pixel bias uncertainty provided in product".into(),
                "coccolithophore blooms cause anomalous retrievals".into(),
            ],
            quality_flag: Some("flags".into()),
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://catalogue.ceda.ac.uk/uuid/9c334fbe6d424a708cf3c4cf0c6a53f5".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::CcBy4,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "log_rmse".into(),
            extraction: ExtractionMethod::TemporalAlignment,
            min_overlap_years: 10.0,
            spinup_discard_years: 5.0,
        },
        evaluates_families: vec![ProcessFamily::Ocean, ProcessFamily::Biogeochemistry],
        tags: vec!["esa_cci".into(), "ocean_colour".into(), "chlorophyll".into(), "climate_record".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Carbon Flux Inversion
// ---------------------------------------------------------------------------

fn carbontracker() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("CARBONTRACKER_CT2022"),
        name: "CARBONTRACKER_CT2022".into(),
        description: "NOAA CarbonTracker CT2022 — global CO₂ flux inversion (2000–present)".into(),
        provider: "NOAA GML".into(),
        observable: Observable {
            name: "co2_surface_flux".into(),
            unit: "g C m-2 day-1".into(),
            families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere],
            description: "Optimised biosphere + ocean + fossil fuel CO₂ fluxes from atmospheric inversion at 1° × 1°".into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent { start_year: 2000, end_year: None },
            spatial_resolution_m: Some(111_000.0),
            cadence: Cadence::Weekly,
            topology: SpatialTopology::Gridded,
            station_count: None,
        },
        uncertainty: UncertaintySpec {
            absolute: None,
            relative: Some(0.40),
            known_biases: vec![
                "transport model error dominates regional flux uncertainty".into(),
                "ocean flux constrained primarily by pCO₂ observations".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::HttpDownload {
            url_pattern: "https://gml.noaa.gov/ccgg/carbontracker/CT2022/".into(),
        }],
        formats: vec![DataFormat::NetCdf],
        license: License::UsGov,
        latency: LatencyClass::Standard,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::AreaWeighted,
            min_overlap_years: 5.0,
            spinup_discard_years: 3.0,
        },
        evaluates_families: vec![ProcessFamily::Biogeochemistry, ProcessFamily::Atmosphere],
        tags: vec!["carbontracker".into(), "co2_flux".into(), "inversion".into(), "carbon_cycle".into(), "global".into()],
    }
}

// ---------------------------------------------------------------------------
// Remote Sensing — Subsurface / Groundwater
// ---------------------------------------------------------------------------

fn global_groundwater() -> ObservationDataset {
    ObservationDataset {
        id: ObservationId::from_name("GGIS"),
        name: "GGIS".into(),
        description: "IGRAC Global Groundwater Information System — well & aquifer observations"
            .into(),
        provider: "IGRAC / UNESCO".into(),
        observable: Observable {
            name: "groundwater_level".into(),
            unit: "m".into(),
            families: vec![ProcessFamily::Hydrology, ProcessFamily::Geology],
            description:
                "In-situ groundwater level, quality, and aquifer geometry from global well networks"
                    .into(),
        },
        coverage: SpatiotemporalCoverage {
            bbox: BoundingBox::GLOBAL,
            temporal: TemporalExtent {
                start_year: 1950,
                end_year: None,
            },
            spatial_resolution_m: None,
            cadence: Cadence::Irregular,
            topology: SpatialTopology::PointNetwork,
            station_count: Some(10_000),
        },
        uncertainty: UncertaintySpec {
            absolute: Some(0.5),
            relative: None,
            known_biases: vec![
                "sampling heavily biased towards developed countries".into(),
                "well datum inconsistencies between national databases".into(),
            ],
            quality_flag: None,
        },
        access: vec![AccessMethod::Api {
            base_url: "https://ggis.un-igrac.org/api/v1/".into(),
        }],
        formats: vec![DataFormat::Csv, DataFormat::GeoJson],
        license: License::OpenData,
        latency: LatencyClass::Archive,
        scoring: ScoringProtocol {
            primary_metric: "rmse".into(),
            extraction: ExtractionMethod::PointExtraction,
            min_overlap_years: 5.0,
            spinup_discard_years: 10.0,
        },
        evaluates_families: vec![ProcessFamily::Hydrology, ProcessFamily::Geology],
        tags: vec![
            "groundwater".into(),
            "wells".into(),
            "aquifer".into(),
            "in_situ".into(),
            "global".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seed_observation_count() {
        let ds = generate_seed_observations();
        assert!(ds.len() >= 30, "Expected ≥30 datasets, got {}", ds.len());
    }

    #[test]
    fn test_all_families_have_observations() {
        use std::collections::HashSet;
        let ds = generate_seed_observations();
        let families: HashSet<_> = ds.iter().flat_map(|d| &d.evaluates_families).collect();
        // At least 10 of 13 families should have observation coverage
        assert!(
            families.len() >= 10,
            "Expected ≥10 families covered, got {}",
            families.len()
        );
    }

    #[test]
    fn test_unique_ids() {
        let ds = generate_seed_observations();
        let ids: Vec<_> = ds.iter().map(|d| d.id).collect();
        let unique: std::collections::HashSet<_> = ids.iter().collect();
        assert_eq!(
            ids.len(),
            unique.len(),
            "Duplicate observation IDs detected"
        );
    }

    #[test]
    fn test_registry_with_seeds() {
        let mut reg = ObservationRegistry::new();
        reg.register_all(generate_seed_observations());
        let summary = reg.summary();
        assert!(summary.total_datasets >= 30);
        assert!(summary.unique_observables >= 15);
        assert!(summary.families_covered >= 10);
    }
}
