use h3o::{CellIndex, LatLng};
use proj::Proj;

#[derive(Debug, Clone, Copy)]
struct Point2D {
    x: f64, // meters
    y: f64, // meters
}

fn convert_h3_to_lat_lon(h3_cell_str: &str) -> Vec<LatLng> {
    let h3_index: CellIndex = h3_cell_str.parse().unwrap();
    let boundary_geo_h3o: Vec<LatLng> = h3_index.boundary().to_vec();

    println!("\nH3 Cell Geographic Boundary (Lat, Lon degrees):");
    for (i, vertex_ll_ref) in boundary_geo_h3o.iter().enumerate() {
        let vertex_ll = *vertex_ll_ref;
        println!(
            "  Vertex {}: ({:.6}, {:.6})",
            i,
            vertex_ll.lat(),
            vertex_ll.lng()
        );
    }

    boundary_geo_h3o
}

fn convert_to_projected_points(
    transformer: &Proj,
    x_lat: f64,
    y_lon: f64,
    h3_lat_lon_vertices: &Vec<LatLng>,
) -> (Point2D, Vec<Point2D>) {
    let (projected_x, projected_y): (f64, f64) = transformer.convert((y_lon, x_lat)).unwrap();
    let xy_projected = Point2D {
        x: projected_x,
        y: projected_y,
    };

    println!("\nProjected Test Point (Web Mercator):");
    println!("  X: {:.3} m, Y: {:.3} m", xy_projected.x, xy_projected.y);

    // ----------------------------------------------------------------------------------------------------------------------------

    let mut h3_xy_projected: Vec<Point2D> = Vec::with_capacity(h3_lat_lon_vertices.len());
    println!("\nProjected H3 Cell Vertices (Web Mercator):");
    for (i, geo_vertex) in h3_lat_lon_vertices.iter().enumerate() {
        let (vx, vy): (f64, f64) = transformer
            .convert((geo_vertex.lng(), geo_vertex.lat()))
            .unwrap();
        let projected_vertex = Point2D { x: vx, y: vy };
        h3_xy_projected.push(projected_vertex);
        println!(
            "  Vertex {}: ({:.3}, {:.3}) meters",
            i, projected_vertex.x, projected_vertex.y
        );
    }

    return (xy_projected, h3_xy_projected);
}

/// Checks if a point is inside a 2D convex polygon.
/// Assumes polygon vertices are ordered counter-clockwise.
/// The epsilon is used to provide a tolerance for points slightly outside an edge.
fn is_point_in_convex_polygon(
    point: &Point2D,
    polygon_vertices: &[Point2D],
    epsilon_m_sq: f64,
) -> bool {
    let num_vertices = polygon_vertices.len();
    if num_vertices < 3 {
        return false;
    }

    let px_m = point.x;
    let py_m = point.y;

    for i in 0..num_vertices {
        let p1 = polygon_vertices[i];
        let p2 = polygon_vertices[(i + 1) % num_vertices];

        let x1_m = p1.x;
        let y1_m = p1.y;
        let x2_m = p2.x;
        let y2_m = p2.y;

        let d_j_m_sq = (x2_m - x1_m) * (py_m - y1_m) - (y2_m - y1_m) * (px_m - x1_m);

        if d_j_m_sq < -epsilon_m_sq {
            return false;
        }
    }
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    const STATIC_EPSILON_M_SQ: f64 = 1.5;
    let h3_cell_str: &str = "8a2a1072b59ffff"; // Statue of Liberty, Resolution 10
    let test_point_lat: f64 = 40.689704593753824; // Just in front of the statue
    let test_point_lon: f64 = -74.04495563970343;

    println!(
        "Input Test Point: Lat={:.6}, Lon={:.6}",
        test_point_lat, test_point_lon
    );
    println!("Input H3 Cell Index: {}", h3_cell_str);
    println!("Static Epsilon for Test: {} m^2", STATIC_EPSILON_M_SQ);

    // ----------------------------------------------------------------------------------------------------------------------------

    let transformer = Proj::new_known_crs("EPSG:4326", "EPSG:3857", None).map_err(|e| {
        Box::<dyn std::error::Error>::from(format!(
            "Failed to create PROJ transformer from {} to {}: {}",
            "EPSG:4326", "EPSG:3857", e
        ))
    })?;

    let boundary_geo_h3o: Vec<LatLng> = convert_h3_to_lat_lon(h3_cell_str); // vertices must be counter-clockwise

    let (test_projected, h3_projected) = convert_to_projected_points(
        &transformer,
        test_point_lat,
        test_point_lon,
        &boundary_geo_h3o,
    );

    // ----------------------------------------------------------------------------------------------------------------------------

    let is_inside = is_point_in_convex_polygon(&test_projected, &h3_projected, STATIC_EPSILON_M_SQ);

    println!("\n--- Result ---");
    println!("Is test point inside H3 cell: {}", is_inside);

    Ok(())
}
