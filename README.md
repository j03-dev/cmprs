# Huffman Compression Algorithm Overview

This project implements the Huffman compression algorithm in Rust. Huffman coding is a widely used method for lossless data compression, particularly for compressing text files. It achieves compression by assigning variable-length codes to input characters, with shorter codes assigned to more frequently occurring characters.
## Features

- **Compression**: Compresses input text files using the Huffman coding algorithm.
- **Decompression**: Decompresses binary files generated by the compression process back to their original text format.
- **Tree Serialization**: The Huffman tree used for compression is serialized to JSON format and saved alongside the compressed file for decompression.

## Usage

Ensure the binary cmprs is available in your system path. Then, execute the program with the appropriate command-line options:  Compression: Run cmprs -c followed by the input file path to compress the text.
```bash
cmprs -c input.txt
```

Decompression: Run cmprs -d followed by the input file path to decompress the data.

```bash
cmprs -d output.bin
```
Make sure to replace input.txt and output.bin with the paths to your input and output files, respectively.

## Installation

To build the project from source, ensure you have Rust installed on your system. Then, clone the repository and navigate to the project directory. Run the following command to build the binary:

```bash
cargo build --release
```

The compiled binary will be available in the target/release directory.
Contributing

Contributions are welcome! If you have any suggestions, bug reports, or feature requests, please open an issue on the GitHub repository.
