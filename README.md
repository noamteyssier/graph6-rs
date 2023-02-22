# graph6-rs

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)
![actions status](https://github.com/noamteyssier/graph6-rs/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/noamteyssier/graph6-rs/branch/main/graph/badge.svg?token=1UIAB0XFRH)](https://codecov.io/gh/noamteyssier/graph6-rs)

a rust library for converting to and from graph6 formatted files from NAUTY

## File Format Description

The graph6 format for undirected, sparse, and directed graphs can be found
[here](https://users.cecs.anu.edu.au/~bdm/data/formats.txt) and is mirrored
in this repo at `resources/formats.txt`.

## Related Crates

- [graph6](https://crates.io/crates/graph6)
  - This work attempts to extend the original work of `graph6` to include
    directed and sparse graphs as well as just undirected graphs.
