# OutLayer Auction — Rust/WASI

Build:

```bash
./build.sh
```

Input:

```json
{"bids":[70,55,91,80]}
```

Expected output:

```json
{"winner":2,"price":80}
```

Build output:

`target/wasm32-wasip1/release/outlayer_auction.wasm`

WASI target:

`wasm32-wasip1`
