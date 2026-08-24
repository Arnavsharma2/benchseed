# Architecture Guide

## Crate Organization
The workspace is organized into crates that separate the data generation, Arrow
conversion, and command-line tools:

1. `benchseed-tpch`: The core library that implements TPC-H data generation.
2. `benchseed-tpch-arrow`: Converts TPC-H generated rows directly into the
   [Apache Arrow] in-memory format.
3. `benchseed-tpcds`: The core library that implements TPC-DS data generation.
4. `benchseed-tpcds-arrow`: Converts TPC-DS generated rows directly into the
   [Apache Arrow] in-memory format.
5. `benchseed-cli`: The shared command-line implementation crate for TPC benchmark
   data generation.
6. `benchseed-tpch-cli`: A convenience package that provides a dedicated
   TPC-H-only `benchseed-tpch-cli` binary.

## Dependencies

The `benchseed-tpch` and `benchseed-tpcds` crates are designed to be embeddable in as many
locations as possible. The TPC-H core crate has no dependencies by design; for
example, it does not depend on Arrow, Parquet, or display libraries.

The `benchseed-tpch-arrow` and `benchseed-tpcds-arrow` crates are similarly designed to be
embeddable with minimal dependencies. They depend on the
[`arrow` crate](https://docs.rs/arrow) and keep Arrow-specific conversion logic
out of the core generator crates.

The `benchseed-cli` crate contains CLI-oriented dependencies and features, including
CSV/Parquet output, multi-threaded orchestration, progress reporting, logging,
and command parsing. The `benchseed-tpch-cli` package is intentionally thin and
depends on `benchseed-cli` so users can choose a dedicated TPC-H entry point.

## Performance

Speed is a very important aspect of this project, and care has been taken to keep 
the code as fast as possible, using some of the following techniques:
1. Avoiding heap allocations during data generation
2. Integer arithmetic and display instead of floating point arithmetic and display
3. Using multiple cores and tuned buffer sizes


[Apache Arrow]: https://arrow.apache.org/
