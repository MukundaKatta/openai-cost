//! Token usage block.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Three-field token usage as returned by OpenAI's chat-completion and
/// responses APIs.
///
/// Chat Completions exposes the cached count under
/// `prompt_tokens_details.cached_tokens`; the Responses API uses
/// `cached_input_tokens` directly. The `From` impls in this crate flatten
/// both shapes onto these fields.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct Usage {
    /// Fresh input/prompt tokens (not served from the prompt cache).
    pub input_tokens: u64,
    /// Output/completion tokens.
    pub output_tokens: u64,
    /// Input tokens that were served from the prompt cache (cache hit).
    pub cached_input_tokens: u64,
}

impl Usage {
    /// True when the request hit the prompt cache.
    pub fn cache_hit(&self) -> bool {
        self.cached_input_tokens > 0
    }

    /// Total tokens billed (input + output + cached_input).
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens + self.cached_input_tokens
    }

    /// Build a Usage from a Chat Completions response.
    ///
    /// OpenAI reports `prompt_tokens` as **including** cached tokens, so we
    /// subtract `cached_tokens` from `prompt_tokens` to get the fresh
    /// input count this struct expects.
    pub fn from_chat_completions(
        prompt_tokens: u64,
        completion_tokens: u64,
        cached_tokens: u64,
    ) -> Self {
        Self {
            input_tokens: prompt_tokens.saturating_sub(cached_tokens),
            output_tokens: completion_tokens,
            cached_input_tokens: cached_tokens,
        }
    }

    /// Build a Usage from a Responses-API response.
    ///
    /// On the Responses API, `input_tokens` already excludes cached tokens
    /// (cached_input_tokens is reported separately).
    pub fn from_responses_api(
        input_tokens: u64,
        output_tokens: u64,
        cached_input_tokens: u64,
    ) -> Self {
        Self {
            input_tokens,
            output_tokens,
            cached_input_tokens,
        }
    }
}
