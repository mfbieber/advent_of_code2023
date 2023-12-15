use std::collections::HashMap;
use std::path::PathBuf;
use std::str::Chars;
use std::vec;
use crate::read_lines;

#[derive(PartialEq, Debug, Clone)]
struct Node {
    label: String,
    left: String,
    right: String,
}

impl Node {
    fn new(label: String, left: String, right: String) -> Node {
        return Node {
            label,
            left,
            right,
        }
    }

    fn last() -> Node {
        return Node {
            label: String::from("ZZZ"),
            left: String::from("ZZZ"),
            right: String::from("ZZZ"),
        }
    }
}

fn day8_part2(path: &PathBuf, instructions: Vec<String>) -> i64 {
    let mut nodes: HashMap<String,Node> = HashMap::new();
    read_network(path, &mut nodes,);

    let start_nodes: Vec<&String> = nodes.keys()
        .filter(|node| node.ends_with("A"))
        .collect::<Vec<&String>>();

    return count_steps_for_path_part2(start_nodes, String::from("Z"), &instructions, &nodes);
}

fn day8_part1(path: &PathBuf, instructions: Vec<String>) -> i32 {
    let mut nodes: HashMap<String,Node> = HashMap::new();
    read_network(path, &mut nodes,);
    return count_steps_for_path(String::from("AAA"), String::from("ZZZ"), &instructions, &nodes);
}

fn count_steps_for_path_part2(start_nodes: Vec<&String>, end_node_match: String, instructions: &Vec<String>,
                              nodes: &HashMap<String,Node>) -> i64 {
    let mut steps: i32 = 0;
    let mut current_nodes: Vec<&String> = start_nodes;
    let iter_length: usize = instructions.len() - 1;
    let mut i: usize = 0;
    let mut paths: Vec<i32> = current_nodes.iter().map(|node| 0).collect();
    //paths cycle (debug observation), looking for LCM of the path's lengths
    while paths.iter().any(|path_length| *path_length == 0) {
        let mut node_number: i32 = 0;
        current_nodes = current_nodes.iter().map(|node| {
            let current_node: &Node = nodes.get(&*node.clone()).unwrap();
            if current_node.label.ends_with("Z") {
                paths[node_number as usize] = steps;
            }
            node_number += 1;
            let instruction: String = instructions[i].clone();
            return if instruction == "L" {
                &current_node.left
            } else {
                &current_node.right
            }
        }).collect::<Vec<&String>>();
        node_number = 0;
        steps += 1;
        if i == iter_length {
            i = 0;
        } else {
            i += 1;
        }
    }
    let mut lcm: i64 = 1;
    paths.iter().for_each(|path| {
        lcm = num::integer::lcm(lcm, *path as i64);
    });
    println!("{}", lcm);
    return lcm;
}

fn count_steps_for_path(start_node_match: String, end_node_match: String, instructions: &Vec<String>,
                        nodes: &HashMap<String,Node>) -> i32 {
    let mut steps: i32 = 0;
    let iter_length: usize = instructions.len() - 1;
    let mut i: usize = 0;
    let mut next_node: String = start_node_match;
    loop {
        let current_node: &Node = nodes.get(&*next_node.clone()).unwrap();
        if current_node.label.ends_with(end_node_match.as_str()) {
            break;
        }
        steps += 1;
        let instruction: String = instructions[i].clone();
        if instruction == "L" {
            next_node = current_node.left.clone();
        } else {
            next_node = current_node.right.clone();
        }
        if i == iter_length {
            i = 0;
        } else {
            i += 1;
        }
    };
    println!("{}", steps);
    return steps;
}

fn read_network(path: &PathBuf, mut nodes: &mut HashMap<String, Node>){
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            let split: Vec<&str> = line
                .as_ref()
                .unwrap()
                .split(" = (")
                .collect::<Vec<&str>>();
            let split_2: Vec<&str> = split[1]
                .split(", ")
                .collect::<Vec<&str>>();
            let node_label: String = String::from(split[0]);
            let left_label: String = String::from(split_2[0]);
            let right_label: String = split_2[1].replace(")", "");
            let node: Node = Node::new(node_label.clone(), left_label, right_label);
            nodes.insert(node_label.clone(), node);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::PathBuf;
    use crate::day8::{day8_part1, day8_part2, Node, read_network};

    #[test]
    fn test_reads_nodes_correctly() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/test/input.txt");
        let mut nodes: HashMap<String,Node> = HashMap::new();
        read_network(&d, &mut nodes);
        let mut expected_nodes: HashMap<String,Node> = HashMap::new();
        expected_nodes.insert(
            String::from("AAA"),
            Node::new(String::from("AAA"), String::from("BBB"), String::from("CCC"))
        );
        expected_nodes.insert(
            String::from("BBB"),
            Node::new(String::from("BBB"), String::from("DDD"), String::from("EEE"))
        );
        expected_nodes.insert(
            String::from("CCC"),
            Node::new(String::from("CCC"), String::from("ZZZ"), String::from("GGG"))
        );
        expected_nodes.insert(
            String::from("DDD"),
            Node::new(String::from("DDD"), String::from("DDD"), String::from("DDD"))
        );
        expected_nodes.insert(
            String::from("EEE"),
            Node::new(String::from("EEE"), String::from("EEE"), String::from("EEE"))
        );
        expected_nodes.insert(
            String::from("GGG"),
            Node::new(String::from("GGG"), String::from("GGG"), String::from("GGG"))
        );
        expected_nodes.insert(
            String::from("ZZZ"),
            Node::new(String::from("ZZZ"), String::from("ZZZ"), String::from("ZZZ"))
        );
        assert_eq!(expected_nodes.clone(), nodes.clone());
    }

    #[test]
    fn test_part1_first_example() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/test/input.txt");
        let instructions: Vec<String> = String::from("RL").chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let steps: i32 = day8_part1(&d, instructions);
       assert_eq!(steps, 2);
    }

    #[test]
    fn test_part1_second_example() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/test/input1.txt");
        let instructions: Vec<String> = String::from("LLR").chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let steps: i32 = day8_part1(&d, instructions);
        assert_eq!(steps, 6);
    }

    #[test]
    fn part1() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/input.txt");
        let instructions: Vec<String> = String::from("LLRLRLRRRLRLRRRLRRRLRRLLRLLRRRLRLRRLLRLRLRRLRLRLLRLRRRLRLRRLRRLRRRLRRLRRLRRLLRRLLRRRLRRLRRLRRRLRLRRLRRLLLLRLRRLRLRRLLLRRLRRRLRRRLLRRRLRRRLRRLRRRLLLRRRLLLRRLRRLRRRLRRLRRRLRRLRRRLLRLRLRRRLRRLRLRLRRRLRLRLLLRRRLRRRLRRLRRLRLRRRLRRRLLRRRLRRLRLLLRRLLRRRLRRRLRRRLLRRRLLRRLRLRRRLRRLRRRR").chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let steps: i32 = day8_part1(&d, instructions);
        assert_eq!(steps, 16343);
    }

    #[test]
    fn test_part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/test/input2.txt");
        let instructions: Vec<String> = String::from("LR").chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let steps: i64 = day8_part2(&d, instructions);
        assert_eq!(steps, 6);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/input.txt");
        let instructions: Vec<String> = String::from("LLRLRLRRRLRLRRRLRRRLRRLLRLLRRRLRLRRLLRLRLRRLRLRLLRLRRRLRLRRLRRLRRRLRRLRRLRRLLRRLLRRRLRRLRRLRRRLRLRRLRRLLLLRLRRLRLRRLLLRRLRRRLRRRLLRRRLRRRLRRLRRRLLLRRRLLLRRLRRLRRRLRRLRRRLRRLRRRLLRLRLRRRLRRLRLRLRRRLRLRLLLRRRLRRRLRRLRRLRLRRRLRRRLLRRRLRRLRLLLRRLLRRRLRRRLRRRLLRRRLLRRLRLRRRLRRLRRRR").chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        let steps: i64 = day8_part2(&d, instructions);
        assert_eq!(steps, 15299095336639);
    }

}