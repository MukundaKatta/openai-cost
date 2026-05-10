# openai-cost

[![crates.io](https://img.shields.io/crates/v/openai-cost.svg)](https://crates.io/crates/openai-cost)
[![docs.rs](https://img.shields.io/docsrs/openai-cost)](https://docs.rs/openai-cost)
[![license: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT_or_Apache--2.0-blue.svg)](#license)

Calculate OpenAI API call cost from a usage block. Cache-aware (cached
input tokens), supports GPT-5, GPT-4.1, o3, o4 model families. Bring your
own pricing override. Zero runtime dependencies.

## Why

Every usage logger ships its own copy of this. They get the cache math
wrong, or they hard-code yesterday's pricing, or they break when OpenAI
adds a new model. This crate is the small, dated, tested version you can
reuse and override where you need to.

## Usage

```rust
use openai_cost::{Usage, default_pricing};

let pricing = default_pricing("gpt-5").unwrap();

// From a Chat Completions response (prompt_tokens INCLUDES cached):
let usage = Usage::from_chat_completions(
    /* prompt_tokens     = */ 1000,
    /* completion_tokens = */  500,
    /* cached_tokens     = */  300,
);
let cost_usd = pricing.cost_for(&usage);
```

Or from the Responses API (already split):

```rust
use openai_cost::Usage;
let usage = Usage::from_responses_api(700, 500, 300);
```

## Dated snapshots

Pass any dated snapshot id; the lookup strips the `-YYYY-MM-DD` suffix
back to the base alias.

```rust
use openai_cost::default_pricing;
assert!(default_pricing("gpt-5-2026-04-01").is_some());
assert!(default_pricing("gpt-4o-mini-2024-07-18").is_some());
```

## BYO pricing

```rust
use openai_cost::{Pricing, Usage};

let custom = Pricing {
    input_per_mtok: 1.25,
    output_per_mtok: 5.0,
    cached_input_per_mtok: 0.125,
};
let _ = custom.cost_for(&Usage::default());
```

## Pricing notes

All rates are USD per 1,000,000 tokens. Pricing is dated as of 2026-Q2.
**Verify against <https://openai.com/api/pricing/> before billing.**

## Features

- `serde` — derive `Serialize`/`Deserialize` on `Usage` so you can parse
  a raw API response directly.

```toml
[dependencies]
openai-cost = { version = "0.1", features = ["serde"] }
```

## License

Licensed under either of [MIT](LICENSE-MIT) or
[Apache-2.0](LICENSE-APACHE) at your option.
