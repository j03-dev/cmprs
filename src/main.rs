/*
 * Codage de huffman by joe
 */
mod node;

use node::Node;
use std::{
    collections::HashMap,
    env,
    fs::{write, File},
    io::{BufReader, BufWriter, Read, Write},
    process,
};

static OUTPUT: &'static str = "/mnt/d/Project/cmprs/output.bin";
static JSON_TREE: &'static str = "/mnt/d/Project/cmprs/tree.json";
static DECOMPRESS_PATH: &'static str = "/mnt/d/Project/cmprs/recompress.data";

fn file_reader(path: &str) -> Result<String, std::io::Error> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;
    Ok(content)
}

fn read_as_binary(input_path: &str) -> Result<Vec<u8>, std::io::Error> {
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn find_most_occurrences(msg: &str) -> Vec<Node> {
    let mut letter_occurrences: HashMap<char, i32> = HashMap::new();

    for l in msg.chars() {
        *letter_occurrences.entry(l).or_insert(0) += 1;
    }

    let mut nodes: Vec<_> = letter_occurrences
        .iter()
        .map(|(&key, &value)| Node {
            data: Some(key.to_string()),
            occurrence: value,
            ..Default::default()
        })
        .collect();
    nodes.sort_by_key(|node| node.occurrence);
    nodes
}

fn make_huffman_tree(mut nodes: Vec<Node>) -> Node {
    while nodes.len() > 1 {
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        let new_node = Node::add(left, right);
        nodes.push(new_node);
        nodes.sort_by_key(|node| node.occurrence);
    }

    nodes.pop().unwrap_or_default()
}

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

fn decode_binary(encoded_data: &[u8]) -> Vec<u8> {
    let mut decoded_bits = Vec::new();

    for &byte in encoded_data.iter() {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            decoded_bits.push(bit);
        }
    }

    decoded_bits
}

fn decode_text(decoded_bits: &[u8], tree: &Node) -> String {
    let mut decoded_text = String::new();
    let mut current_node = tree;

    for &bit in decoded_bits {
        match bit {
            0 => {
                if let Some(left) = &current_node.left {
                    current_node = left.as_ref();
                }
            }
            1 => {
                if let Some(right) = &current_node.right {
                    current_node = right.as_ref();
                }
            }
            _ => {
                panic!("Invalid bit encountered during decoding");
            }
        }

        if let Some(data) = &current_node.data {
            decoded_text.push_str(data);
            current_node = tree; // Reset to the root for next character
        }
    }

    decoded_text
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

fn save_tree(tree: &Node, output_path: &str) -> Result<(), std::io::Error> {
    let encoded_tree = serde_json::to_string(tree)?;
    let mut file = BufWriter::new(File::create(output_path)?);
    file.write_all(encoded_tree.as_bytes())?;
    Ok(())
}

fn load_tree(input_path: &str) -> Result<Node, std::io::Error> {
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let decoded_tree: Node = serde_json::from_reader(reader)?;
    Ok(decoded_tree)
}

fn compress(input_file_path: &str) {
    println!("Reading input file: {}", input_file_path);
    let text = match file_reader(input_file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };
    println!("Input file read successfully.");

    println!("Finding occurrences of characters in the text.");
    let nodes = find_most_occurrences(&text);
    println!("Occurrences found.");

    println!("Constructing Huffman tree.");
    let tree = make_huffman_tree(nodes);
    println!("Huffman tree constructed.");

    // Save the tree as JSON
    println!("Saving Huffman tree to file: {}", JSON_TREE);
    if let Err(err) = save_tree(&tree, JSON_TREE) {
        eprintln!("Error saving tree to file: {}", err);
        return;
    }
    println!("Huffman tree saved successfully.");

    println!("Encoding text using the Huffman tree.");
    let encoded_text = encode_text(&text, &tree);
    println!("Text encoded successfully.");

    println!("Writing encoded data to file: {}", OUTPUT);
    if let Err(err) = write_as_binary(&encoded_text, OUTPUT) {
        eprintln!("Error writing encoded data to file: {}", err);
    } else {
        println!("Encoded data written to file successfully.");
    }

    println!("Compression completed successfully.");
}

fn save_string_to_file(data: &str, file_path: &str) -> Result<(), std::io::Error> {
    println!("Saving data to file: {}", file_path);
    write(file_path, data)?;
    println!("Data saved successfully to file: {}", file_path);
    Ok(())
}

fn decompress(input_file: &str) {
    let tree = match load_tree(JSON_TREE) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error loading tree from file: {}", err);
            return;
        }
    };

    let encoded_data = match read_as_binary(input_file) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error reading input file: {}", err);
            return;
        }
    };

    let decoded_bits = decode_binary(&encoded_data);
    let output = decode_text(&decoded_bits, &tree);
    save_string_to_file(&output, DECOMPRESS_PATH).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <option> <input_file>", args[0]);
        process::exit(1);
    }

    let option = args[1].clone();
    let input_file = args[2].clone();

    match option.as_str() {
        "-c" | "--compress" => compress(&input_file),
        "-d" | "--decompress" => decompress(&input_file),
        _ => {
            eprintln!("Invalid option provided");
            process::exit(1);
        }
    }
}
