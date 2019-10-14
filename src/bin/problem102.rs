use std::fs;
use std::str::FromStr;

const FILENAME: &str = "inputs/p102_triangles.txt";

type Point = (f64, f64);
const ORIGIN: Point = (0.0, 0.0);

struct Triangle {
    points: [Point; 3],
}

impl Triangle {
    fn new(points: [Point; 3]) -> Self {
        Triangle { points }
    }

    // Get the barycentric coordinates of a point `p`.
    fn barycentric_coordinates(&self, p: Point) -> [f64; 3] {
        let (x, y) = p;
        let (x1, y1) = self.points[0];
        let (x2, y2) = self.points[1];
        let (x3, y3) = self.points[2];

        let lambda1 = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3))
            / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
        let lambda2 = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3))
            / ((y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3));
        let lambda3 = 1.0 - lambda1 - lambda2;

        [lambda1, lambda2, lambda3]
    }

    // A point is contained inside the hull if all the barycentric coordinates are between 0 and  1.
    // 0 < lambda_i < 1. Use an inequality to include the edge.
    fn contains_origin(&self) -> bool {
        let origin = self.barycentric_coordinates(ORIGIN);
        origin.iter().all(|&b| b >= 0.0 && b <= 1.0)
    }
}

impl From<&str> for Triangle {
    fn from(input: &str) -> Self {
        let mut iter = input.split(',').map(f64::from_str);
        let p1 = (iter.next().unwrap().unwrap(), iter.next().unwrap().unwrap());
        let p2 = (iter.next().unwrap().unwrap(), iter.next().unwrap().unwrap());
        let p3 = (iter.next().unwrap().unwrap(), iter.next().unwrap().unwrap());

        Triangle::new([p1, p2, p3])
    }
}

/// Three distinct points are plotted at random on a Cartesian plane, for which -1000 ≤ x, y ≤
/// 1000, such that a triangle is formed.
///
/// Consider the following two triangles:
///
/// A(-340,495), B(-153,-910), C(835,-947)
/// X(-175,41), Y(-421,-714), Z(574,-645)
///
/// It can be verified that triangle ABC contains the origin, whereas triangle XYZ does not.
///
/// Using triangles.txt, a 27K text file containing the co-ordinates of one thousand "random"
/// triangles, find the number of triangles for which the interior contains the origin.
///
/// NOTE: The first two examples in the file represent the triangles in the example given above.
fn main() {
    let input = fs::read_to_string(FILENAME).unwrap();

    let answer = input
        .lines()
        .filter(|&line| Triangle::from(line).contains_origin())
        .count();

    println!("The answer is: {}", answer);
}
