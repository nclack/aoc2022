use std::{cell::RefCell, collections::HashMap, rc::Rc};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Node {
    score: i64,
    edges: (usize, usize),
}

#[derive(Debug, Default)]
struct Graph {
    nodes: HashMap<usize, Node>,
    edges: Vec<usize>,
    start: usize,
}

impl Graph {
    fn edges(&self, node: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        let n = &self.nodes[&node];
        Box::new((n.edges.0..n.edges.1).map(|e| self.edges[e]).into_iter())
    }

    fn to_dot_file(&self) {
        std::fs::write(
            "day16.dot",
            "graph {\n".to_owned()
                + &self
                    .nodes
                    .iter()
                    .map(|(k, v)| {
                        let score = v.score;
                        let is_start = if *k == self.start { "*" } else { "" };
                        format!("{k} [label=\"{is_start}{k}/{score}\"]\n")
                    })
                    .reduce(|acc, s| acc + &s)
                    .unwrap()
                + &self
                    .nodes
                    .iter()
                    .map(|(k, v)| {
                        (v.edges.0..v.edges.1)
                            .map(|i| format!("{k} -- {}\n", self.edges[i]))
                            .reduce(|acc, s| acc + &s)
                            .unwrap()
                    })
                    .reduce(|acc, s| acc + &s)
                    .unwrap()
                + "}",
        )
        .unwrap();
    }
}

#[derive(Debug)]
struct Observation {
    node: usize,
    score: i64,
    others: Vec<usize>,
}

struct Doc {
    obs: Vec<Observation>,
    start: usize,
}

fn parse(input: &str) -> IResult<&str, Doc> {
    #[derive(Default)]
    struct State<'a> {
        names: HashMap<&'a str, usize>,
        count: usize,
    }
    let state = Rc::new(RefCell::new(State::default()));

    fn node_id<'a>(state: Rc<RefCell<State<'a>>>, name: &'a str) -> usize {
        let mut state = state.borrow_mut();
        let mut count = state.count;
        let id = *state.names.entry(name).or_insert_with(|| {
            count += 1;
            count - 1
        });
        state.count = count;
        id
    }

    let number = map_res(digit1, |d| i64::from_str_radix(d, 10));
    let subject = preceded(
        tag("Valve "),
        map(alpha1, |name| node_id(state.clone(), name)),
    );
    let flow = preceded(tag(" has flow rate="), number);
    let others = preceded(
        alt((
            tag("; tunnels lead to valves "),
            tag("; tunnel leads to valve "),
        )),
        separated_list1(tag(", "), map(alpha1, |name| node_id(state.clone(), name))),
    );

    let (rest, obs) = separated_list1(
        line_ending,
        map(tuple((subject, flow, others)), |(node, score, others)| {
            Observation {
                node,
                score,
                others,
            }
        }),
    )(input)?;

    dbg!(&state.borrow().names);

    Ok((
        rest,
        Doc {
            obs,
            start: node_id(state, "AA"),
        },
    ))
}

fn build(doc: Doc) -> Graph {
    let mut graph = Graph {
        start: doc.start,
        ..Graph::default()
    };
    for Observation {
        node,
        score,
        mut others,
    } in doc.obs
    {
        let e = graph.edges.len();
        graph.nodes.insert(
            node,
            Node {
                score,
                edges: (e, e + others.len()),
            },
        );
        graph.edges.append(&mut others);
    }
    graph
}

pub(crate) fn part1(input: &str) -> i64 {
    let graph = {
        let (_rest, doc) = parse(input).unwrap();
        // assert_eq!(_rest.len(),0,"rest: {_rest:?}");
        build(doc)
    };
    graph.to_dot_file();

    #[derive(Default, Debug, Clone, Copy)]
    struct State {
        score: i64,
        valves: u64,
        total_score: i64,
    }

    const IMPOSSIBLE: i64 = -1 << 30;
    let mut state = vec![
        State {
            score: IMPOSSIBLE,
            valves: graph
                .nodes
                .iter()
                // go ahead and open 0-score valves
                .map(|(i, n)| if n.score == 0 { 1 << i } else { 0 })
                .reduce(|acc, f| acc | f)
                .unwrap(),
            ..State::default()
        };
        graph.nodes.len()
    ];
    state[graph.start].score = 0;

    for t in 1..=35 {
        state = (0..graph.nodes.len())
            .map(|inode| {
                // accumulate flow
                let total_score = state[inode].total_score + state[inode].score.max(0);
                // best score for this node at time t is:
                // max([moves from edge nodes, stay at current and open valve])
                // can only open valve if it's reachable at time t
                let reachable = state[inode].score >= 0;
                let can_open = (state[inode].valves >> inode) & 1 == 0;
                let open_valve_score = if reachable && can_open {
                    graph.nodes[&inode].score
                } else {
                    0
                } + state[inode].score;
                let (best_incoming_score, argmax) = graph
                    .edges(inode)
                    .map(|enode| (state[enode].score, enode))
                    .max()
                    .unwrap();

                let (valves, score) = if open_valve_score >= best_incoming_score {
                    // mark as opened on this path
                    (state[inode].valves | 1 << inode, open_valve_score)
                } else {
                    // transfer valve state for best path
                    (state[argmax].valves, best_incoming_score)
                };
                State {
                    score,
                    valves,
                    total_score,
                }
            })
            .collect();
        {
            let State {
                score,
                valves,
                total_score,
            } = state.iter().max_by_key(|s| (s.total_score,s.score)).unwrap();
            println!("{t:3} {score:5} {total_score:5} {valves:#064b}");
        }
    }
    state.into_iter().map(|s| s.total_score).max().unwrap()
}

#[test]
fn day16() {
    assert_eq!(1651, part1(include_str!("../assets/day16.test.txt")));
    // assert_eq!(93, part2(include_str!("../assets/day16.test.txt")));
}
