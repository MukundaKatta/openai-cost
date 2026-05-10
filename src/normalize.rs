//! Strip dated snapshot suffixes from an OpenAI model identifier so the
//! same pricing table works for `gpt-5` and `gpt-5-2026-04-01`.
//!
//! OpenAI model ids come in three shapes:
//!
//! * `gpt-5` — alias for the latest snapshot
//! * `gpt-5-2026-04-01` — dated snapshot id
//! * `gpt-4o-mini-2024-07-18` — dated snapshot id
//!
//! We normalize dated snapshots back to the alias by stripping any
//! trailing `-YYYY-MM-DD` segment.

/// Strip a trailing `-YYYY-MM-DD` snapshot suffix.
///
/// Returns the base alias (`gpt-5`, `gpt-4o-mini`).
pub fn normalize_model_id(id: &str) -> &str {
    if let Some((head, tail)) = id.rsplit_once('-') {
        // tail is expected to be DD; previous segment is MM; one before is YYYY
        if is_n_digits(tail, 2) {
            if let Some((head2, tail2)) = head.rsplit_once('-') {
                if is_n_digits(tail2, 2) {
                    if let Some((head3, tail3)) = head2.rsplit_once('-') {
                        if is_n_digits(tail3, 4) {
                            return head3;
                        }
                    }
                }
            }
        }
    }
    id
}

fn is_n_digits(s: &str, n: usize) -> bool {
    s.len() == n && s.bytes().all(|b| b.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_dated_snapshot() {
        assert_eq!(normalize_model_id("gpt-5-2026-04-01"), "gpt-5");
        assert_eq!(normalize_model_id("gpt-4o-mini-2024-07-18"), "gpt-4o-mini");
        assert_eq!(normalize_model_id("o3-mini-2025-01-31"), "o3-mini");
    }

    #[test]
    fn keeps_plain_alias() {
        assert_eq!(normalize_model_id("gpt-5"), "gpt-5");
        assert_eq!(normalize_model_id("gpt-4o-mini"), "gpt-4o-mini");
        assert_eq!(normalize_model_id("o3"), "o3");
    }

    #[test]
    fn does_not_strip_partial_dates() {
        // Only 2 trailing date-shaped segments — not a full date.
        assert_eq!(normalize_model_id("foo-12-34"), "foo-12-34");
        // 3 segments but wrong widths.
        assert_eq!(normalize_model_id("foo-1-2-3"), "foo-1-2-3");
    }
}
