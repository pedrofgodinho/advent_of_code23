use std::collections::HashMap;

use super::Solution;

#[ouroboros::self_referencing]
pub struct Day8 {
    input: String,
    #[covariant]
    #[borrows(input)]
    path: PathSequence<'this>,
    #[covariant]
    #[borrows(input)]
    node_map: HashMap<&'this str, Node<'this>>,
    #[covariant]
    #[borrows(node_map)]
    starting_nodes: Vec<&'this str>,
}

impl Solution for Day8 {
    fn part1(&mut self) -> String {
        self.borrow_path()
            .solve_node_map(self.borrow_node_map(), "AAA")
            .to_string()
    }

    fn part2(&mut self) -> String {
        self.borrow_starting_nodes()
            .iter()
            .map(|node| {
                self.borrow_path()
                    .solve_node_map(self.borrow_node_map(), node)
            })
            .fold(1, |accum, n| num::integer::lcm(n, accum))
            .to_string()
    }

    fn parse(input: String) -> Box<dyn Solution> {
        Box::new(
            Day8Builder {
                input,
                path_builder: |input: &String| {
                    PathSequence::from_str(input.lines().next().unwrap())
                },
                node_map_builder: |input: &String| {
                    input
                        .lines()
                        .skip(2)
                        .map(|line| {
                            let node = Node::from_str(line);
                            (node.name, node)
                        })
                        .collect()
                },
                starting_nodes_builder: |node_map| {
                    node_map
                        .keys()
                        .filter_map(|key| if key.ends_with('A') { Some(*key) } else { None })
                        .collect()
                },
            }
            .build(),
        )
    }
}

#[derive(Debug)]
struct PathSequence<'a> {
    sequence: &'a str,
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> PathSequence<'a> {
    fn from_str(str: &'a str) -> Self {
        Self { sequence: str }
    }

    fn solve_node_map(&self, nodes: &HashMap<&str, Node>, starting_node_name: &str) -> usize {
        let mut count = 0;
        let mut current_node = nodes.get(starting_node_name).unwrap();
        for direction in self.sequence.bytes().cycle() {
            match direction {
                b'R' => current_node = nodes.get(current_node.right).unwrap(),
                b'L' => current_node = nodes.get(current_node.left).unwrap(),
                _ => panic!(),
            }
            count += 1;
            if current_node.name.ends_with('Z') {
                break;
            }
        }
        count
    }
}

impl<'a> Node<'a> {
    fn from_str(str: &'a str) -> Self {
        let mut parts = str.split(" = ");

        let name = parts.next().unwrap();

        let mut sides = parts.next().unwrap().split(", ");

        let left = &sides.next().unwrap()[1..];
        let right = sides.next().unwrap();

        let right_len = right.len();
        let right = &right[..right_len - 1];

        Self { name, right, left }
    }
}
