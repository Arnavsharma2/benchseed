# BenchSeed TPC-H Arrow

Generate TPCH data directly into [Apache Arrow] format using the [benchseed-tpch] and [arrow] crate.

[Apache Arrow]: https://arrow.apache.org/
[benchseed-tpch]: https://crates.io/crates/benchseed-tpch
[arrow]: https://crates.io/crates/arrow

# Example usage: 

See [docs.rs page](https://docs.rs/benchseed-tpch-arrow/latest/benchseed_tpch_arrow/)

# Testing:
This crate ensures correct results using two methods.

1. Basic functional tests are in Rust doc tests in the source code (`cargo test --doc`)
2. The `reparse` integration test ensures that the Arrow generators 
   produce the same results as parsing the original `tbl` format (`cargo test --test reparse`) 

# Contributing: 

Please see [CONTRIBUTING.md] for more information on how to contribute to this project.

[CONTRIBUTING.md]: https://github.com/Arnavsharma2/benchseed/blob/main/CONTRIBUTING.md
