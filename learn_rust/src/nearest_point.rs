struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // First source point.
    let first_point: usize = 8;

    // Array of points with x and y position.
    let points: [Point; 10] = [
        Point{x: 0.0, y: 1000.0},
        Point{x: 100.0, y: 100.0},
        Point{x: 200.0, y: 800.0},
        Point{x: 300.0, y: 500.0},
        Point{x: 400.0, y: 300.0},
        Point{x: 500.0, y: 200.0},
        Point{x: 600.0, y: 600.0},
        Point{x: 600.0, y: 400.0},
        Point{x: 800.0, y: 300.0},
        Point{x: 900.0, y: 900.0},
    ];

    // Nearest current distance from first point.
    let mut nearest_distance: f64 = f64::MAX;
    // Nearest point.
    let mut nearest_point: usize = usize::MAX;

    // Points array loop.
    for i in 0..points.len() {
        // Square root of distance between first point and second point.
        let square_root: f64 = distance(
            points[i].x,
            points[first_point].x,
            points[i].y,
            points[first_point].y)
            .sqrt();

        // Compare current nearest distance with current loop point in points array.
        if square_root < nearest_distance && i != first_point {
            nearest_distance = square_root;
            nearest_point = i;
        }
    }

    // Print nearest found point from first point.
    println!("Nearest point is: {}", nearest_point);
}

// Euclidean distance formula.
fn distance(x1:f64, x2:f64, y1:f64, y2:f64) -> f64 {
    return ((x1 - x2).abs() * 2.0) + ((y1 - y2).abs() * 2.0);
}
