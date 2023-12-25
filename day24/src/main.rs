use itertools::Itertools;
use nalgebra as na;

use na::{DMatrix, DVector};

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct Hail {
    x: isize,
    y: isize,
    z: isize,
    dx: isize,
    dy: isize,
    dz: isize,
}

impl Hail {
    fn from_string(s: &str) -> Self {
        let (a, b) = s.split_once(" @ ").unwrap();
        let binding = a
            .split(", ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let (x, y, z) = match binding.as_slice() {
            [x, y, z] => (x, y, z),
            _ => panic!("Invalid input"),
        };
        let binding = b
            .split(", ")
            .map(|s| s.trim_start().parse::<isize>().unwrap())
            .collect::<Vec<_>>();
        let (dx, dy, dz) = match binding.as_slice() {
            [dx, dy, dz] => (dx, dy, dz),
            _ => panic!("Invalid input"),
        };
        Self {
            x: *x,
            y: *y,
            z: *z,
            dx: *dx,
            dy: *dy,
            dz: *dz,
        }
    }

    fn slope(&self) -> f64 {
        (self.dy as f64) / (self.dx as f64)
    }

    fn intersect_xy_with(&self, other: &Hail) -> Option<(f64, f64)> {
        let (m1, m2) = (self.slope(), other.slope());
        let (dx1, dx2) = (self.dx as f64, other.dx as f64);
        let (dy1, dy2) = (self.dy as f64, other.dy as f64);

        let (x1, x2) = (self.x as f64, other.x as f64);
        let (y1, y2) = (self.y as f64, other.y as f64);
        //let (b1, b2) = (y1 - (m1 * x1), y2 - (m2 * x2));

        if m1 == m2 {
            None
        } else {
            let intersect_x = -((-dx1 * dx2 * y1) + (dx1 * dx2 * y2) - (dx1 * dy2 * x2)
                + (dx2 * dy1 * x1))
                / ((dx1 * dy2) - (dx2 * dy1));

            let dx1_actual = intersect_x - x1;
            if dx1_actual / dx1 < 0.0 {
                // wrong way.
                return None;
            }

            let dx2_actual = intersect_x - x2;
            if dx2_actual / dx2 < 0.0 {
                // wrong way.
                return None;
            }

            let intersect_y = -((-dx1 * dy2 * y1) + (dx2 * dy1 * y2) + (dy1 * dy2 * x1)
                - (dy1 * dy2 * x2))
                / ((dx1 * dy2) - (dx2 * dy1));

            Some((intersect_x, intersect_y))
        }
    }
}

fn p1(hail: &Vec<Hail>, min: f64, max: f64) -> usize {
    let intersections = hail
        .iter()
        .enumerate()
        .map(|(i1, h1)| {
            hail.iter()
                .enumerate()
                .filter(move |(i2, _)| i2 > &i1)
                .map(move |(i2, h2)| ((h1, h2), h1.intersect_xy_with(h2)))
        })
        .flatten()
        .collect_vec();

    dbg!(&intersections);

    let tmp = intersections
        .iter()
        .filter_map(|(pair, o)| {
            if o.is_some() {
                Some((pair, o.unwrap()))
            } else {
                None
            }
        })
        .filter(|(_, (x, y))| *x >= min && *x <= max && *y >= min && *y <= max)
        .collect_vec();

    dbg!(&tmp);

    for t in &tmp {
        println!("{:?}, {:?}", t, t.0 .0.slope());
    }

    tmp.len()
}

fn p2(hail: &Vec<Hail>) {
    let n = hail.len();

    let nrow = 3 * n;
    let ncol = 6 + n;

    let r1p1 = [1.0, 0.0, 0.0, -1.0, 0.0, 0.0];
    let r2p1 = [0.0, 1.0, 0.0, 0.0, -1.0, 0.0];
    let r3p1 = [0.0, 0.0, 1.0, 0.0, 0.0, -1.0];

    let A = hail
        .iter()
        .enumerate()
        .flat_map(|(i, h)| {
            // ok, i zeroes, dx1, then n - i - 2 zeros
            let r1 = r1p1.clone()
                .into_iter()
                .chain(std::iter::repeat(0.0).take(i))
                .chain(std::iter::once(h.dx as f64))
                .chain(std::iter::repeat(0.0).take(n - i - 1))
                .collect_vec();
            let r2 = r2p1.clone()
                .into_iter()
                .chain(std::iter::repeat(0.0).take(i))
                .chain(std::iter::once(h.dy as f64))
                .chain(std::iter::repeat(0.0).take(n - i - 1))
                .collect_vec();
            let r3 = r3p1.clone()
                .into_iter()
                .chain(std::iter::repeat(0.0).take(i))
                .chain(std::iter::once(h.dz as f64))
                .chain(std::iter::repeat(0.0).take(n - i - 1))
                .collect_vec();
            [r1, r2, r3]
            //vec![r1, r2, r3]
        })
        .flatten()
        .collect_vec();
}

fn from_stdin(stdin: std::io::Stdin) -> Vec<Hail> {
    stdin
        .lines()
        .map(|l| Hail::from_string(&l.unwrap()))
        .collect_vec()
}

fn test() {
    // Define your constants for both sets of equations
    let (x1, y1, z1, dx1, dy1, dz1) = (1.0, 2.0, 3.0, 4.0, 5.0, 6.0); // First set
    let (x2, y2, z2, dx2, dy2, dz2) = (7.0, 8.0, 9.0, 10.0, 11.0, 12.0); // Second set

    // Coefficient matrix A
    // so there are 3 rows for each hailstone
    // columns: 6 + #hailstones
    let a = DMatrix::from_row_slice(
        6,
        8,
        &[
            1.0, 0.0, 0.0, -1.0, 0.0, 0.0, dx1, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, dy1, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0, -1.0, dz1, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0, dx2, 0.0, 1.0,
            0.0, 0.0, -1.0, 0.0, 0.0, dy2, 0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, dz2,
        ],
    );

    // Constants vector b
    let b = DVector::from_column_slice(&[x1, y1, z1, x2, y2, z2]);

    // Solve the least squares problem
    let solution = a.clone().svd(true, true).solve(&b, 1.0e-12).unwrap();

    println!("Solution: {:?}", solution);
}

fn main() {
    test();

    let hails = from_stdin(std::io::stdin());
    let p1o = p1(&hails, 7.0, 27.0);
    println!("p1 example: {p1o}");

    let p1o = p1(&hails, 200000000000000.0, 400000000000000.0);
    println!("p1 real: {p1o}");
}
