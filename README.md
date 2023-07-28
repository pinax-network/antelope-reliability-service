# Blocktime Query Service

gRPC service for querying the number of missing blocks on the Antelope chain.

## Setup

Make sure to have the [`substreams-sink-kv`](https://github.com/streamingfast/substreams-sink-kv) tool installed and available in your `$PATH`.

**Populate the kv-store with data from the remote endpoint**
```bash
./inject.sh <endpoint-url>
```

You can install the [`kvdb`](https://github.com/streamingfast/kvdb) tool to inspect the data stored in the *kv-store* locally:
```bash
go install github.com/streamingfast/kvdb/cmd/kvdb@develop
```

**To inspect the *Timestamp > BlockId* data**
```bash
kvdb read prefix kdate --dsn "badger3://$(pwd)/badger_data.db" --decoder="proto://./proto/antelope_block_meta.proto@antelope.antelope_block_meta.v1.AntelopeBlockMeta"
```

## Running the service

**Serve the data from localhost through badgerDB**
```bash
./serve.sh
```

**Service query examples (make sure the `serve` script is running)**
```bash
$ grpcurl -plaintext -proto ./proto/service.proto -d '{"start_date": "2023-07-26T11:36:37.500Z", "end_date": "2023-07-26T11:36:39.000Z"}' localhost:7878 pinax.service.v1.AntelopeReliability.CountMissingBlocks

```