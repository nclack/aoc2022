use std::{
    cell::RefCell,
    cmp::Reverse,
    collections::HashMap,
    ops::{Index, IndexMut},
    rc::Rc,
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};
use num_traits::ToPrimitive;

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

struct Mat<T> {
    inner: Vec<T>,
    stride: usize,
    shape: (usize, usize),
}

impl<T: Clone> Mat<T> {
    fn new(rows: usize, cols: usize, val: T) -> Self {
        Mat {
            inner: vec![val; rows * cols],
            stride: cols,
            shape: (rows, cols),
        }
    }
}

impl<T> Index<(usize, usize)> for Mat<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.inner[col + row * self.stride]
    }
}

impl<T> IndexMut<(usize, usize)> for Mat<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.inner[col + row * self.stride]
    }
}

impl<T> IntoIterator for Mat<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<T: ToPrimitive + Copy> Mat<T> {
    fn plot<P: AsRef<std::path::Path>>(&self, filename: P, title: &str) {
        use plotters::prelude::*;
        let root = BitMapBackend::new(&filename, (1024, 768)).into_drawing_area();

        root.fill(&WHITE).unwrap();

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30))
            .margin(5)
            .top_x_label_area_size(40)
            .y_label_area_size(40)
            .build_cartesian_2d(0..self.shape.0 as i32, self.shape.1 as i32..0)
            .unwrap();

        chart
            .configure_mesh()
            .x_labels(self.shape.1)
            .y_labels(self.shape.0)
            .max_light_lines(4)
            .x_label_offset(35)
            .y_label_offset(25)
            .disable_x_mesh()
            .disable_y_mesh()
            .label_style(("sans-serif", 20))
            .draw()
            .unwrap();

        chart
            .draw_series(
                (0..self.shape.0)
                    .cartesian_product(0..self.shape.1)
                    .map(|(x, y)| {
                        let v = self[(y, x)];
                        let (x, y) = (x as i32, y as i32);
                        Rectangle::new(
                            [(x, y), (x + 1, y + 1)],
                            HSLColor(
                                240.0 / 360.0 - 240.0 / 360.0 * (v.to_f64().unwrap() / 20.0),
                                0.7,
                                0.1 + 0.4 * v.to_f64().unwrap() / 20.0,
                            )
                            .filled(),
                        )
                    }),
            )
            .unwrap();

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {:?}", filename.as_ref());
    }
}

pub(crate) fn part1(input: &str) -> i64 {
    let graph = {
        let (_rest, doc) = parse(input).unwrap();
        // assert_eq!(_rest.len(),0,"rest: {_rest:?}");
        build(doc)
    };
    graph.to_dot_file();

    let n = graph.nodes.len();

    //
    // all pairs shortest paths
    // next([src,dst]) will be the next node on the shortest path from src to dst
    //

    let (dt, next) = {
        let mut score = Mat::new(n, n, 1 << 10);
        let mut next = Mat::new(n, n, n << 2);

        for i in 0..n {
            score[(i, i)] = 0;
            next[(i, i)] = i;
            for j in graph.edges(i) {
                score[(i, j)] = 1;
                next[(i, j)] = j;
            }
        }

        for k in 0..n {
            for src in 0..n {
                for dst in 0..n {
                    if score[(src, dst)] > score[(src, k)] + score[(k, dst)] {
                        score[(src, dst)] = score[(src, k)] + score[(k, dst)];
                        next[(src, dst)] = next[(src, k)];
                    }
                }
            }
        }
        #[cfg(feature = "debug")]
        {
            score.plot("dt.png", "Time");
            next.plot("next.png", "Next");
        }
        (score, next)
    };

    // dfs for best path

    // Given some previous part of the path, find the best continuation

    let valves: Vec<_> = graph
        .nodes
        .iter()
        .filter(|&(_, v)| v.score > 0)
        .sorted_by_key(|(_, v)| Reverse(v.score))
        .map(|(k, _)| *k)
        .collect();

    let mut path = vec![graph.start];
    let mut used = 0u64;
    let mut flow = 0;
    let mut total = 0;

    /// compute the sum of the shortest edges from each remaining valve
    fn lower_bound_time(dt: &Mat<i32>, valves: &[usize], used: u64) -> i32 {
        valves
            .iter()
            .enumerate()
            .filter(|(_, &v)| (used >> v) & 1 == 0)
            .map(|(i, &src)| {
                // min over unused nodes k in valves s.t. i<j
                valves
                    .iter()
                    .enumerate()
                    .filter(|(_, &dst)| (used >> dst) & 1 == 0)
                    .filter(|&(j, _)| i < j)
                    .map(|(_, &dst)| dt[(src, dst)])
                    .min()
                    .unwrap()
            })
            .sum()
    }

    /// assuming valves is ordered in descending order of flow, compute the
    /// total score as if each was visited in turn and separated by dt
    /// where dt is the minimum number of steps between the unused valves
    fn upper_bound_score(graph: Graph, dt: &Mat<i32>, valves: &[usize], used: u64) -> i64 {
        let unused: Vec<_> = valves.iter().filter(|&v| (used >> v) & 1 == 0).collect();
        let dt_min = &unused[..]
            .into_iter()
            .cartesian_product(&unused[..])
            .map(|(&src, &dst)| dt[(*src, *dst)])
            .min()
            .unwrap();
        unused
            .into_iter()
            .map(|v| graph.nodes[v].score)
            .fold((0, 0), |(flow, total), f| (flow + f, total + flow))
            .1
    }

    let mut best_score=0;

    for choice in 0..valves.len() {
        let v={
            
        };
        path[choice] = v;
    }

    todo!()
}

#[test]
fn day16() {
    assert_eq!(1651, part1(include_str!("../assets/day16.test.txt")));
    // assert_eq!(93, part2(include_str!("../assets/day16.test.txt")));
}
