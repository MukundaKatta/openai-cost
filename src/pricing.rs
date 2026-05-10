//! Per-model price table.
//!
//! All values are USD per 1,000,000 tokens, matching how OpenAI publishes
//! their rates. cached_input is typically 50% of fresh input.

use crate::normalize::normalize_model_id;
use crate::usage::Usage;

/// Per-model rates, USD per 1M tokens.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pricing {
    /// Fresh input tokens (no cache).
    pub input_per_mtok: f64,
    /// Output tokens.
    pub output_per_mtok: f64,
    /// Cached input tokens (cache hit).
    pub cached_input_per_mtok: f64,
}

impl Pricing {
    /// Compute USD cost for the given usage.
    pub fn cost_for(&self, usage: &Usage) -> f64 {
        (usage.input_tokens as f64 * self.input_per_mtok
            + usage.output_tokens as f64 * self.output_per_mtok
            + usage.cached_input_tokens as f64 * self.cached_input_per_mtok)
            / 1_000_000.0
    }
}

/// Built-in pricing table. Source: openai.com/api/pricing as of 2026-Q2.
/// VERIFY before billing.
///
/// Keys are normalized base aliases (no dated suffix). Use
/// [`default_pricing`] to look up by an aliased or dated model id.
pub const DEFAULT_PRICING_TABLE: &[(&str, Pricing)] = &[
    (
        "gpt-5",
        Pricing {
            input_per_mtok: 1.25,
            output_per_mtok: 10.0,
            cached_input_per_mtok: 0.125,
        },
    ),
    (
        "gpt-5-mini",
        Pricing {
            input_per_mtok: 0.25,
            output_per_mtok: 2.0,
            cached_input_per_mtok: 0.025,
        },
    ),
    (
        "gpt-5-nano",
        Pricing {
            input_per_mtok: 0.05,
            output_per_mtok: 0.4,
            cached_input_per_mtok: 0.005,
        },
    ),
    (
        "gpt-4.1",
        Pricing {
            input_per_mtok: 2.0,
            output_per_mtok: 8.0,
            cached_input_per_mtok: 0.5,
        },
    ),
    (
        "gpt-4.1-mini",
        Pricing {
            input_per_mtok: 0.4,
            output_per_mtok: 1.6,
            cached_input_per_mtok: 0.1,
        },
    ),
    (
        "gpt-4.1-nano",
        Pricing {
            input_per_mtok: 0.1,
            output_per_mtok: 0.4,
            cached_input_per_mtok: 0.025,
        },
    ),
    (
        "gpt-4o",
        Pricing {
            input_per_mtok: 2.5,
            output_per_mtok: 10.0,
            cached_input_per_mtok: 1.25,
        },
    ),
    (
        "gpt-4o-mini",
        Pricing {
            input_per_mtok: 0.15,
            output_per_mtok: 0.6,
            cached_input_per_mtok: 0.075,
        },
    ),
    (
        "o3",
        Pricing {
            input_per_mtok: 2.0,
            output_per_mtok: 8.0,
            cached_input_per_mtok: 0.5,
        },
    ),
    (
        "o3-mini",
        Pricing {
            input_per_mtok: 1.1,
            output_per_mtok: 4.4,
            cached_input_per_mtok: 0.55,
        },
    ),
    (
        "o4-mini",
        Pricing {
            input_per_mtok: 1.1,
            output_per_mtok: 4.4,
            cached_input_per_mtok: 0.275,
        },
    ),
];

/// Look up the price table entry for an OpenAI model id.
///
/// Accepts dated snapshot ids (`gpt-5-2026-04-01`) and resolves them back
/// to the base alias.
pub fn default_pricing(model_id: &str) -> Option<Pricing> {
    let key = normalize_model_id(model_id);
    DEFAULT_PRICING_TABLE
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, p)| *p)
}
