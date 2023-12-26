use itertools::Itertools;
use rand::{seq::SliceRandom, thread_rng};

type TallGraph = Vec<(String, Vec<String>)>;
type FlatGraph = Vec<(String, String, usize)>;

fn parse_input(stdin: std::io::Stdin) -> (TallGraph, FlatGraph) {
    let tall = stdin
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let (a, b) = l.split_once(": ").unwrap();
            (
                a.to_string(),
                b.split_ascii_whitespace()
                    .map(|s| s.to_string())
                    .collect_vec(),
            )
        })
        .collect_vec();

    let flat = tall
        .iter()
        .flat_map(|(a, list)| {
            list.iter().map(|b| (a.to_string(), b.to_string(), 1))
            //.chain(list.iter().map(|b| (b.to_string(), a.to_string(), 1)))
        })
        .collect_vec();

    (tall, flat)
}

fn flatgraph_vertices(graph: &FlatGraph) -> Vec<String> {
    graph
        .iter()
        .flat_map(|(a, b, _)| [a, b])
        .unique()
        .map(|s| s.to_string())
        .collect_vec()
}

fn karger(graph: &FlatGraph) -> FlatGraph {
    let mut rng = rand::thread_rng();
    let mut g = graph.clone();

    while flatgraph_vertices(&g).len() > 2 {
        let (a, b, _) = g.choose(&mut rng).unwrap();
        let a = a.clone();
        let b = b.clone();

        // contract a and b into a.
        g = g
            .into_iter()
            // remove edges between a and b
            .filter(|(x, y, _)| !((x == &a && y == &b) || (x == &b && y == &a)))
            // replace edges to b with edges to a
            .map(|(x, y, z)| {
                let merged = format!("{}-{}", a, b);

                let new = (
                    if x == a || x == b { merged.clone() } else { x },
                    if y == b || y == a { merged.clone() } else { y },
                    z,
                );

                new
            })
            .collect_vec();

        //dbg!(&g);
    }

    dbg!(g.len());

    g
}

fn p1(g: &FlatGraph) -> usize {
    let binding = std::iter::repeat(g)
        .map(|g| karger(g))
        .find(|g| g.len() == 3)
        .unwrap();
    let edge = binding.get(0).unwrap();

    edge.0.split("-").count() * edge.1.split("-").count()
}

fn main() {
    let (tallgraph, flatgraph) = parse_input(std::io::stdin());

    //let k = karger(&flatgraph);
    //dbg!(&k);

    let p1 = p1(&flatgraph);
    println!("p1: {p1}");

    //let mst = pathfinding::undirected::kruskal::kruskal(&flatgraph).collect_vec();
    //dbg!(&mst);

    //dbg!(g.len());
    //dbg!(mst.len());
    println!("tall: {:?}", tallgraph);
    println!("flat: {:?}", flatgraph);
    //println!("mst: {:?}", mst);
}
