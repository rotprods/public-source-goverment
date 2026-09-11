#![forbid(unsafe_code)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    pub actor_id: Uuid,
    pub action: String,
    pub organization_ref: Option<String>,
    pub territory_id: Option<Uuid>,
    pub competence_node_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub resource_owner_actor_id: Option<Uuid>,
    pub step_up_authenticated: bool,
    pub second_approver_present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityScope {
    pub global: bool,
    pub organization_ref: Option<String>,
    pub territory_id: Option<Uuid>,
    pub competence_node_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub resource_type: Option<String>,
    pub resource_id: Option<Uuid>,
    pub owner_only: bool,
}

impl CapabilityScope {
    pub fn global() -> Self {
        Self {
            global: true,
            organization_ref: None,
            territory_id: None,
            competence_node_id: None,
            process_id: None,
            resource_type: None,
            resource_id: None,
            owner_only: false,
        }
    }

    pub fn scoped() -> Self {
        Self {
            global: false,
            organization_ref: None,
            territory_id: None,
            competence_node_id: None,
            process_id: None,
            resource_type: None,
            resource_id: None,
            owner_only: false,
        }
    }

    pub fn is_valid(&self) -> bool {
        let has_constraint = self.organization_ref.is_some()
            || self.territory_id.is_some()
            || self.competence_node_id.is_some()
            || self.process_id.is_some()
            || self.resource_type.is_some()
            || self.resource_id.is_some()
            || self.owner_only;

        if self.global {
            !has_constraint
        } else {
            has_constraint
        }
    }

    fn matches(&self, request: &AuthorizationRequest) -> bool {
        if !self.is_valid() {
            return false;
        }

        if self.global {
            return true;
        }

        if self
            .organization_ref
            .as_ref()
            .is_some_and(|expected| request.organization_ref.as_ref() != Some(expected))
        {
            return false;
        }
        if self
            .territory_id
            .is_some_and(|expected| request.territory_id != Some(expected))
        {
            return false;
        }
        if self
            .competence_node_id
            .is_some_and(|expected| request.competence_node_id != Some(expected))
        {
            return false;
        }
        if self
            .process_id
            .is_some_and(|expected| request.process_id != Some(expected))
        {
            return false;
        }
        if self
            .resource_type
            .as_ref()
            .is_some_and(|expected| request.resource_type.as_ref() != Some(expected))
        {
            return false;
        }
        if self
            .resource_id
            .is_some_and(|expected| request.resource_id != Some(expected))
        {
            return false;
        }
        if self.owner_only && request.resource_owner_actor_id != Some(request.actor_id) {
            return false;
        }

        true
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityAssignment {
    pub assignment_id: Uuid,
    pub subject_actor_id: Uuid,
    pub capability: String,
    pub scope: CapabilityScope,
    pub active: bool,
    pub requires_step_up: bool,
    pub requires_second_approver: bool,
    pub policy_version: String,
}

impl CapabilityAssignment {
    fn matches_subject_action_scope(&self, request: &AuthorizationRequest) -> bool {
        self.active
            && self.subject_actor_id == request.actor_id
            && self.capability == request.action
            && self.scope.matches(request)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AuthorizationDecisionKind {
    Allow,
    Deny,
    RequireStepUp,
    RequireSecondApprover,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationDecision {
    pub decision: AuthorizationDecisionKind,
    pub requested_action: String,
    pub matched_assignment_id: Option<Uuid>,
    pub policy_version: Option<String>,
    pub reason_codes: Vec<String>,
}

pub trait PolicyEngine: Send + Sync {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision;
}

#[derive(Debug, Clone, Default)]
pub struct StaticPolicyEngine {
    assignments: Vec<CapabilityAssignment>,
}

impl StaticPolicyEngine {
    pub fn new(assignments: Vec<CapabilityAssignment>) -> Self {
        Self { assignments }
    }
}

impl PolicyEngine for StaticPolicyEngine {
    fn authorize(&self, request: &AuthorizationRequest) -> AuthorizationDecision {
        let matches: Vec<&CapabilityAssignment> = self
            .assignments
            .iter()
            .filter(|assignment| assignment.matches_subject_action_scope(request))
            .collect();

        if matches.is_empty() {
            return AuthorizationDecision {
                decision: AuthorizationDecisionKind::Deny,
                requested_action: request.action.clone(),
                matched_assignment_id: None,
                policy_version: None,
                reason_codes: vec!["NO_MATCHING_ACTIVE_CAPABILITY".into()],
            };
        }

        if let Some(assignment) = matches.iter().copied().find(|assignment| {
            (!assignment.requires_step_up || request.step_up_authenticated)
                && (!assignment.requires_second_approver || request.second_approver_present)
        }) {
            return AuthorizationDecision {
                decision: AuthorizationDecisionKind::Allow,
                requested_action: request.action.clone(),
                matched_assignment_id: Some(assignment.assignment_id),
                policy_version: Some(assignment.policy_version.clone()),
                reason_codes: vec!["CAPABILITY_SCOPE_MATCH".into()],
            };
        }

        if let Some(assignment) = matches
            .iter()
            .copied()
            .find(|assignment| assignment.requires_step_up && !request.step_up_authenticated)
        {
            return AuthorizationDecision {
                decision: AuthorizationDecisionKind::RequireStepUp,
                requested_action: request.action.clone(),
                matched_assignment_id: Some(assignment.assignment_id),
                policy_version: Some(assignment.policy_version.clone()),
                reason_codes: vec!["STEP_UP_AUTH_REQUIRED".into()],
            };
        }

        let assignment = matches[0];
        AuthorizationDecision {
            decision: AuthorizationDecisionKind::RequireSecondApprover,
            requested_action: request.action.clone(),
            matched_assignment_id: Some(assignment.assignment_id),
            policy_version: Some(assignment.policy_version.clone()),
            reason_codes: vec!["SECOND_APPROVER_REQUIRED".into()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(actor_id: Uuid, action: &str) -> AuthorizationRequest {
        AuthorizationRequest {
            actor_id,
            action: action.into(),
            organization_ref: None,
            territory_id: None,
            competence_node_id: None,
            process_id: None,
            resource_type: None,
            resource_id: None,
            resource_owner_actor_id: None,
            step_up_authenticated: false,
            second_approver_present: false,
        }
    }

    fn assignment(
        actor_id: Uuid,
        capability: &str,
        scope: CapabilityScope,
    ) -> CapabilityAssignment {
        CapabilityAssignment {
            assignment_id: Uuid::now_v7(),
            subject_actor_id: actor_id,
            capability: capability.into(),
            scope,
            active: true,
            requires_step_up: false,
            requires_second_approver: false,
            policy_version: "authz/test/1.0.0".into(),
        }
    }

    #[test]
    fn deny_by_default_without_matching_capability() {
        let actor = Uuid::now_v7();
        let engine = StaticPolicyEngine::default();
        let decision = engine.authorize(&request(actor, "need.publish"));

        assert_eq!(decision.decision, AuthorizationDecisionKind::Deny);
        assert_eq!(decision.reason_codes, vec!["NO_MATCHING_ACTIVE_CAPABILITY"]);
    }

    #[test]
    fn territory_scope_prevents_cross_territory_authority() {
        let actor = Uuid::now_v7();
        let allowed_territory = Uuid::now_v7();
        let other_territory = Uuid::now_v7();
        let mut scope = CapabilityScope::scoped();
        scope.territory_id = Some(allowed_territory);
        let engine =
            StaticPolicyEngine::new(vec![assignment(actor, "response.publish_official", scope)]);

        let mut allowed = request(actor, "response.publish_official");
        allowed.territory_id = Some(allowed_territory);
        assert_eq!(
            engine.authorize(&allowed).decision,
            AuthorizationDecisionKind::Allow
        );

        let mut denied = request(actor, "response.publish_official");
        denied.territory_id = Some(other_territory);
        assert_eq!(
            engine.authorize(&denied).decision,
            AuthorizationDecisionKind::Deny
        );
    }

    #[test]
    fn owner_only_scope_requires_actual_resource_ownership() {
        let actor = Uuid::now_v7();
        let other_actor = Uuid::now_v7();
        let mut scope = CapabilityScope::scoped();
        scope.owner_only = true;
        let engine = StaticPolicyEngine::new(vec![assignment(actor, "need.edit_own_draft", scope)]);

        let mut allowed = request(actor, "need.edit_own_draft");
        allowed.resource_owner_actor_id = Some(actor);
        assert_eq!(
            engine.authorize(&allowed).decision,
            AuthorizationDecisionKind::Allow
        );

        let mut denied = request(actor, "need.edit_own_draft");
        denied.resource_owner_actor_id = Some(other_actor);
        assert_eq!(
            engine.authorize(&denied).decision,
            AuthorizationDecisionKind::Deny
        );
    }

    #[test]
    fn high_impact_capability_can_require_step_up() {
        let actor = Uuid::now_v7();
        let mut high_impact = assignment(actor, "policy.activate", CapabilityScope::global());
        high_impact.requires_step_up = true;
        let engine = StaticPolicyEngine::new(vec![high_impact]);

        let mut req = request(actor, "policy.activate");
        assert_eq!(
            engine.authorize(&req).decision,
            AuthorizationDecisionKind::RequireStepUp
        );

        req.step_up_authenticated = true;
        assert_eq!(
            engine.authorize(&req).decision,
            AuthorizationDecisionKind::Allow
        );
    }

    #[test]
    fn high_impact_capability_can_require_second_approver() {
        let actor = Uuid::now_v7();
        let mut high_impact = assignment(actor, "policy.activate", CapabilityScope::global());
        high_impact.requires_second_approver = true;
        let engine = StaticPolicyEngine::new(vec![high_impact]);

        let mut req = request(actor, "policy.activate");
        assert_eq!(
            engine.authorize(&req).decision,
            AuthorizationDecisionKind::RequireSecondApprover
        );

        req.second_approver_present = true;
        assert_eq!(
            engine.authorize(&req).decision,
            AuthorizationDecisionKind::Allow
        );
    }

    #[test]
    fn invalid_ambiguous_scope_never_matches() {
        let actor = Uuid::now_v7();
        let invalid_scope = CapabilityScope::scoped();
        assert!(!invalid_scope.is_valid());

        let engine = StaticPolicyEngine::new(vec![assignment(
            actor,
            "response.publish_official",
            invalid_scope,
        )]);

        assert_eq!(
            engine
                .authorize(&request(actor, "response.publish_official"))
                .decision,
            AuthorizationDecisionKind::Deny
        );
    }
}
