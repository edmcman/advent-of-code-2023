use indicatif::ProgressIterator;
use itertools::Itertools;
use memoize::memoize;
use rayon::prelude::*;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from_string(s: &str) -> Self {
        match s.split(',').collect_vec().as_slice() {
            [x, y, z] => Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
                z: z.parse().unwrap(),
            },
            _ => panic!("Invalid point string"),
        }
    }

    fn is_immediately_below(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z - 1
    }

    fn is_somewhere_below(&self, other: &Point) -> Option<usize> {
        if self.x == other.x && self.y == other.y && self.z < other.z {
            let diff = other.z - self.z;
            Some(diff)
        } else {
            None
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Brick {
    start: Point,
    end: Point,
    id: usize,
}

impl Brick {
    fn from_string(s: &str, id: usize) -> Self {
        match s.split('~').collect_vec().as_slice() {
            [start, end] => Brick {
                start: Point::from_string(start),
                end: Point::from_string(end),
                id,
            },
            _ => panic!("Invalid brick string"),
        }
    }

    fn points(&self) -> impl Iterator<Item = Point> + '_ {
        let xs = (self.start.x)..=(self.end.x);

        xs.flat_map(move |x| {
            let ys = (self.start.y)..=(self.end.y);
            ys.flat_map(move |y| {
                let zs = (self.start.z)..=(self.end.z);
                zs.map(move |z| Point { x, y, z })
            })
        })
    }

    fn lowest_points(&self) -> impl Iterator<Item = Point> + '_ {
        let min_z = *[self.start.z, self.end.z].iter().min().unwrap();
        self.points().filter(move |p| p.z == min_z)
    }

    fn on_ground(&self) -> bool {
        self.to_ground() == 0
    }

    fn to_ground(&self) -> usize {
        let min_z = **[&self.start.z, &self.end.z].iter().min().unwrap();
        assert!(min_z > 0);
        min_z - 1
    }

    // What is the shortest distance that the other brick could fall to me?
    fn is_somewhere_below(&self, other: &Brick) -> Option<usize> {
        // If any of my points is somewhere below one of their points
        self.points()
            .filter_map(|myp| {
                other
                    .points()
                    .filter_map(|theirp| myp.is_somewhere_below(&theirp))
                    .min()
            })
            .min()
    }

    fn supports(&self, other: &Brick) -> bool {
        // If any of my points is immediately below one of their points
        self != other
            && self.points().any(|myp| {
                other
                    .points()
                    .any(|theirp| myp.is_immediately_below(&theirp))
            })
    }

}

// Let's compute the final positions recursively.  Let's say we have single
// block points to start.  We have a block.  How far will it fall?  Well, it
// will fall to the ground, or to the block below it.

// If we expand to larger blocks, things get a little trickier.  It will fall to
// the highest point of any block below it. E.g., recurse and take the max.

// It's super annoying that to get this to memoize easily, this function can't take references.  Sigh...

#[memoize(Capacity: 1500)]
fn compute_fall(bricks: Vec<Brick>, brick: Brick) -> Brick {
    //println!("compute_fall {:?}", &brick);
    let are_we_on_the_ground = brick.on_ground();
    if are_we_on_the_ground {
        brick.clone()
    } else {
        let other_bricks = bricks
            .iter()
            .filter(|&other| other.is_somewhere_below(&brick).is_some() && *other != brick)
            .collect_vec();

        // For each point in the x-y plane, we need to determine which block is
        // the highest but still below us.
        let points_in_brick = brick.lowest_points(); //.map(|p| (p.x,p.y));

        let block_below_each_point = points_in_brick.filter_map(|p| {
            // Is there another brick below this point?

            let closest_point_in_all_blocks = other_bricks
                .iter()
                .filter_map(|other_brick| {
                    let points_in_other_brick = other_brick.points();
                    let closest_point_in_other = points_in_other_brick
                        .filter(|other_point| other_point.is_somewhere_below(&p).is_some())
                        .max_by_key(|other_point| other_point.z);

                    closest_point_in_other.map(|other_point| (p.clone(), other_brick, other_point))
                })
                .max_by_key(|(_, _, other_point)| other_point.z);

            closest_point_in_all_blocks
        });

        // OK, now we know the highest point and block below each point of our
        // brick.  We need to take one for each brick.

        // We CANNOT take the max of the highest point of each brick, because
        // these are the current highest points, and they may change after
        // falling.

        let highest_points = block_below_each_point
            // group by brick
            .group_by(|(_, b, _)| b.id)
            .into_iter()
            // take max by z. I think we can actually just pick an arbitrary one, but whatever.
            .map(|(_, group)| group.max_by_key(|(_, _, p)| p.z).unwrap())
            .collect_vec();

        let dz = if highest_points.is_empty() {
            // no one is below us, we can just fall to the ground
            brick.to_ground()
        } else {
            // Our brick is on top of some other bricks.  We need to find their
            // final resting locations.

            let highest_point = highest_points
                .iter()
                .map(|(p, b, bp)| {
                    let adjusted_brick = compute_fall(bricks.clone(), (**b).clone());
                    (p, b, bp, adjusted_brick)
                })
                .max_by_key(|(_p, _b, _bp, adjusted_brick)| {
                    assert!(adjusted_brick.end.z >= adjusted_brick.start.z);
                    adjusted_brick.end.z
                })
                .unwrap();

            // We know where the highest brick has landed.  Find the point in
            // the brick that will intersect.

            let bp = &highest_point.2;
            let our_brick_point = &highest_point.0;
            let adjusted_brick = &highest_point.3;
            let adjusted_intersect_point = adjusted_brick
                .points()
                .filter(|p| p.x == bp.x && p.y == bp.y)
                .max_by_key(|p| p.z)
                .unwrap();

            our_brick_point.z - adjusted_intersect_point.z - 1
        };

        let mut new_brick = brick.clone();
        new_brick.start.z -= dz;
        new_brick.end.z -= dz;

        new_brick
    }
}

fn compute_fall_for_all(bricks: &Vec<Brick>) -> Vec<Brick> {
    bricks
        .iter()
        .map(|b| compute_fall(bricks.clone(), b.clone()))
        .collect::<Vec<_>>()

}

fn parse(stdin: std::io::Stdin) -> Vec<Brick> {
    stdin
        .lines()
        .map(|l| l.unwrap())
        .enumerate()
        .map(|(n, l)| Brick::from_string(&l, n))
        .collect_vec()
}

fn p1(bricks: &Vec<Brick>) -> usize {
    println!("Starting disintegrable bricks computation");

    let disintegratable_bricks = bricks
        //.par_iter()
        .iter()
        .progress()
        .filter(|&current_brick| {
            // is `current_brick` disintegratable?
            let all_other_bricks_we_support = bricks
                .iter()
                .filter(|other| current_brick.supports(&other))
                .collect_vec();

            let wouldfall = all_other_bricks_we_support
                .iter()
                .filter(|supported| {
                    // no other brick supports supported except for us
                    let other_supporters = bricks
                        .iter()
                        .filter(|otherother| {
                            otherother.supports(supported) && *otherother != current_brick
                        })
                        .collect_vec();

                    other_supporters.len() == 0
                })
                .collect_vec();

            wouldfall.len() == 0
        })
        .collect::<Vec<_>>();

    dbg!(&disintegratable_bricks);

    disintegratable_bricks.len()
}

fn p2(bricks: &Vec<Brick>) -> usize {

    // This is very inefficient because we can't reuse the cache :-(.  But I'm
    // just going to use a bunch of threads and wait a bit.

    bricks
        .par_iter()
        .enumerate()
        .map(|(i,brick)| {
            println!("progress {i}");
            let other_bricks = bricks
                .clone()
                .into_iter()
                .filter(|other| other != brick)
                .collect_vec();

            let fallen = compute_fall_for_all(&other_bricks);

            other_bricks
                .iter()
                .zip(fallen)
                .filter(|(a, b)| *a != b)
                .count()
        })
        .sum()
}

fn _vis(bricks: &Vec<Brick>) {
    let max_x = bricks.iter().map(|b| b.end.x).max().unwrap();
    let _max_y = bricks.iter().map(|b| b.end.y).max().unwrap();
    let max_z = bricks.iter().map(|b| b.end.z).max().unwrap();

    for r in (0..max_z).rev() {
        for c in 0..max_x {
            let mut found = None;
            for brick in bricks.iter() {
                if brick.start.x <= c && brick.end.x >= c && brick.start.z <= r && brick.end.z >= r
                {
                    found = Some(brick.id);
                    break;
                }
            }
            if found.is_some() {
                print!("{:02}", found.unwrap());
            } else {
                print!("..");
            }
        }
        println!(" {:02}", r);
    }
    println!("");
}

fn main() {
    let bricks = parse(std::io::stdin());

    let fallen_bricks = compute_fall_for_all(&bricks);

    let p1 = p1(&fallen_bricks);
    println!("p1: {p1}");

    let p2 = p2(&fallen_bricks);
    println!("p2: {p2}");
}
