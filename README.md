# BenchSeed

[![Apache licensed][license-badge]][license-url]
[![Build Status][actions-badge]][actions-url]

[license-badge]: https://img.shields.io/badge/license-Apache%20v2-blue.svg
[license-url]: https://github.com/Arnavsharma2/benchseed/blob/main/LICENSE
[actions-badge]: https://github.com/Arnavsharma2/benchseed/actions/workflows/rust.yml/badge.svg
[actions-url]: https://github.com/Arnavsharma2/benchseed/actions?query=branch%3Amain

Fast, deterministic [TPC-H] and [TPC-DS] benchmark data generation in Rust.

[TPC-H]: https://www.tpc.org/tpch/
[TPC-DS]: https://www.tpc.org/tpcds/

> **Origin:** This repository began as a clean-history import of
> [clflushopt/tpchgen-rs](https://github.com/clflushopt/tpchgen-rs) at commit
> [`7e64184`](https://github.com/clflushopt/tpchgen-rs/commit/7e64184d25648ffa8db4575bc683a24d23f60297).
> The original work is by clflushopt, alamb, and the upstream contributors.
> See [NOTICE](NOTICE) for attribution and a summary of subsequent changes.

## Features

1. High-throughput TPC-H and TPC-DS generation
2. Deterministic, conformance-tested output
3. Parallel, streaming execution with bounded memory usage

## Try it now

Try with `uvx`:

```shell
uvx benchseed-cli tpch parquet -s 1 --output-dir /tmp/tpch
```

Install with `pip`:

```shell
python -m pip install benchseed-cli
```

Then generate TPC-H data:

```shell
benchseed-cli tpch parquet -s 1 --output-dir /tmp/tpch
```

`benchseed-cli` is the unified command-line program distributed as a Python
package. See its [README](benchseed-cli/README.md) for more examples.

## Performance

The inherited benchmark suite records the TPC-H generator as more than 10x
faster than the next fastest generator tested. On a 2023 Mac M3 Max laptop, it
easily generates data faster than can
know of. On a 2023 Mac M3 Max laptop, it easily generates data faster than can
be written to SSD. See [BENCHMARKS.md](./benchmarks/BENCHMARKS.md) for more
details on performance and benchmarking.

Times to create TPC-H tables in Parquet format using `benchseed-tpch-cli` and
DuckDB for various scale factors. These measurements were inherited from the
[upstream benchmark suite](https://github.com/clflushopt/tpchgen-rs/tree/7e64184d25648ffa8db4575bc683a24d23f60297/benchmarks).

| Scale Factor | `benchseed-tpch-cli` | DuckDB     | DuckDB (proprietary) |
| ------------ | ------------- | ---------- | -------------------- |
| 1            | `0:02.24`     | `0:12.29`  | `0:10.68`            |
| 10           | `0:09.97`     | `1:46.80`  | `1:41.14`            |
| 100          | `1:14.22`     | `17:48.27` | `16:40.88`           |
| 1000         | `10:26.26`    | N/A (OOM)  | N/A (OOM)            |

- DuckDB (proprietary) is the time required to create TPCH data using the
  proprietary DuckDB format
- Creating Scale Factor 1000 using DuckDB [required 647 GB of memory](https://duckdb.org/docs/stable/extensions/tpch.html#resource-usage-of-the-data-generator),
  which is why it is not included in the table above.

## Answers

The core `benchseed-tpch` crate provides answers for queries 1 to 22 and for a scale factor
of 1. The answers exposed were derived from the [TPC-H Tools](https://www.tpc.org/)
official distribution.

## Testing

This crate has extensive tests to ensure correctness and produces exactly the
same, byte-for-byte output as the original [`dbgen`] implementation. We compare
the output of this crate with [`dbgen`] as part of every checkin. See
[TESTING.md](TESTING.md) for more details on testing methodology

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
