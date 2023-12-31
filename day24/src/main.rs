use z3;

use z3::{ast::Ast, ast::Int, Config, Context, SatResult, Solver};

use itertools::Itertools;

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

fn p2(hails: &[Hail]) -> i64 {
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // Create integer variables for x_r, y_r, z_r, dx_r, dy_r, dz_r
    let xr = Int::new_const(&ctx, "xr");
    let yr = Int::new_const(&ctx, "yr");
    let zr = Int::new_const(&ctx, "zr");
    let dxr = Int::new_const(&ctx, "dxr");
    let dyr = Int::new_const(&ctx, "dyr");
    let dzr = Int::new_const(&ctx, "dzr");

    // Process each Hail object
    for (i, hail) in hails.iter().enumerate() {
        // Create a new variable 't'
        let t = Int::new_const(&ctx, format!("t{}", i));

        // Add constraints for each dimension
        solver.assert(
            &Int::add(
                &ctx,
                &[
                    &Int::from_i64(&ctx, hail.x as i64),
                    &Int::mul(&ctx, &[&Int::from_i64(&ctx, hail.dx as i64), &t]),
                ],
            )
            ._eq(&Int::add(&ctx, &[&xr, &Int::mul(&ctx, &[&dxr, &t])])),
        );
        solver.assert(
            &Int::add(
                &ctx,
                &[
                    &Int::from_i64(&ctx, hail.y as i64),
                    &Int::mul(&ctx, &[&Int::from_i64(&ctx, hail.dy as i64), &t]),
                ],
            )
            ._eq(&Int::add(&ctx, &[&yr, &Int::mul(&ctx, &[&dyr, &t])])),
        );
        solver.assert(
            &Int::add(
                &ctx,
                &[
                    &Int::from_i64(&ctx, hail.z as i64),
                    &Int::mul(&ctx, &[&Int::from_i64(&ctx, hail.dz as i64), &t]),
                ],
            )
            ._eq(&Int::add(&ctx, &[&zr, &Int::mul(&ctx, &[&dzr, &t])])),
        );
    }

    // Check for satisfiability and return the solution if satisfiable
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            [xr, yr, zr]
                .iter()
                .map(|v| model.eval(v, true).unwrap().as_i64().unwrap())
                .sum::<i64>()
        }
        _ => panic!("what"),
    }
}

fn from_stdin(stdin: std::io::Stdin) -> Vec<Hail> {
    stdin
        .lines()
        .map(|l| Hail::from_string(&l.unwrap()))
        .collect_vec()
}

fn main() {
    //test();

    let hails = from_stdin(std::io::stdin());

    let p1o = p1(&hails, 7.0, 27.0);
    println!("p1 example: {p1o}");

    let p1o = p1(&hails, 200000000000000.0, 400000000000000.0);
    println!("p1 real: {p1o}");

    let p2 = p2(&hails);
    println!("p2: {:?}", p2);
}
