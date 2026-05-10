//! # openai-cost
//!
//! Calculate OpenAI API call cost from a usage block.
//!
//! OpenAI returns three token counts on a chat-completion or responses-API
//! response: `prompt_tokens` (a.k.a. `input_tokens`), `completion_tokens`
//! (a.k.a. `output_tokens`), and `prompt_tokens_details.cached_tokens`
//! (a.k.a. `cached_input_tokens` on the Responses API). Each has its own
//! price. This crate gives you a small `Pricing` table for popular models
//! and a `Usage` struct that knows how to compute cost from those fields.
//!
//! Pricing is best-effort and dated; verify against
//! <https://openai.com/api/pricing/> before using these numbers for
//! billing.
//!
//! ## Quick example
//!
//! ```
//! use openai_cost::{Usage, default_pricing};
//!
//! let pricing = default_pricing("gpt-5").unwrap();
//! let usage = Usage {
//!     input_tokens: 1_000,
//!     output_tokens: 500,
//!     cached_input_tokens: 0,
//! };
//! let cost = pricing.cost_for(&usage);
//! // 1k * input + 500 * output, divided by 1M
//! assert!(cost > 0.0);
//! ```
//!
//! ## Versioned model ids
//!
//! ```
//! use openai_cost::default_pricing;
//! // The crate normalizes dated suffixes (`gpt-5-2026-04-01`) back to
//! // the base model name.
//! assert!(default_pricing("gpt-5-2026-04-01").is_some());
//! ```
//!
//! ## BYO pricing
//!
//! ```
//! use openai_cost::{Pricing, Usage};
//! let custom = Pricing {
//!     input_per_mtok: 1.25,
//!     output_per_mtok: 5.0,
//!     cached_input_per_mtok: 0.125,
//! };
//! let usage = Usage::default();
//! let _ = custom.cost_for(&usage);
//! ```

#![deny(missing_docs)]

mod normalize;
mod pricing;
mod usage;

pub use normalize::normalize_model_id;
pub use pricing::{default_pricing, Pricing, DEFAULT_PRICING_TABLE};
pub use usage::Usage;
