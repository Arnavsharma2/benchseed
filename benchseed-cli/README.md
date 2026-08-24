# BenchSeed CLI

`benchseed-cli` provides the command line interface for generating TPC-H
and TPC-DS benchmark data.

## Install

```shell
pip install benchseed-cli
```

## Examples

```shell
benchseed-cli tpch -s 1 --output-dir /tmp/tpch
benchseed-cli tpch csv -s 1 --output-dir /tmp/tpch
benchseed-cli tpch parquet -s 100 --tables lineitem --parts 10 --output-dir /tmp/tpch
benchseed-cli tpcds csv -s 1 --output-dir /tmp/tpcds
benchseed-cli tpcds csv -s 1 --delimiter='\t' --output-dir /tmp/tpcds
```

CSV output supports comma, pipe, tab (`\t`), and semicolon delimiters. BenchSeed
rejects other delimiters because generated unquoted fields may contain them.
