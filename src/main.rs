/*
 * Codage de huffman by joe
 */
mod node;
use node::Node;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    process,
};

fn file_reader(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

// Calculate the occurrences of each character in the input text
fn find_most_occurrences(msg: &str) -> Vec<Node> {
    let mut letter_occurrences: HashMap<char, i32> = HashMap::new();

    for l in msg.chars() {
        *letter_occurrences.entry(l).or_insert(0) += 1;
    }

    let mut nodes: Vec<_> = letter_occurrences
        .iter()
        .map(|(&key, &value)| Node {
            data: Some(key.to_string()),
            occurence: value,
            ..Default::default()
        })
        .collect();
    nodes.sort_by_key(|node| node.occurence);
    nodes
}

// Construct the Huffman tree from a sorted list of nodes
fn make_huffman_tree(nodes: Vec<Node>) -> Node {
    let mut nodes = nodes;

    while nodes.len() > 1 {
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        let new_node = Node::add(left, right);
        nodes.push(new_node);
        nodes.sort_by_key(|node| node.occurence);
    }

    nodes.pop().unwrap_or_default()
}

// Encode the input text using the Huffman tree
fn encode_text(text: &str, tree: &Node) -> String {
    let mut output = String::new();
    for char in text.chars() {
        let target = char.to_string();
        let code: String = tree
            .search(&target, &mut Vec::new())
            .unwrap()
            .iter()
            .map(|&b| b.to_string())
            .collect();
        output.push_str(&code);
    }
    output
}

fn write_as_binary(input: &str, output_path: &str) -> Result<(), std::io::Error> {
    let mut file = BufWriter::new(File::create(output_path)?);

    for chunk in input.chars().collect::<Vec<_>>().chunks(8) {
        let byte_str: String = chunk.iter().collect();
        let byte = u8::from_str_radix(&byte_str, 2).map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid character found in code",
            )
        })?;
        file.write_all(&[byte])?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file>", args[0]);
        process::exit(1);
    }

    let input_file_path = &args[1];
    let output_file_path = "/mnt/d/Project/cmprs/output.bin";

    println!("Reading input file...");
    let text = match file_reader(input_file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };

    println!("Finding occurrences...");
    let nodes = find_most_occurrences(&text);

    println!("Building Huffman tree...");
    let tree = make_huffman_tree(nodes);

    println!("Encoding text...");
    let encoded_text = encode_text(&text, &tree);

    println!("Writing encoded data to file...");
    if let Err(err) = write_as_binary(&encoded_text, output_file_path) {
        eprintln!("Error writing encoded data to file: {}", err);
        return;
    }

    println!("Compression completed successfully.");
}
