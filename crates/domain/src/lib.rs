#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ActorId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TerritoryId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NeedId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LegalEffect {
    Informative,
    Signal,
    Advisory,
    AdministrativeInput,
    FormalConsultation,
    InternalBinding,
    RegulatedPublicProcess,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ParticipationKind {
    Support,
    Signature,
    Assessment,
    AdvisoryBallot,
}

/// Deliberately no generic `Vote` variant: civic actions have different semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CivicAction {
    pub actor_id: ActorId,
    pub territory_id: TerritoryId,
    pub kind: ParticipationKind,
    pub policy_version: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn support_and_ballot_are_distinct_domain_values() {
        assert_ne!(
            ParticipationKind::Support,
            ParticipationKind::AdvisoryBallot
        );
    }
}
