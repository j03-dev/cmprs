/*
 * Codage de huffman by joe
 */
mod node;

use crate::node::Node;

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn file_reader(path: &str) -> String {
    let mut output = String::new();
    let mut file = File::open(path).expect("File not found");
    file.read_to_string(&mut output)
        .expect("Failed to read to string the file");
    output
}

fn find_most_occurences(msg: String) -> Vec<Node> {
    let mut letter_occurences: HashMap<String, i32> = HashMap::new();

    for l in msg.chars() {
        let value = letter_occurences.get(&l.to_string()).unwrap_or(&0) + 1;
        letter_occurences.insert(l.to_string(), value);
    }

    let mut nodes = letter_occurences
        .iter()
        .map(|(key, value)| Node {
            data: Some(key.clone()),
            occurence: *value,
            ..Default::default()
        })
        .collect::<Vec<_>>();
    nodes.sort_by(|a, b| a.occurence.cmp(&b.occurence));
    nodes
}

fn make_huffman_tree(nodes: Vec<Node>) -> Node {
    let mut tree: Option<Node> = None;
    let mut i = 0;
    while i < nodes.len() {
        let left = nodes.get(i);
        let right = nodes.get(i + 1);

        let new_node = match (left, right) {
            (Some(l), Some(r)) => l.add(Box::new(r.clone())),
            _ => left.unwrap().clone(),
        };

        tree = if let Some(node) = tree {
            Some(node.add(Box::new(new_node)))
        } else {
            Some(new_node)
        };

        i += 2;
    }
    tree.unwrap_or_default()
}

fn main() {
    let mut out = file_reader("/mnt/d/Project/cmprs/message.txt");
    let nodes = find_most_occurences(out.clone());
    let tree = make_huffman_tree(nodes.clone());

    for n in nodes.iter() {
        let target = n.data.clone().unwrap();
        let code = tree
            .search(&target, &mut vec![])
            .unwrap()
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join("");
        out = out.replace(&target, &code);
    }
    println!("{out}");
}
