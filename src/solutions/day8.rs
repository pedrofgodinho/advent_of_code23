use std::collections::HashMap;

use super::Solution;

pub struct Day8 {}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> String {
        let mut lines = input.lines();
        let path = Path::from_str(lines.next().unwrap());

        let nodes = lines
            .skip(1)
            .map(|line| {
                let node = Node::from_str(line);
                (node.name, node)
            })
            .collect::<HashMap<_, _>>();

        path.solve_node_map(&nodes, "AAA").to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut lines = input.lines();
        let path = Path::from_str(lines.next().unwrap());

        let mut starting_nodes = Vec::new();

        let nodes = lines
            .skip(1)
            .map(|line| {
                let node = Node::from_str(line);
                if node.name.ends_with('A') {
                    starting_nodes.push(node.name);
                }
                (node.name, node)
            })
            .collect::<HashMap<_, _>>();

        starting_nodes
            .iter()
            .map(|node| path.solve_node_map(&nodes, node))
            .fold(1, |accum, n| num::integer::lcm(n, accum))
            .to_string()
    }

    fn parse(&mut self) {}
}

impl Day8 {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Path<'a> {
    sequence: &'a str,
}

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

impl<'a> Path<'a> {
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
