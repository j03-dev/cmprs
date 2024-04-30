**README.md**

**CMPrs: A Huffman Coding Implementation**

**Introduction**

CMPrs is a simple and efficient implementation of Huffman coding in Rust. It provides a command-line interface for both compressing and decompressing files.

**How to Use**

To use CMPrs, first install it using the following command:

```bash
git clone https://github.com/j03-dev/cmprs
cd cmprs
cargo build --release
```

Once installed, you can use CMPrs to compress a file using the following command:

```
cmprs -c input.txt output.cmprs
```

This will create a compressed file called `output.cmprs`.

To decompress a file, use the following command:

```
cmprs -d input.cmprs output.txt
```

This will create a decompressed file called `output.txt`.

**Features**

* Simple and easy to use command-line interface
* Efficient Huffman coding algorithm
* Supports both compression and decompression
* Cross-platform support

**Benefits**

* Reduced file sizes
* Faster file transfers
* Improved data security

**Requirements**

* Rust 1.61 or later
* A text editor

**Getting Started**

1. Install Rust from the official website.
2. Install CMPrs using the command above.
3. Open a terminal window and navigate to the directory containing the file you want to compress or decompress.
4. Use the `cmprs` command to compress or decompress the file.

**Example**

To compress the file `input.txt` and save the compressed file as `output.cmprs`, run the following command:

```
cmprs -c input.txt output.cmprs
```

To decompress the file `output.cmprs` and save the decompressed file as `output.txt`, run the following command:

```
cmprs -d output.cmprs output.txt
```

**License**

CMPrs is licensed under the MIT License.

