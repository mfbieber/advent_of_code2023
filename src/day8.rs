use std::collections::HashMap;
use std::path::PathBuf;
use std::str::Chars;
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

fn day8_part2(path: &PathBuf) -> i32 {
    let mut sum: i32 = 0;
    // File input.txt must exist in the current path
    if let Ok(lines) = read_lines(path) {
        for line in lines {

        }
    }
    println!("{}", sum);
    return sum;
}

fn day8_part1(path: &PathBuf, instructions: Vec<String>) -> i32 {
    let mut steps: i32 = 0;
    let mut nodes: HashMap<String,Node> = HashMap::new();
    read_network(path, &mut nodes,);

    let iter_length: usize = instructions.len() - 1;
    let mut i: usize = 0;
    let mut next_node: String = String::from("AAA");;
    loop {
        let current_node: &Node = nodes.get(&next_node).unwrap();
        if current_node.label == String::from("ZZZ") {
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
    }

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
    use crate::day8::{day8_part1, Node, read_network};

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
        let steps: i32 = day8_part1(&d, instructions);
        assert_eq!(steps, 6);
    }

    #[test]
    fn part2() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/day8/input.txt");
        //assert_eq!(day8_part2(&d), 281);
    }

}