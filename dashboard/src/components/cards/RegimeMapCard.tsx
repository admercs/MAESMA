"use client";

import { useEffect, useRef } from "react";

export function RegimeMapCard() {
  const mapContainer = useRef<HTMLDivElement>(null);
  const mapRef = useRef<any>(null);

  useEffect(() => {
    if (!mapContainer.current || mapRef.current) return;

    // Dynamic import to avoid SSR issues
    import("maplibre-gl").then((maplibregl) => {
      const map = new maplibregl.Map({
        container: mapContainer.current!,
        style: {
          version: 8,
          sources: {
            osm: {
              type: "raster",
              tiles: [
                "https://tile.openstreetmap.org/{z}/{x}/{y}.png",
              ],
              tileSize: 256,
              attribution: "&copy; OpenStreetMap contributors",
            },
          },
          layers: [
            {
              id: "osm-tiles",
              type: "raster",
              source: "osm",
              minzoom: 0,
              maxzoom: 19,
            },
          ],
        },
        center: [0, 20],
        zoom: 1.5,
      });

      map.on("load", () => {
        // Add demo regime polygons as a GeoJSON source
        map.addSource("regimes", {
          type: "geojson",
          data: {
            type: "FeatureCollection",
            features: [
              {
                type: "Feature",
                properties: { regime: "boreal_forest", fire_prone: true },
                geometry: {
                  type: "Polygon",
                  coordinates: [[[-140, 50], [-60, 50], [-60, 65], [-140, 65], [-140, 50]]],
                },
              },
              {
                type: "Feature",
                properties: { regime: "tropical_forest", fire_prone: false },
                geometry: {
                  type: "Polygon",
                  coordinates: [[[-80, -15], [-35, -15], [-35, 5], [-80, 5], [-80, -15]]],
                },
              },
              {
                type: "Feature",
                properties: { regime: "savanna", fire_prone: true },
                geometry: {
                  type: "Polygon",
                  coordinates: [[[10, -10], [40, -10], [40, 10], [10, 10], [10, -10]]],
                },
              },
              {
                type: "Feature",
                properties: { regime: "tundra", fire_prone: false },
                geometry: {
                  type: "Polygon",
                  coordinates: [[[60, 65], [180, 65], [180, 75], [60, 75], [60, 65]]],
                },
              },
            ],
          },
        });

        map.addLayer({
          id: "regime-fill",
          type: "fill",
          source: "regimes",
          paint: {
            "fill-color": [
              "match",
              ["get", "regime"],
              "boreal_forest", "#2d6a4f",
              "tropical_forest", "#1b4332",
              "savanna", "#dda15e",
              "tundra", "#a8dadc",
              "#6b7280",
            ],
            "fill-opacity": 0.4,
          },
        });

        map.addLayer({
          id: "regime-outline",
          type: "line",
          source: "regimes",
          paint: {
            "line-color": "#e2e8f0",
            "line-width": 1,
          },
        });

        // Fire-prone indicators
        map.addLayer({
          id: "fire-prone",
          type: "fill",
          source: "regimes",
          filter: ["==", ["get", "fire_prone"], true],
          paint: {
            "fill-pattern": undefined as any,
            "fill-color": "#ef4444",
            "fill-opacity": 0.15,
          },
        });
      });

      mapRef.current = map;
    });

    return () => {
      mapRef.current?.remove();
      mapRef.current = null;
    };
  }, []);

  return (
    <div className="card">
      <h3
        className="text-sm font-semibold mb-4 uppercase tracking-wider"
        style={{ color: "var(--text-secondary)" }}
      >
        Regime Map
      </h3>
      <div
        ref={mapContainer}
        style={{ height: 400, borderRadius: "0.5rem", overflow: "hidden" }}
      />
      <div className="flex gap-4 mt-3 text-xs" style={{ color: "var(--text-secondary)" }}>
        <span className="flex items-center gap-1">
          <span className="w-3 h-3 rounded" style={{ background: "#2d6a4f" }} />
          Boreal Forest
        </span>
        <span className="flex items-center gap-1">
          <span className="w-3 h-3 rounded" style={{ background: "#1b4332" }} />
          Tropical Forest
        </span>
        <span className="flex items-center gap-1">
          <span className="w-3 h-3 rounded" style={{ background: "#dda15e" }} />
          Savanna
        </span>
        <span className="flex items-center gap-1">
          <span className="w-3 h-3 rounded" style={{ background: "#a8dadc" }} />
          Tundra
        </span>
      </div>
    </div>
  );
}
