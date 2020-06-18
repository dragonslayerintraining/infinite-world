# infinite-world

I made this project because I was learning Rust and wanted to make something with it that was simple but still interesting.

## Goal 

The goal of this project is to generate a grid of cells that has some nice properties:
* The world is infinite. In particular, there must be a way to compute what is at any cell in constant (expected) time.
* Any two cells are equally likely to be anything. Further, squares of the same size have the same probability distribution.
* Far away cells are independent. This rules out randomly-shifted periodic patterns.

In the current world,
* any 3 by 3 square has at least one tree.
* no two trees are adjacent.

## How it works

This problem can be rephrased as finding a maximal independent set in an infinite graph. We can use a distributed maximal independent set algorithm.

* Each cell picks a random number
* If a cell's number is greater than all of its neighbors, it gets a tree.
* If a cell's neighbor gets a tree, it gets nothing.
* At this point one-ninth of the cells get trees, but empty 3 by 3 squares are still possible. The process is repeated on the undetermined cells until none remain.

While the entire process takes forever, the value at an individual cell is computed fairly quickly. Random numbers at each cell are emulated by hashing the coordinates.

