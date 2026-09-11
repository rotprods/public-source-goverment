#![forbid(unsafe_code)]

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq)]
pub struct FusedItem<T> {
    pub item: T,
    pub score: f64,
    pub source_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FusionError {
    InvalidRankConstant,
}

#[derive(Debug)]
struct Aggregate<T> {
    item: T,
    score: f64,
    source_count: usize,
    first_seen: usize,
}

/// Merge independent ranked candidate lists using Reciprocal Rank Fusion (RRF).
///
/// `rank_constant` is commonly set around 60, but this crate deliberately has
/// no production default: retrieval policy owns that choice and its evaluation.
/// Duplicate items inside one source list contribute at most once.
pub fn reciprocal_rank_fusion<T>(
    rankings: &[Vec<T>],
    rank_constant: f64,
) -> Result<Vec<FusedItem<T>>, FusionError>
where
    T: Clone + Eq + Hash,
{
    if !rank_constant.is_finite() || rank_constant <= 0.0 {
        return Err(FusionError::InvalidRankConstant);
    }

    let mut aggregate: HashMap<T, Aggregate<T>> = HashMap::new();
    let mut first_seen_counter = 0usize;

    for ranking in rankings {
        let mut seen_in_source = HashSet::new();

        for (zero_based_rank, item) in ranking.iter().enumerate() {
            if !seen_in_source.insert(item.clone()) {
                continue;
            }

            let contribution = 1.0 / (rank_constant + zero_based_rank as f64 + 1.0);
            let entry = aggregate.entry(item.clone()).or_insert_with(|| {
                let first_seen = first_seen_counter;
                first_seen_counter += 1;
                Aggregate {
                    item: item.clone(),
                    score: 0.0,
                    source_count: 0,
                    first_seen,
                }
            });

            entry.score += contribution;
            entry.source_count += 1;
        }
    }

    let mut items: Vec<_> = aggregate.into_values().collect();
    items.sort_by(|left, right| {
        right
            .score
            .total_cmp(&left.score)
            .then_with(|| right.source_count.cmp(&left.source_count))
            .then_with(|| left.first_seen.cmp(&right.first_seen))
    });

    Ok(items
        .into_iter()
        .map(|item| FusedItem {
            item: item.item,
            score: item.score,
            source_count: item.source_count,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consensus_across_sources_beats_single_source_top_result() {
        let rankings = vec![
            vec!["a", "b", "c"],
            vec!["b", "c", "d"],
            vec!["b", "e", "a"],
        ];

        let fused = reciprocal_rank_fusion(&rankings, 60.0).expect("valid RRF policy");

        assert_eq!(fused[0].item, "b");
        assert_eq!(fused[0].source_count, 3);
    }

    #[test]
    fn duplicate_inside_one_source_is_counted_once() {
        let rankings = vec![vec!["a", "a", "b"], vec!["b"]];

        let fused = reciprocal_rank_fusion(&rankings, 60.0).expect("valid RRF policy");
        let a = fused.iter().find(|entry| entry.item == "a").expect("a");

        assert_eq!(a.source_count, 1);
    }

    #[test]
    fn invalid_rank_constant_is_rejected() {
        assert_eq!(
            reciprocal_rank_fusion::<&str>(&[], 0.0),
            Err(FusionError::InvalidRankConstant)
        );
        assert_eq!(
            reciprocal_rank_fusion::<&str>(&[], f64::NAN),
            Err(FusionError::InvalidRankConstant)
        );
    }
}
