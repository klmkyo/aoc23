// warning: this code is needlessly overcomplicated, unorganized, and unreadable
// it does work though

use std::{cell::RefCell, collections::HashMap, fmt::Debug, fs, rc::Rc};

use num::Integer;

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    let directions: Vec<Direction> = file
        .lines()
        .next()
        .unwrap()
        .trim()
        .chars()
        .map(Direction::from)
        .collect();

    let node_ids: Vec<NodeId> = file
        .lines()
        .skip(2)
        .map(|line: &str| line.trim().chars().collect::<Vec<char>>())
        .map(|chars| chars[..3].try_into().unwrap())
        .collect();

    let empty_nodes_map: HashMap<NodeId, Rc<RefCell<Node>>> = node_ids
        .iter()
        .map(|id| (*id, Rc::new(RefCell::new(Node::new(*id)))))
        .collect();

    file.lines().skip(2).for_each(|line| {
        let (node_id, links) = line.split_once(" = ").unwrap();

        let node_id: NodeId = str_to_nodeid(node_id);

        let (left, right) = links[1..links.len() - 1].split_once(", ").unwrap();

        let left: NodeId = str_to_nodeid(left);
        let right: NodeId = str_to_nodeid(right);

        let node = empty_nodes_map.get(&node_id).unwrap();

        let left_node = empty_nodes_map.get(&left).unwrap();
        let right_node = empty_nodes_map.get(&right).unwrap();

        node.borrow_mut().set_left(left_node.clone());
        node.borrow_mut().set_right(right_node.clone());
    });

    let nodes_map = empty_nodes_map;
    // print all nodes
    nodes_map
        .iter()
        .for_each(|(_, node)| println!("{:?}", node.borrow()));

    // print all node ids
    node_ids
        .iter()
        .for_each(|id| println!("{}{}{}", id[0], id[1], id[2]));

    let infinite_directions = directions.iter().cycle();

    // part 1 solution
    // let first_node = nodes_map.get(&START_NODE).unwrap().clone();
    // let mut curr_node = first_node;
    // for (i, direction) in infinite_directions.enumerate() {
    //     let new_node = match direction {
    //         Direction::Left => curr_node.borrow().left.clone().unwrap(),
    //         Direction::Right => curr_node.borrow().right.clone().unwrap(),
    //     };

    //     let new_node_val  = new_node.borrow().value;
    //     println!("{:?} {} -> {}", direction, curr_node.borrow().value.iter().collect::<String>(), new_node_val.iter().collect::<String>());

    //     if new_node_val == WIN_NODE {
    //         println!("Won in {} steps", i + 1);
    //         break;
    //     }

    //     curr_node = new_node;
    // }

    // stuff part 2
    let start_node_ids: Vec<NodeId> = node_ids
        .clone()
        .into_iter()
        .filter(|x| x[2] == 'A')
        .collect();

    println!("Start nodes: {:?}", start_node_ids);

    let curr_nodes = start_node_ids
        .iter()
        .map(|id| nodes_map.get(id).unwrap().clone())
        .collect::<Vec<Rc<RefCell<Node>>>>();

    // since the elements are going to end up in a cycle, we can find the
    // amount needed by finding the lcm of all the lowest iteration counts
    let iterations_needed: Vec<u32> = curr_nodes
        .iter()
        .map(|node| {
            let mut curr_node = node.clone();
            for (i, direction) in infinite_directions.clone().enumerate() {
                let new_node = match direction {
                    Direction::Left => curr_node.borrow().left.clone().unwrap(),
                    Direction::Right => curr_node.borrow().right.clone().unwrap(),
                };

                let new_node_val = new_node.borrow().value;

                // println!(
                //     "{:?} {} -> {}",
                //     direction,
                //     curr_node.borrow().value.iter().collect::<String>(),
                //     new_node_val.iter().collect::<String>()
                // );

                if new_node_val[2] == 'Z' {
                    return i as u32 + 1;
                }

                curr_node = new_node;
            }

            return 0;
        })
        .collect();

    let result: u64 = iterations_needed
        .iter()
        .fold(iterations_needed[0] as u64, |acc, &x| acc.lcm(&(x as u64)));

    println!("Iterations needed: {:?}", iterations_needed);
    println!("result: {}", result);
}

type NodeId = [char; 3];

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    value: NodeId,
}

impl Node {
    fn new(value: NodeId) -> Self {
        Node {
            left: None,
            right: None,
            value,
        }
    }

    fn set_left(&mut self, node: Rc<RefCell<Node>>) {
        self.left = Some(node);
    }

    fn set_right(&mut self, node: Rc<RefCell<Node>>) {
        self.right = Some(node);
    }
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let left = match &self.left {
            Some(node) => node.borrow().value,
            None => [' ', ' ', ' '],
        };

        let right = match &self.right {
            Some(node) => node.borrow().value,
            None => [' ', ' ', ' '],
        };

        write!(
            f,
            "{} = ({}, {})",
            self.value.iter().collect::<String>(),
            left.iter().collect::<String>(),
            right.iter().collect::<String>()
        )
    }
}

fn str_to_nodeid(s: &str) -> NodeId {
    let chars: Vec<char> = s.chars().take(3).collect();
    chars.try_into().expect("Invalid NodeId")
}

const START_NODE: NodeId = ['A', 'A', 'A'];
const WIN_NODE: NodeId = ['Z', 'Z', 'Z'];
