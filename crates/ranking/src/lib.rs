#![forbid(unsafe_code)]

pub mod confidence;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CivicFeatures {
    pub territorial_relevance: f64,
    pub explicit_interest: f64,
    pub semantic_relevance: f64,
    pub civic_priority: f64,
    pub unresolved_importance: f64,
    pub evidence_quality: f64,
    pub deliberative_need: f64,
    pub institutional_urgency: f64,
    pub freshness: f64,
    pub exploration: f64,
    pub manipulation_risk: f64,
    pub fatigue: f64,
    pub duplicate_probability: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CivicWeights {
    pub territorial_relevance: f64,
    pub explicit_interest: f64,
    pub semantic_relevance: f64,
    pub civic_priority: f64,
    pub unresolved_importance: f64,
    pub evidence_quality: f64,
    pub deliberative_need: f64,
    pub institutional_urgency: f64,
    pub freshness: f64,
    pub exploration: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PenaltyWeights {
    pub manipulation_risk: f64,
    pub fatigue: f64,
    pub duplicate_probability: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RankingPolicy {
    pub version: String,
    pub weights: CivicWeights,
    pub penalties: PenaltyWeights,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreComponent {
    pub name: &'static str,
    pub weighted_value: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScoreBreakdown {
    pub policy_version: String,
    pub positive_score: f64,
    pub penalty_score: f64,
    pub total_score: f64,
    pub components: Vec<ScoreComponent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScoreError {
    NonFiniteFeature(&'static str),
    FeatureOutOfRange(&'static str),
    NonFiniteWeight(&'static str),
    NegativeWeight(&'static str),
    EmptyPolicyVersion,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VisibilityDecision {
    Allow,
    AllowWithContext { reason_code: String },
    DownrankByPolicy { reason_code: String },
    Drop { reason_code: String },
}

fn validate_feature(name: &'static str, value: f64) -> Result<(), ScoreError> {
    if !value.is_finite() {
        return Err(ScoreError::NonFiniteFeature(name));
    }
    if !(0.0..=1.0).contains(&value) {
        return Err(ScoreError::FeatureOutOfRange(name));
    }
    Ok(())
}

fn validate_weight(name: &'static str, value: f64) -> Result<(), ScoreError> {
    if !value.is_finite() {
        return Err(ScoreError::NonFiniteWeight(name));
    }
    if value < 0.0 {
        return Err(ScoreError::NegativeWeight(name));
    }
    Ok(())
}

/// Score one already-visible candidate using an explicit, versioned civic policy.
///
/// Visibility/authorization is intentionally not represented as a giant negative
/// score. Callers must evaluate those rules separately and pass only candidates
/// that the relevance/ranking stage is permitted to order.
pub fn score_candidate(
    policy: &RankingPolicy,
    features: CivicFeatures,
) -> Result<ScoreBreakdown, ScoreError> {
    if policy.version.trim().is_empty() {
        return Err(ScoreError::EmptyPolicyVersion);
    }

    let feature_values = [
        ("territorial_relevance", features.territorial_relevance),
        ("explicit_interest", features.explicit_interest),
        ("semantic_relevance", features.semantic_relevance),
        ("civic_priority", features.civic_priority),
        ("unresolved_importance", features.unresolved_importance),
        ("evidence_quality", features.evidence_quality),
        ("deliberative_need", features.deliberative_need),
        ("institutional_urgency", features.institutional_urgency),
        ("freshness", features.freshness),
        ("exploration", features.exploration),
        ("manipulation_risk", features.manipulation_risk),
        ("fatigue", features.fatigue),
        ("duplicate_probability", features.duplicate_probability),
    ];
    for (name, value) in feature_values {
        validate_feature(name, value)?;
    }

    let positive_terms = [
        (
            "territorial_relevance",
            features.territorial_relevance,
            policy.weights.territorial_relevance,
        ),
        (
            "explicit_interest",
            features.explicit_interest,
            policy.weights.explicit_interest,
        ),
        (
            "semantic_relevance",
            features.semantic_relevance,
            policy.weights.semantic_relevance,
        ),
        (
            "civic_priority",
            features.civic_priority,
            policy.weights.civic_priority,
        ),
        (
            "unresolved_importance",
            features.unresolved_importance,
            policy.weights.unresolved_importance,
        ),
        (
            "evidence_quality",
            features.evidence_quality,
            policy.weights.evidence_quality,
        ),
        (
            "deliberative_need",
            features.deliberative_need,
            policy.weights.deliberative_need,
        ),
        (
            "institutional_urgency",
            features.institutional_urgency,
            policy.weights.institutional_urgency,
        ),
        ("freshness", features.freshness, policy.weights.freshness),
        (
            "exploration",
            features.exploration,
            policy.weights.exploration,
        ),
    ];

    let penalty_terms = [
        (
            "manipulation_risk",
            features.manipulation_risk,
            policy.penalties.manipulation_risk,
        ),
        ("fatigue", features.fatigue, policy.penalties.fatigue),
        (
            "duplicate_probability",
            features.duplicate_probability,
            policy.penalties.duplicate_probability,
        ),
    ];

    for (name, _, weight) in positive_terms.iter().chain(penalty_terms.iter()) {
        validate_weight(name, *weight)?;
    }

    let mut components = Vec::with_capacity(positive_terms.len() + penalty_terms.len());
    let mut positive_score = 0.0;
    for (name, feature, weight) in positive_terms {
        let weighted_value = feature * weight;
        positive_score += weighted_value;
        components.push(ScoreComponent {
            name,
            weighted_value,
        });
    }

    let mut penalty_score = 0.0;
    for (name, feature, weight) in penalty_terms {
        let weighted_value = feature * weight;
        penalty_score += weighted_value;
        components.push(ScoreComponent {
            name,
            weighted_value: -weighted_value,
        });
    }

    Ok(ScoreBreakdown {
        policy_version: policy.version.clone(),
        positive_score,
        penalty_score,
        total_score: positive_score - penalty_score,
        components,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn policy() -> RankingPolicy {
        RankingPolicy {
            version: "test/1.0.0".into(),
            weights: CivicWeights {
                territorial_relevance: 1.0,
                explicit_interest: 0.8,
                semantic_relevance: 0.6,
                civic_priority: 0.8,
                unresolved_importance: 0.7,
                evidence_quality: 0.4,
                deliberative_need: 0.5,
                institutional_urgency: 0.7,
                freshness: 0.3,
                exploration: 0.1,
            },
            penalties: PenaltyWeights {
                manipulation_risk: 1.0,
                fatigue: 0.4,
                duplicate_probability: 1.0,
            },
        }
    }

    fn features() -> CivicFeatures {
        CivicFeatures {
            territorial_relevance: 1.0,
            explicit_interest: 0.5,
            semantic_relevance: 0.8,
            civic_priority: 0.7,
            unresolved_importance: 0.6,
            evidence_quality: 0.9,
            deliberative_need: 0.4,
            institutional_urgency: 0.2,
            freshness: 0.5,
            exploration: 0.1,
            manipulation_risk: 0.1,
            fatigue: 0.2,
            duplicate_probability: 0.0,
        }
    }

    #[test]
    fn explanation_components_reconstruct_total_score() {
        let score = score_candidate(&policy(), features()).expect("valid score");
        let reconstructed: f64 = score
            .components
            .iter()
            .map(|part| part.weighted_value)
            .sum();

        assert!((reconstructed - score.total_score).abs() < 1e-12);
    }

    #[test]
    fn manipulation_risk_can_only_reduce_score_with_nonnegative_policy_weights() {
        let clean = score_candidate(&policy(), features()).expect("clean score");
        let mut risky_features = features();
        risky_features.manipulation_risk = 0.9;
        let risky = score_candidate(&policy(), risky_features).expect("risky score");

        assert!(risky.total_score < clean.total_score);
    }

    #[test]
    fn out_of_range_feature_is_rejected_instead_of_silently_distorting_ranking() {
        let mut invalid = features();
        invalid.civic_priority = 1.2;

        assert_eq!(
            score_candidate(&policy(), invalid),
            Err(ScoreError::FeatureOutOfRange("civic_priority"))
        );
    }

    #[test]
    fn visibility_decision_remains_a_separate_type() {
        let decision = VisibilityDecision::Drop {
            reason_code: "BLOCKED_ACTOR".into(),
        };

        assert!(matches!(decision, VisibilityDecision::Drop { .. }));
    }
}
