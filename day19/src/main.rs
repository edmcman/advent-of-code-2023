use itertools::Itertools;
use regex::Regex;
use std::{collections::HashSet, ops::RangeInclusive, thread::current};

use pathfinding::prelude::bfs_reach;

fn range_intersection(
    r1: &RangeInclusive<usize>,
    r2: &RangeInclusive<usize>,
) -> RangeInclusive<usize> {
    let start = r1.start().max(r2.start());
    let end = r1.end().min(r2.end());

    *start..=*end
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl State {

    fn from_string(s: &str) -> Self {
        let re = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();

        let cap = re.captures(s).unwrap();

        let x = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let m = cap.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let a = cap.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let s = cap.get(4).unwrap().as_str().parse::<usize>().unwrap();

        Self { x, m, a, s }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Condition {
    op1: String,
    op2: String,
    compare_op: char,
}

impl Condition {
    fn negate(&self) -> Self {
        let op2 = self.op2.parse::<usize>().unwrap();
        match self.compare_op {
            '>' => Self {
                op1: self.op1.to_owned(),
                op2: (op2 + 1).to_string(),
                compare_op: '<',
            },
            '<' => Self {
                op1: self.op1.to_owned(),
                op2: (op2 - 1).to_string(),
                compare_op: '>',
            },
            _ => panic!("uh oh"),
        }
    }

    fn from_string(str: &str) -> Self {
        match (str.split_once('<'), str.split_once('>')) {
            (Some((op1, op2)), None) => Self {
                op1: op1.to_string(),
                op2: op2.to_string(),
                compare_op: '<',
            },
            (None, Some((op1, op2))) => Self {
                op1: op1.to_string(),
                op2: op2.to_string(),
                compare_op: '>',
            },
            _ => panic!("uh oh"),
        }
    }
}

#[derive(Debug)]
struct Rule {
    name: String,
    subrules: Vec<(Condition, String)>,
    default: String,
}

impl Rule {
    fn from_string(s: &str) -> Self {
        let re = Regex::new(r"(\w+)\{((([^,\}]+:\w+),)*)([^,\}]+)\}").unwrap();

        let cap = re.captures(s).unwrap();

        let name = cap.get(1).unwrap().as_str().to_string();
        let default = cap.get(5).unwrap().as_str().to_string();
        let mut subrules = cap.get(2).unwrap().as_str().split(",").collect_vec();
        subrules.pop();
        let subrules = subrules
            .into_iter()
            .map(|v| v.split_once(':').unwrap())
            .map(|(a, b)| (Condition::from_string(a), b.to_string()))
            .collect::<Vec<(Condition, String)>>();

        Rule {
            name,
            subrules,
            default,
        }
    }
}

fn parse(stdin: std::io::Stdin) -> (Vec<Rule>, Vec<State>) {
    let lines = stdin.lines().map(|l| l.unwrap()).collect_vec();

    let rule_lines = lines.iter().take_while(|l| !l.is_empty()).collect_vec();

    let rules = rule_lines
        .into_iter()
        .map(|l| Rule::from_string(&l))
        .collect_vec();

    let state_iter = lines.iter().skip_while(|l| !l.is_empty()).skip(1);

    let states = state_iter.map(|l| State::from_string(&l)).collect_vec();

    (rules, states)
}

fn eval_op(state: &State, s: &str) -> usize {
    if let Ok(n) = s.parse::<usize>() {
        n
    } else {
        match s {
            "x" => state.x,
            "m" => state.m,
            "a" => state.a,
            "s" => state.s,
            _ => panic!("uh oh"),
        }
    }
}

fn eval_op_symbolic<'a>(state: &'a mut SymbolicState, s: &'a str) -> &'a mut RangeInclusive<usize> {
    match s {
        "x" => &mut state.x,
        "m" => &mut state.m,
        "a" => &mut state.a,
        "s" => &mut state.s,
        _ => panic!("uh oh"),
    }
}

fn eval_cond(state: &State, c: &Condition) -> bool {
    let op1 = eval_op(state, &c.op1);
    let op2 = c.op2.parse::<usize>().unwrap();

    match c.compare_op {
        '>' => op1 > op2,
        '<' => op1 < op2,
        _ => panic!("unknown operator"),
    }
}

fn eval_cond_symbolic(state: &mut SymbolicState, c: &Condition) {
    let op1 = eval_op_symbolic(state, &c.op1);

    // op2 is always an int
    let op2 = c.op2.parse::<usize>().unwrap();

    match c.compare_op {
        '>' => {
            let true_range = (op2 + 1)..=(*op1.end());
            let true_range = range_intersection(&true_range, op1);
            *op1 = true_range;
        }
        '<' => {
            let true_range = *op1.start()..=(op2 - 1);
            let true_range = range_intersection(&true_range, op1);
            *op1 = true_range;
        }
        _ => panic!("unknown operator"),
    }
}

fn run_state(rules: &Vec<Rule>, state: &State) -> char {
    let mut rule = rules.iter().find(|r| r.name == "in").unwrap();

    let result = loop {
        let rule_out = rule
            .subrules
            .iter()
            .find_map(|(cond, dest)| {
                if eval_cond(state, cond) {
                    Some(dest)
                } else {
                    None
                }
            })
            .unwrap_or(&rule.default);

        dbg!(&rule_out);

        match rule_out.as_str() {
            o @ ("A" | "R") => break o,
            rule_name => rule = rules.iter().find(|r| r.name == rule_name).unwrap(),
        }
    };

    // dbg!(result);

    result.chars().next().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct SymbolicState {
    x: RangeInclusive<usize>,
    m: RangeInclusive<usize>,
    a: RangeInclusive<usize>,
    s: RangeInclusive<usize>,
    rules: Vec<String>,
    pos_conds: Vec<Condition>,
    //neg_conds: Vec<Condition>,
}

impl SymbolicState {

    fn update_symbolic(&mut self) {

        let conds = self.pos_conds.clone();

        for c in conds {
            eval_cond_symbolic(self, &c)
        }
    }

    fn succ(&self, rules: &Vec<Rule>) -> Vec<Self> {
        //println!("HI");

        match self.rules.last().unwrap().as_str() {
            "A" | "R" => return vec![],
            _ => (),
        }

        let current_rule = rules
            .iter()
            .find(|r| r.name == *self.rules.last().unwrap())
            .unwrap();

        let mut old_pos_conds = vec![];
        let mut states = vec![];

        for (c, nextrule) in &current_rule.subrules {
            //let (true_range, false_range) = eval_cond_symbolic(self, c);

            let mut new_rules = self.rules.clone();
            new_rules.push(nextrule.to_owned());

            let mut pos_conds = old_pos_conds.clone();
            pos_conds.push(c.to_owned());

            dbg!(new_rules.len());
            let mut new_st = SymbolicState {
                rules: new_rules,
                pos_conds,
                //neg_conds: neg_conds.clone(),
                ..(*self).clone()
            };
            new_st.update_symbolic();

            states.push(new_st);

            // Push not c because anybody after this point gets the negated condition
            old_pos_conds.push(c.negate());
        }

        // DEFAULT STATE
        let mut new_rules = self.rules.clone();
        new_rules.push(current_rule.default.clone());
        let mut new_st = SymbolicState {
            rules: new_rules,
            pos_conds: old_pos_conds,
            //neg_conds,
            ..(*self).clone()
        };
        new_st.update_symbolic();
        states.push(new_st);

        println!("I am returning {} states", states.len());

        states
    }
}

fn part2(rules: &Vec<Rule>) {
    let start = SymbolicState {
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
        pos_conds: vec![],
        //neg_conds: vec![],
        rules: vec!["in".to_owned()],
    };

    let accepted = bfs_reach(start, |succ| succ.succ(rules))
        .filter(|s| dbg!(s).rules.last().unwrap() == "A")
        .map(|s| {
            let mut s = s.clone();
            //let mut neg_conds = s.neg_conds.iter().map(|c| c.negate());

            let conds = &s.pos_conds.clone();

            for c in conds {
                eval_cond_symbolic(&mut s, c)
            }

            s
            //s.x.count() * s.m.count() * s.a.count() * s.s.count()
        })
        .map(|s| s.x.count() * s.m.count() * s.a.count() * s.s.count())
        .sum::<usize>();
        //.collect_vec();

    dbg!(&accepted);
}

fn main() {
    let (rules, states) = parse(std::io::stdin());

    dbg!(&rules);
    dbg!(&states);

    let p1 = states
        .iter()
        .filter_map(|st| {
            if run_state(&rules, st) == 'A' {
                Some(st.sum())
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("p1 {p1}");

    let p2 = part2(&rules);

    /*    let thresholds = rules
        .iter()
        .flat_map(|r| {
            r.subrules
                .iter()
                .flat_map(|(c, _)| {
                    [c.op1.to_owned(), c.op2.to_owned()]
        })
        .filter_map(|s| s.parse::<usize>().ok())
        .flat_map(|t| [t - 1, t, t + 1])
        .collect_vec();
    println!("thresholds: {:?}", thresholds);
    */
}
