
# H3 Point-in-Cell Geometric Test (Rust Prototype)

This repository contains a Rust prototype for determining if a given latitude/longitude point falls within a specified H3 hexagonal cell. The check is performed by projecting the H3 cell's boundary and the point to Web Mercator (EPSG:3857) coordinates and then executing a 2D planar point-in-polygon algorithm.

## Inspiration and Goal 💡

This work is inspired by the concepts presented in "[Zero-Knowledge Location Privacy
 via Accurate Floating-Point SNARKs](https://eprint.iacr.org/2024/1842.pdf)".

The primary motivation is to explore an implementation for verifying point-in-H3-cell containment in a manner that could be adapted to be **zero-knowledge (ZK) friendly**. A key aspect for ZK circuits is the avoidance of floating-point arithmetic. While this Rust prototype currently uses `f64` for projections and calculations, the geometric approach (point-in-polygon with projected coordinates) is a step towards a system that could potentially be implemented with fixed-point arithmetic inside a ZK circuit.

## Key Characteristics and Limitations

This program is a **prototype** and demonstrates the core geometric check. It has the following characteristics and known limitations:

-   **Projection Method (EPSG:3857 Web Mercator):**
    
    -   Geographic coordinates (latitude/longitude) are projected to Web Mercator (x/y meters) for the geometric test.
    -   **Polar Regions:** The Web Mercator projection exhibits significant distortions and is typically undefined or clipped at extreme latitudes (generally beyond ±85°). Consequently, H3 cells located in these far northern or southern regions may not be processed with reliable accuracy by this method.
    -   **Dateline:** H3 cells that cross the ±180° longitude line (the international dateline) can pose challenges for simple planar projections and point-in-polygon tests. This prototype does not include specialized handling for such cases, and their behavior has not been specifically tested.
    -   **Recommended Use:** The current projection approach is most suitable for H3 cells situated in temperate and equatorial regions, and for cells that do not intersect the dateline or lie in extreme polar areas.
-   **Epsilon Value in Geometric Test:**
    
    -   A **static epsilon of `1.5 m²` (square meters)** is used within the 2D point-in-polygon algorithm (specifically, in the condition `if d_j < -epsilon`).
    -   **Purpose:** This fixed tolerance is implemented to ensure points located exactly on, or extremely close to, an H3 cell boundary are inclusively handled. It aims to accommodate minor discrepancies arising from floating-point arithmetic or slight differences between the H3 library's cell assignment logic and a strict geometric boundary interpretation.
    -   **Behavior & Implications:** Due to the nature of the Web Mercator projection, which distorts metric sizes based on latitude, this fixed `1.5 m²` epsilon has a varying _relative_ effect:
        -   For **low-resolution H3 cells (e.g., resolutions 0-12)**, which appear very large when projected into Web Mercator, the `1.5 m²` epsilon is proportionally tiny. This results in an almost negligible effective ground buffer (on the order of micrometers to centimeters), meaning the boundary test is very strict for these large cells.
        -   For **high-resolution H3 cells (e.g., resolutions 13 and up)**, the `1.5 m²` epsilon becomes more significant compared to their smaller projected dimensions:
            -   At **Resolution 13** (average equatorial edge length ~3.6m), this epsilon creates an effective ground buffer of approximately **0.4 meters** around the cell boundary.
            -   At **Resolution 15** (average equatorial edge length ~0.5m), it yields an effective ground buffer of approximately **3 meters**. This buffer is substantially larger than the cell itself, ensuring a highly inclusive test for boundary points, though the margin is very generous.
    -   **Effective Range:** Consequently, this static `1.5 m²` epsilon primarily establishes a noticeable geometric buffer for H3 cells at **resolutions 13 and higher**. For cells at lower resolutions, its impact as a buffer is minimal.