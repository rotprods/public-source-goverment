#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfidenceError {
    InvalidZScore,
}

/// Wilson score interval lower bound for a Bernoulli proportion.
///
/// This is a project-native mathematical implementation. It is suitable for
/// confidence-adjusted quality/helpfulness signals where small sample sizes
/// must not dominate. It is explicitly not a civic-priority or vote-weight
/// function.
pub fn wilson_lower_bound(
    positive: u64,
    negative: u64,
    z_score: f64,
) -> Result<f64, ConfidenceError> {
    if !z_score.is_finite() || z_score <= 0.0 {
        return Err(ConfidenceError::InvalidZScore);
    }

    let n = positive.saturating_add(negative);
    if n == 0 {
        return Ok(0.0);
    }

    let n = n as f64;
    let p = positive as f64 / n;
    let z2 = z_score * z_score;
    let center = p + z2 / (2.0 * n);
    let margin = z_score * ((p * (1.0 - p) + z2 / (4.0 * n)) / n).sqrt();
    let denominator = 1.0 + z2 / n;

    Ok(((center - margin) / denominator).clamp(0.0, 1.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_sample_has_zero_confidence() {
        assert_eq!(wilson_lower_bound(0, 0, 1.96), Ok(0.0));
    }

    #[test]
    fn more_consistent_positive_evidence_increases_lower_bound() {
        let small = wilson_lower_bound(8, 2, 1.96).expect("valid score");
        let large = wilson_lower_bound(80, 20, 1.96).expect("valid score");

        assert!(large > small);
    }

    #[test]
    fn negative_votes_reduce_quality_confidence() {
        let strong = wilson_lower_bound(90, 10, 1.96).expect("valid score");
        let weak = wilson_lower_bound(60, 40, 1.96).expect("valid score");

        assert!(strong > weak);
    }

    #[test]
    fn invalid_z_score_is_rejected() {
        assert_eq!(
            wilson_lower_bound(1, 0, 0.0),
            Err(ConfidenceError::InvalidZScore)
        );
    }
}
