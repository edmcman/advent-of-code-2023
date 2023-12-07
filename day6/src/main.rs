#[derive(Debug)]
struct Race {
    time: usize,
    distance: usize,
}

impl Race {
    fn win(&self, charge_ms: usize) -> bool {
        let go_time = self.time - charge_ms;
        let speed = 1 * charge_ms;
        let boat_goes_distance = go_time * speed;

        boat_goes_distance > self.distance
    }

    fn winners(&self) -> std::ops::Range<usize> {
        /*
         * OK, looking at win code.
         *
         *   boat_goes_distance > self.distance
         * = go_time * speed > self.distance
         * = (self.time - charge_ms) * charge_ms > self.distance
         *
         *
         * self.time and self.distance are constants here.  So solve for charge_ms.
         * = self.time * charge_ms - charge_ms^2 > self.distance.
         * = charge_ms(self.time - charge_ms) > self.distance
         *
         * Quadratic... true if charge_ms < self.time/2 - sqrt(self.time^2 -4*self.distance)/2
         */

        //dbg!((0..self.time).filter(|t| self.win(*t)).collect::<Vec<_>>());

        let a = -1.0;
        let b = self.time as f64;
        let c = -(self.distance as f64);

        // Calculate the discriminant
        let discriminant = b.powi(2) - 4.0 * a * c;

        // Check if the discriminant is non-negative
        if discriminant >= 0.0 {
            let sqrt_discriminant = discriminant.sqrt();
            let root2 = (-b - sqrt_discriminant) / (2.0 * a);
            let root1 = (-b + sqrt_discriminant) / (2.0 * a);

            // There's probably a better way to do this... we want ceil/floor,
            // except that exact integers don't count because of the > in the
            // win function.
            let root1 = root1 + 0.0000001;
            let root2 = root2 - 0.0000001;

            let root1_int = root1.ceil() as usize;
            let root2_int = root2.floor() as usize;

            dbg!(root1, root1_int, root2, root2_int);

            let r = root1_int..root2_int + 1;
            /*let dbg = (0..self.time).filter(|t| self.win(*t)).collect::<Vec<_>>().len();


            dbg!(&r);

            assert!(dbg!(dbg) == dbg!(r.len()));
            */
            r
        } else {
            0..0
        }

        //(0..self.time).filter(|t| self.win(*t)).collect::<Vec<_>>()
        //(0..self.time).filter(|t| (*t as f32) > bound).collect::<Vec<_>>()
    }

    fn num_winners(&self) -> usize {
        dbg!(self.winners()).len()
    }
}

type Races = Vec<Race>;

fn parse(stdin: std::io::Stdin) -> Races {

    let twolines = stdin
        .lines()
        .take(2)
        .map(|l| l.unwrap())
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (time, dist) = (&twolines[0], &twolines[1]);

    time.iter()
        .zip(dist.iter())
        .map(|(t, d)| Race {
            time: *t,
            distance: *d,
        })
        .collect()
}

fn main() {
    let races = parse(std::io::stdin());

    println!("Races: {:?}", races);

    races
        .iter()
        .map(|r| r.winners())
        .for_each(|w| println!("{:?}", w));

    let p1 = races.iter().map(|r| r.num_winners()).product::<usize>();
    println!("Part 1: {p1}");

    let p2_race = Race {
        time: races
            .iter()
            .map(|r| r.time.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
        distance: races
            .iter()
            .map(|r| r.distance.to_string())
            .collect::<String>()
            .parse::<usize>()
            .unwrap(),
    };

    println!("Part 2 race: {:?}", p2_race);
    println!("Answer: {}", p2_race.num_winners());
}
