# BenchSeed

[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[license-badge]: https://img.shields.io/badge/license-Apache%20v2-blue.svg
[license-url]: https://github.com/Arnavsharma2/benchseed/blob/main/LICENSE
[actions-badge]: https://github.com/Arnavsharma2/benchseed/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/Arnavsharma2/benchseed/actions?query=branch%3Amain

Generate deterministic [TPC-H] and [TPC-DS] datasets in Rust, with CSV, Parquet, and Apache Arrow output for testing data systems.

Built on [tpchgen-rs](https://github.com/clflushopt/tpchgen-rs). This repository focuses on safer CSV output and allocation/coordination overhead in the generation pipeline. See [NOTICE](NOTICE) for the source snapshot and attribution.

[Quick start](#quick-start) · [My contributions](#my-contributions) · [Architecture](ARCHITECTURE.md) · [Testing](TESTING.md)

[TPC-H]: https://www.tpc.org/tpch/
[TPC-DS]: https://www.tpc.org/tpcds/

## My contributions

- **Safer CSV output:** shared delimiter validation across both CLIs and rejected characters that can corrupt unquoted table output, with regression coverage. This follows the proposed contract in [upstream issue #381](https://github.com/clflushopt/tpchgen-rs/issues/381). [Implementation](https://github.com/Arnavsharma2/benchseed/commit/71b7c78759a071a992c99db97b3929a8be5a07c2)
- **Lower allocation overhead:** parsed distribution records without temporary vectors and scanned text-generation grammar as ASCII bytes. [Changes](https://github.com/Arnavsharma2/benchseed/commits/main/)
- **Less coordination work:** batched TPC-DS progress updates, reused command metadata, and shortened buffer-recycler lock hold time. [Changes](https://github.com/Arnavsharma2/benchseed/commits/main/)

The generator foundation, conformance suite, and benchmark results below are inherited from tpchgen-rs. No new end-to-end throughput gain is claimed for these changes.

## Quick start

Build from this repository using [Rust and Cargo](https://www.rust-lang.org/tools/install). The toolchain is pinned in [rust-toolchain.toml](rust-toolchain.toml).

```sh
git clone https://github.com/Arnavsharma2/benchseed.git
cd benchseed
cargo build --release -p benchseed-cli
./target/release/benchseed-cli tpch parquet -s 1 --output-dir /tmp/benchseed-tpch
```

Generate TPC-DS CSV instead:

```sh
./target/release/benchseed-cli tpcds csv -s 1 --output-dir /tmp/benchseed-tpcds
```

CSV supports comma, pipe, tab (`\t`), and semicolon delimiters. Other delimiters are rejected because generated fields are not quoted. See the [CLI examples](benchseed-cli/README.md#examples) for table selection and partitioning.

The workspace includes dependency-free TPC-H generation, Arrow adapters, and parallel streaming CLIs. See [Crates](#crates) to embed a generator in your own Rust application.

## Performance

The following results were measured by the upstream tpchgen-rs project. They describe its benchmark workload and hardware, not a measured gain from BenchSeed-specific changes. See [BENCHMARKS.md](./benchmarks/BENCHMARKS.md) for methodology and details.

Times to create TPC-H tables in Parquet format using the upstream generator and
DuckDB for various scale factors. These measurements were inherited from the
[upstream benchmark suite](https://github.com/clflushopt/tpchgen-rs/tree/7e64184d25648ffa8db4575bc683a24d23f60297/benchmarks).

| Scale Factor | Upstream generator | DuckDB     | DuckDB (proprietary) |
| ------------ | ------------- | ---------- | -------------------- |
| 1            | `0:02.24`     | `0:12.29`  | `0:10.68`            |
| 10           | `0:09.97`     | `1:46.80`  | `1:41.14`            |
| 100          | `1:14.22`     | `17:48.27` | `16:40.88`           |
| 1000         | `10:26.26`    | N/A (OOM)  | N/A (OOM)            |

- DuckDB (proprietary) is the time required to create TPC-H data using the
  proprietary DuckDB format
- Creating Scale Factor 1000 using DuckDB [required 647 GB of memory](https://duckdb.org/docs/stable/extensions/tpch.html#resource-usage-of-the-data-generator),
  which is why it is not included in the table above.

## Answers

The core `benchseed-tpch` crate provides answers for queries 1 to 22 and for a scale factor
of 1. The answers exposed were derived from the [TPC-H Tools](https://www.tpc.org/)
official distribution.

## Testing

```sh
cargo test --workspace
```

The inherited conformance tests compare generated output with reference data from [`dbgen`] and the TPC tools. Additional CLI tests cover supported and rejected delimiters. See [TESTING.md](TESTING.md) for methodology and the reference-tool requirements for the full comparison suite.

## Crates

- [`benchseed-tpch`](benchseed-tpch): the core data generator logic for TPC-H. It has no
  dependencies and is easy to embed in other Rust project.

- [`benchseed-tpch-arrow`](benchseed-tpch-arrow) generates TPC-H data in [Apache Arrow]
  format. It depends on the arrow-rs library

- [`benchseed-tpch-cli`](benchseed-tpch-cli) is a [`dbgen`] compatible CLI tool that generates
  benchmark dataset using multiple processes.

- [`benchseed-tpcds`](benchseed-tpcds) provides core TPC-DS generation logic.

- [`benchseed-tpcds-arrow`](benchseed-tpcds-arrow) converts generated TPC-DS
  rows into Apache Arrow.

- [`benchseed-cli`](benchseed-cli) provides the unified TPC-H and TPC-DS CLI.

[Apache Arrow]: https://arrow.apache.org/
[`dbgen`]: https://github.com/electrum/tpch-dbgen

## Contributing

Pull requests are welcome. For major changes, please open an issue first for
discussion. See our [contributors guide](CONTRIBUTING.md) for more details.

## Architecture

Please see [architecture guide](ARCHITECTURE.md) for details on how the code
is structured.

## License

The project is licensed under the [Apache License 2.0](LICENSE). See
[NOTICE](NOTICE) for upstream attribution.

## References

- The TPC-H Specification, see the specification [page](https://www.tpc.org/tpc_documents_current_versions/current_specifications5.asp).
- The Original `dbgen` Implementation you must submit an official request to access the software `dbgen` at their official [website](https://www.tpc.org/tpch/)
