const INPUT: i64 = 50;

#[derive(Debug, Clone, Copy)]
struct Coordinates(i64, i64);

impl Coordinates {
    fn new(x: i64, y: i64) -> Self {
        Coordinates(x, y)
    }

    fn distance(self, other: Self) -> f64 {
        (((self.0 - other.0).pow(2) + (self.1 - other.1).pow(2)) as f64).sqrt()
    }

    fn distance_to_origin(self) -> f64 {
        (self.0 as f64).hypot(self.1 as f64)
    }
}

#[derive(Debug, Clone, Copy)]
struct OriginTriangle {
    p: Coordinates,
    q: Coordinates,
}

impl OriginTriangle {
    fn new(p: Coordinates, q: Coordinates) -> Self {
        OriginTriangle { p, q }
    }

    fn is_right_triangle(&self) -> bool {
        let mut sides = [
            self.p.distance_to_origin(),
            self.q.distance_to_origin(),
            self.p.distance(self.q),
        ];
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        // Turns out the issue was floating point inaccuracy.
        (sides[0].powi(2) + sides[1].powi(2) - sides[2].powi(2)).abs() < 1e-8
    }
}

/// The points P (x1, y1) and Q (x2, y2) are plotted at integer co-ordinates and are joined to
/// the origin, O(0,0), to form ΔOPQ.
///
/// There are exactly fourteen triangles containing a right angle that can be formed when each
/// co-ordinate lies between 0 and 2 inclusive; that is,
/// 0 ≤ x1, y1, x2, y2 ≤ 2.
///
/// Given that 0 ≤ x1, y1, x2, y2 ≤ 50, how many right triangles can be formed?
fn main() {
    let mut points: Vec<Coordinates> = (0..=INPUT).flat_map(|x| {
        (0..=INPUT).filter_map(move |y| {
            // The origin is not a valid point.
            if x != 0 || y != 0 {
                Some(Coordinates::new(x, y))
            } else {
                None
            }
        })
    }).collect();

    let mut triangles = Vec::new();
    while let Some(p) = points.pop() {
        for &q in &points {
            triangles.push(OriginTriangle::new(p, q))
        }
    }

    let answer= triangles.into_iter().filter(OriginTriangle::is_right_triangle).count();
    println!("The answer is: {}", answer);
}
