#![allow(unused_imports, unused_variables)]
use super::prelude::*;
type Input = Node;

pub struct Node {
    childs: Vec<Node>,
    metadata: Vec<u32>,
}

pub fn input_generator(input: &str) -> Input {
    fn parse_node(iter: &mut impl Iterator<Item = u32>) -> Node {
        let child_count = iter.next().expect("Invalid input");
        let metadata_count = iter.next().expect("Invalid input");

        let childs = (0..child_count)
            .map(|_| parse_node(iter))
            .collect();
        let metadata = (0..metadata_count)
            .map(|_| iter.next().expect("Invalid input"))
            .collect();

        Node { childs, metadata }
    }

    parse_node(
        &mut input
            .split_whitespace()
            .map(|p| p.parse().expect("Invalid input")),
    )
}

pub fn part1(input: &Input) -> u32 {
    fn sum_metadata(node: &Node) -> u32 {
        node.metadata.iter().sum::<u32>() + node.childs.iter().map(sum_metadata).sum::<u32>()
    }

    sum_metadata(input)
}

pub fn part2(input: &Input) -> u32 {
    fn value(node: &Node) -> u32 {
        if node.childs.len() == 0 {
            node.metadata.iter().sum()
        } else {
            node.metadata
                .iter()
                .filter_map(|i| node.childs.get((*i - 1) as usize).map(value))
                .sum()
        }
    }

    value(input)
}
