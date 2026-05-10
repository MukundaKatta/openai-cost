use openai_cost::{default_pricing, Pricing, Usage};

#[test]
fn computes_basic_cost() {
    let pricing = default_pricing("gpt-5").unwrap();
    // 1M input + 500k output = $1.25 + $5.00 = $6.25
    let usage = Usage {
        input_tokens: 1_000_000,
        output_tokens: 500_000,
        cached_input_tokens: 0,
    };
    let cost = pricing.cost_for(&usage);
    assert!((cost - 6.25).abs() < 1e-6, "got {cost}");
}

#[test]
fn cached_tokens_discounted() {
    let pricing = default_pricing("gpt-5").unwrap();
    // 1M cached input at 10% of input rate = $0.125
    let usage = Usage {
        input_tokens: 0,
        output_tokens: 0,
        cached_input_tokens: 1_000_000,
    };
    let cost = pricing.cost_for(&usage);
    assert!((cost - 0.125).abs() < 1e-6, "got {cost}");
}

#[test]
fn dated_snapshot_resolves() {
    assert_eq!(
        default_pricing("gpt-5-2026-04-01"),
        default_pricing("gpt-5")
    );
    assert_eq!(
        default_pricing("gpt-4o-mini-2024-07-18"),
        default_pricing("gpt-4o-mini")
    );
}

#[test]
fn unknown_model_returns_none() {
    assert!(default_pricing("not-a-real-model").is_none());
}

#[test]
fn chat_completions_constructor_subtracts_cached() {
    // OpenAI reports prompt_tokens as INCLUDING cached. Make sure we
    // subtract to avoid double-billing the cached chunk.
    let u = Usage::from_chat_completions(1000, 500, 300);
    assert_eq!(u.input_tokens, 700);
    assert_eq!(u.output_tokens, 500);
    assert_eq!(u.cached_input_tokens, 300);
    assert_eq!(u.total_tokens(), 1500);
}

#[test]
fn responses_api_constructor_is_flat() {
    let u = Usage::from_responses_api(700, 500, 300);
    assert_eq!(u.input_tokens, 700);
    assert_eq!(u.output_tokens, 500);
    assert_eq!(u.cached_input_tokens, 300);
}

#[test]
fn cache_hit_helper() {
    assert!(!Usage::default().cache_hit());
    assert!(Usage {
        cached_input_tokens: 1,
        ..Default::default()
    }
    .cache_hit());
}

#[test]
fn byo_pricing_works() {
    let custom = Pricing {
        input_per_mtok: 1.0,
        output_per_mtok: 2.0,
        cached_input_per_mtok: 0.1,
    };
    // 1M each: $1 + $2 + $0.10 = $3.10
    let usage = Usage {
        input_tokens: 1_000_000,
        output_tokens: 1_000_000,
        cached_input_tokens: 1_000_000,
    };
    let cost = custom.cost_for(&usage);
    assert!((cost - 3.1).abs() < 1e-6, "got {cost}");
}
