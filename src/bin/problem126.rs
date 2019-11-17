use counter::Counter;
use std::collections::BinaryHeap;

const TARGET: u32 = 1000;

#[derive(Debug, Clone, PartialEq, Eq)]
struct CuboidLayering {
    current: u32,
    layer: u32,
    x: u32,
    y: u32,
    z: u32,
    step: u32,
}

impl CuboidLayering {
    /// For each layer, we need to cover at least the six faces, so 2 * (xy + yz + xz).
    /// Also, we need to cover the sides of faces, so 4 * (x + y + z).
    /// But these covers also have to be covered, so this should be
    /// 4 * (x + y + z) * (n - 1), where `n` is the current layer, and they also
    /// create sides of size 1, which have to be covered. At the first layer, none
    /// has to be created; at the second layer, 4 such sides exist, then 16, etc.
    /// So, add 4 * (n - 2) * (n - 1).
    /// Thus, from the current size of the layer, we need to add 4 * (x + y + z)
    /// and 8 * (layer - 2).
    fn new(x: u32, y: u32, z: u32) -> Self {
        let layer = 1;
        let step = 4 * (x + y + z);
        let current = 2 * (x * y + x * z + y * z);
        CuboidLayering {
            current,
            layer,
            x,
            y,
            z,
            step,
        }
    }

    fn next(mut self) -> Self {
        self.layer += 1;
        self.current += self.step + 8 * (self.layer - 2);
        self
    }
}

/// Reverse ordering to make it easier to use a `BinaryHeap`.
impl Ord for CuboidLayering {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.current
            .cmp(&other.current)
            .then(self.x.cmp(&other.x))
            .then(self.y.cmp(&other.y))
            .then(self.z.cmp(&other.z))
            .reverse()
    }
}

impl PartialOrd for CuboidLayering {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// The minimum number of cubes to cover every visible face on a cuboid
/// measuring 3 x 2 x 1 is twenty-two.
///
/// If we then add a second layer to this solid it would require forty-six
/// cubes to cover every visible face, the third layer would require
/// seventy-eight cubes, and the fourth layer would require one-hundred and
/// eighteen cubes to cover every visible face.
///
/// However, the first layer on a cuboid measuring 5 x 1 x 1 also requires
/// twenty-two cubes; similarly the first layer on cuboids measuring 5 x 3 x 1,
/// 7 x 2 x 1, and 11 x 1 x 1 all contain forty-six cubes.
///
/// We shall define C(n) to represent the number of cuboids that contain n cubes
/// in one of its layers. So C(22) = 2, C(46) = 4, C(78) = 5, and C(118) = 8.
///
/// It turns out that 154 is the least value of n for which C(n) = 10.
///
/// Find the least value of n for which C(n) = 1000.
fn main() {
    let mut counter: Counter<u32, u32> = Counter::new();
    let mut queue = BinaryHeap::new();
    queue.push(CuboidLayering::new(1, 1, 1));

    let mut answer = 0;

    while let Some(layer) = queue.pop() {
        counter.update(std::iter::once(layer.current));
        if *counter.get(&layer.current).unwrap() == TARGET {
            // We need exactly TARGET, so check that the next up won't push us
            // over the count.
            if queue.peek().unwrap().current > layer.current {
                answer = layer.current;
                break;
            }
        }

        if layer.layer == 1 && layer.y == 1 && layer.z == 1 {
            let x = layer.x + 1;
            // We rapidly overflow memory, so try to cut down by filtering.
            let new_cuboids = (1..=x)
                .flat_map(|y| (1..=y).map(move |z| CuboidLayering::new(x, y, z)))
                .filter(|layer| layer.current < 20_000);
            queue.extend(new_cuboids);
        }

        queue.push(layer.next())
    }

    println!("The answer is: {}", answer);
}
