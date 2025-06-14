package main

import (
	"fmt"
	"os"
	"slices"
	"strings"
)

func pop[T interface{}](s []T) []T {
	if len(s) == 0 {
		return s
	}
	s = s[:len(s)-1]
	return s
}

type Node struct {
	value rune
	frec  int
	left  *Node
	right *Node
}

func (n *Node) add(left, right *Node) {
	n.left = left
	n.right = right
	n.frec = left.frec + right.frec
}

func (n Node) path(target rune, path []byte) []byte {
	if n.value == target {
		return path
	}

	if n.left != nil {
		path = append(path, 0)
		if l_path := n.left.path(target, path); l_path != nil {
			return l_path
		}
		path = pop(path)
	}

	if n.right != nil {
		path = append(path, 1)
		if r_path := n.right.path(target, path); r_path != nil {
			return r_path
		}
		path = pop(path)
	}

	return nil
}

func lessNode(a, b Node) int {
	return b.frec - a.frec
}

func makeNodeFrom(msg string) []Node {
	var l_oc = make(map[rune]int, len(msg)) // letters occurences
	for _, r := range msg {
		l_oc[r] += 1
	}

	var nodes []Node

	for v, f := range l_oc {
		node := Node{
			value: v,
			frec:  f,
		}
		nodes = append(nodes, node)
	}

	slices.SortFunc(nodes, lessNode)
	return nodes
}

func makeHuffmanTree(nodes []Node) Node {
	var n = len(nodes)
	if n == 1 {
		return nodes[0]
	}
	var newNodes []Node
	for i := 0; i < n; i += 2 {
		if i+1 < n {
			var parent Node
			parent.add(&nodes[i], &nodes[i+1])
			newNodes = append(newNodes, parent)
		} else {
			newNodes = append(newNodes, nodes[i])
		}
	}
	slices.SortFunc(newNodes, lessNode)
	return makeHuffmanTree(newNodes)
}

func saveFile(bits []byte) {
	f, err := os.Create("cmprs_file")
	if err != nil {
		panic(err)
	}
	defer f.Close()

	for i := 0; i < len(bits); i += 8 {
		var b byte = 0
		for j := 0; j < 8 && i+j < len(bits); j++ {
			b <<= 1            // shift left
			b |= bits[i+j] & 1 // set bit if it's 1
		}
		// If chunk is smaller than 8 bits, shift to pad right
		if len(bits)-i < 8 {
			b <<= uint(8 - (len(bits) - i))
		}
		f.Write([]byte{b})
	}
}

func encodeText(text string, tree Node) []byte {
	var output []byte
	for _, r := range text {
		path := tree.path(r, []byte{})
		output = append(output, path...)
	}
	return output
}

func printTree(n *Node, level int) {
	if n == nil {
		return
	}
	printTree(n.right, level+1)
	fmt.Printf("%s '%s'->%d\n\n", strings.Repeat("  ", level), string(n.value), n.frec)
	printTree(n.left, level+1)
}

func main() {
	content, err := os.ReadFile("text.txt")

	if err != nil {
		panic(err)
	}

	text := string(content)
	nodes := makeNodeFrom(text)
	tree := makeHuffmanTree(nodes)
	output := encodeText(text, tree)
	fmt.Println(output)
	saveFile(output)

}
