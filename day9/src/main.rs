type Seq = Vec<i32>;

fn parse(stdin: std::io::Stdin) -> impl Iterator<Item = Seq> {
    stdin.lines().map(|l| l.unwrap()).map(|l| {
        l.split_ascii_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    })
}

fn extrapolate(seq: &Seq) -> i32 {
    if seq.iter().all(|e| *e == 0) {
        0
    } else {
        let derivative = seq
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Seq>();
        let e = extrapolate(&derivative);
        seq.last().unwrap() + e
    }
}

fn extrapolate_backwards(seq: &Seq) -> i32 {
    if seq.iter().all(|e| *e == 0) {
        0
    } else {
        let derivative = seq
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect::<Seq>();
        let e = extrapolate_backwards(&derivative);
        seq.first().unwrap() - e
    }
}


fn main() {
    let seqs = parse(std::io::stdin());
    let seqs = seqs.collect::<Vec<_>>();

    let p1 = seqs.iter().map(|s| extrapolate(&s)).sum::<i32>();

    let p2 = seqs.iter().map(|s| extrapolate_backwards(&s)).sum::<i32>();

    println!("p1: {p1} p2: {p2}");
}
