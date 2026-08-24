# BenchSeed TPC-DS Arrow

Generate TPC-DS data directly into [Apache Arrow] format using the [benchseed-tpcds] and [arrow] crate.

[Apache Arrow]: https://arrow.apache.org/
[benchseed-tpcds]: https://crates.io/crates/benchseed-tpcds
[arrow]: https://crates.io/crates/arrow

# Example usage:

See [docs.rs page](https://docs.rs/benchseed-tpcds-arrow/latest/benchseed_tpcds_arrow/)

# Testing:

This crate ensures correct results using two methods.

1. Basic functional tests are in Rust doc tests in the source code (`cargo test -p benchseed-tpcds-arrow --doc`)
2. The `reparse` integration test ensures that the Arrow generators
   produce the same results as parsing the original DAT format (`cargo test -p benchseed-tpcds-arrow --test reparse`)

# Contributing:

Please see [CONTRIBUTING.md] for more information on how to contribute to this project.

[CONTRIBUTING.md]: https://github.com/Arnavsharma2/benchseed/blob/main/CONTRIBUTING.md
