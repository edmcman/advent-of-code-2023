use itertools::Itertools;
use std::ops::Range;

#[derive(Debug)]
struct OneRangeMap {
    dest_range: Range<usize>,
    src_range: Range<usize>,
}

#[derive(Debug)]
struct MapRangeResult {
    mapped: Vec<Range<usize>>,
    unmapped: Vec<Range<usize>>,
}

fn range_intersect(a: &Range<usize>, b: &Range<usize>) -> Option<Range<usize>> {
    let start = *[a.start, b.start].iter().max().unwrap();
    let end = *[a.end, b.end].iter().min().unwrap();
    let r = start..end;
    if r.is_empty() { None } else { Some(r) }
}

impl OneRangeMap {
    fn map(&self, id: usize) -> Option<usize> {
        if id < self.src_range.start || id >= self.src_range.end {
            return None;
        }

        let idx = id - self.src_range.start;

        return Some(self.dest_range.start + idx);
    }

    fn map_range(&self, in_range: Range<usize>) -> MapRangeResult {
        //dbg!("OneRangeMap map_range");

        let mut in_idx = in_range.start;
        let mut out = vec![];

        //let mut unmapped = vec![];

        let intersection = range_intersect(&in_range, &self.src_range);
        if intersection.is_none() {
            return MapRangeResult {
                mapped: vec![],
                unmapped: vec![in_range],
            };
        }
        let intersection = intersection.unwrap();

        let unmap1 = in_range.start..intersection.start;
        let unmap2 = intersection.end..in_range.end;

        let unmapped = vec![unmap1, unmap2];
        
        dbg!(&intersection);
        dbg!(&self);
        dbg!(&in_range);
        dbg!(&unmapped);

        while in_idx < in_range.end {
            if self.src_range.contains(&in_idx) {
                //dbg!("The src_range", &self.src_range, "contains", in_idx);
                //dbg!(&in_range);
                let earliest_end = *[in_range.end, self.src_range.end].iter().min().unwrap();
                let src_idx_start = in_idx - self.src_range.start;
                dbg!(src_idx_start);
                let dest_idx_start = src_idx_start;
                let dest_idx_end = earliest_end - self.src_range.start;
                //let src_idx_end =
                let range = (self.dest_range.start + dest_idx_start)
                    ..(self.dest_range.start + dest_idx_end);
                //dbg!(range.len());
                dbg!(&range);
                in_idx += range.len();
                //dbg!(&in_range, &self.src_range, &range, in_idx, &earliest_end);
                out.push(range);
            } else if self.src_range.start > in_idx {
                // in_idx is not contained in the src range.  but maybe it's below.
                in_idx += self.src_range.start - in_idx;
                //dbg!("2");
            } else {
                in_idx = in_range.end;
                //dbg!("3");
            }
        }

        dbg!(MapRangeResult {
            mapped: out,
            unmapped,
        })
    }
}

#[derive(Debug)]
struct RangeMap {
    name: String,
    maps: Vec<OneRangeMap>,
}

impl RangeMap {
    fn map(&self, id: usize) -> usize {
        self.maps.iter().find_map(|m| m.map(id)).unwrap_or(id)
    }

    fn map_range(&self, in_range: Range<usize>) -> Vec<Range<usize>> {
        let init = MapRangeResult {
            mapped: vec![],
            unmapped: vec![in_range],
        };

        let mut aftermap = self.maps.iter().fold(init, |mut acc, m| {
            let unmapped_orig = acc.unmapped;
            acc.unmapped = vec![];

            for range in unmapped_orig {
                //dbg!(&range, &m);
                let result = m.map_range(range);
                //dbg!(&result);
                acc.mapped.extend(result.mapped);
                acc.unmapped.extend(result.unmapped);
            }

            acc
        });

        //dbg!(&aftermap);

        aftermap.mapped.extend(aftermap.unmapped);

        aftermap.mapped
    }
}

#[derive(Debug)]
struct Stuff {
    seeds: Vec<usize>,
    maps: Vec<RangeMap>,
}

impl Stuff {
    fn map(&self, id: usize) -> usize {
        self.maps.iter().fold(id, |id, m| m.map(id))
    }

    fn map_range(&self, in_range: Range<usize>) -> Vec<Range<usize>> {
        let init = vec![in_range];

        self.maps.iter()
        //.take(1) // ONE MAP
        .fold(init, |ranges, m| {
            ranges
                .iter()
                .flat_map(|range| m.map_range(range.clone()))
                .collect::<Vec<_>>()
        })
    }
}

fn parse(stdin: std::io::Stdin) -> Stuff {
    let mut lines = stdin.lines();
    let seeds = lines
        .nth(0)
        .unwrap()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut maps = Vec::new();

    let mut map = RangeMap {
        name: String::new(),
        maps: Vec::new(),
    };

    for line in lines.map(|x| x.unwrap()) {
        if line.ends_with(':') {
            if !map.name.is_empty() {
                maps.push(map);
            };

            map = RangeMap {
                name: line,
                maps: Vec::new(),
            };
        } else {
            match line
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
                .as_slice()
            {
                [dest, src, len] => {
                    map.maps.push(OneRangeMap {
                        dest_range: *dest..(*dest + *len),
                        src_range: *src..(*src + *len),
                    });
                }
                [] => {}
                _ => panic!("Invalid line: {}", line),
            }
        }
    }

    maps.push(map);

    Stuff { seeds, maps }
}

fn main() {
    let stuff = parse(std::io::stdin());

    dbg!(&stuff);

    let mapped_seeds = stuff
        .seeds
        .iter()
        .map(|seed| stuff.map(*seed))
        .collect::<Vec<_>>();
    dbg!(&mapped_seeds);

    let themin = stuff
        .seeds
        .iter()
        .map(|seed| stuff.map(*seed))
        .min()
        .unwrap();

    println!("The min: {}", themin);

    // part 2, reinterpret the seeds...
    let new_seeds = stuff
        .seeds
        .iter()
        .tuples::<(_, _)>()
        .map(|(a, b)| *a..(*a+*b))
        //.flatten()
        .collect::<Vec<_>>();

    dbg!(&new_seeds);

    /*println!(
        "The new min: {}",
        new_seeds.iter().map(|seed| stuff.map(*seed)).min().unwrap()
    );*/

    let new_seeds_location_ranges = new_seeds
        .iter()
        .flat_map(|seed| stuff.map_range(seed.clone()))
        .filter(|r| r.len() > 0)
        .collect::<Vec<_>>();

    dbg!(&new_seeds_location_ranges);

    let thenewmin = new_seeds_location_ranges.iter().map(|r| r.start).min().unwrap();

    println!("The new min: {}", thenewmin);

    ////let wat = stuff.map_range(new_seeds[0].clone());
}
