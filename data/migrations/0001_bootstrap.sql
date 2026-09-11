BEGIN;

CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS vector;
CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE SCHEMA IF NOT EXISTS identity_private;
CREATE SCHEMA IF NOT EXISTS civic;
CREATE SCHEMA IF NOT EXISTS policy;
CREATE SCHEMA IF NOT EXISTS institution;
CREATE SCHEMA IF NOT EXISTS moderation;
CREATE SCHEMA IF NOT EXISTS ai;
CREATE SCHEMA IF NOT EXISTS audit;
CREATE SCHEMA IF NOT EXISTS search_projection;

CREATE TABLE policy.policy_version (
    policy_version_id uuid PRIMARY KEY,
    policy_type text NOT NULL,
    semantic_version text NOT NULL,
    status text NOT NULL CHECK (status IN ('DRAFT','APPROVED','ACTIVE','SUPERSEDED','REVOKED')),
    body jsonb NOT NULL,
    body_sha256 text NOT NULL,
    authority_ref text NOT NULL,
    approved_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (policy_type, semantic_version)
);

CREATE TABLE identity_private.account (
    account_id uuid PRIMARY KEY,
    auth_subject text NOT NULL UNIQUE,
    status text NOT NULL CHECK (status IN ('ACTIVE','SUSPENDED','CLOSED')),
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE identity_private.identity_assertion (
    assertion_id uuid PRIMARY KEY,
    account_id uuid NOT NULL REFERENCES identity_private.account(account_id),
    assertion_type text NOT NULL,
    issuer text NOT NULL,
    subject_hash text NOT NULL,
    assurance_level text NOT NULL,
    territory_id uuid,
    valid_from timestamptz NOT NULL,
    valid_until timestamptz,
    revoked_at timestamptz,
    metadata jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.actor (
    actor_id uuid PRIMARY KEY,
    account_id uuid UNIQUE REFERENCES identity_private.account(account_id),
    actor_type text NOT NULL,
    public_handle text UNIQUE,
    display_name text,
    status text NOT NULL DEFAULT 'ACTIVE',
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.territory (
    territory_id uuid PRIMARY KEY,
    parent_territory_id uuid REFERENCES civic.territory(territory_id),
    territory_type text NOT NULL,
    canonical_code text NOT NULL UNIQUE,
    name text NOT NULL,
    geometry geometry(MultiPolygon, 4326),
    valid_from timestamptz NOT NULL DEFAULT now(),
    valid_until timestamptz,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX territory_geometry_gix ON civic.territory USING gist (geometry);

CREATE TABLE civic.topic (
    topic_id uuid PRIMARY KEY,
    parent_topic_id uuid REFERENCES civic.topic(topic_id),
    canonical_key text NOT NULL UNIQUE,
    display_name text NOT NULL,
    description text NOT NULL DEFAULT '',
    taxonomy_version text NOT NULL,
    active boolean NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.competence_node (
    competence_node_id uuid PRIMARY KEY,
    canonical_key text NOT NULL,
    territory_type text NOT NULL,
    scope_mode text NOT NULL,
    responsible_authority_type text NOT NULL,
    description text NOT NULL,
    registry_version text NOT NULL,
    valid_from timestamptz NOT NULL,
    valid_until timestamptz,
    source_refs jsonb NOT NULL DEFAULT '[]'::jsonb,
    UNIQUE (canonical_key, registry_version)
);

CREATE TABLE civic.topic_competence_route (
    route_id uuid PRIMARY KEY,
    topic_id uuid NOT NULL REFERENCES civic.topic(topic_id),
    competence_node_id uuid NOT NULL REFERENCES civic.competence_node(competence_node_id),
    route_version text NOT NULL,
    source_kind text NOT NULL,
    confidence numeric(5,4),
    source_refs jsonb NOT NULL DEFAULT '[]'::jsonb,
    valid_from timestamptz NOT NULL,
    valid_until timestamptz
);

CREATE TABLE civic.need (
    need_id uuid PRIMARY KEY,
    actor_id uuid NOT NULL REFERENCES civic.actor(actor_id),
    territory_id uuid NOT NULL REFERENCES civic.territory(territory_id),
    topic_id uuid REFERENCES civic.topic(topic_id),
    competence_node_id uuid REFERENCES civic.competence_node(competence_node_id),
    status text NOT NULL CHECK (status IN ('DRAFT','PUBLISHED','CLUSTERED','IN_DELIBERATION','ADDRESSED','CLOSED','WITHDRAWN')),
    current_version bigint NOT NULL DEFAULT 1,
    canonical_cluster_id uuid,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE INDEX need_territory_status_idx ON civic.need (territory_id, status, created_at DESC);
CREATE INDEX need_topic_status_idx ON civic.need (topic_id, status, created_at DESC);

CREATE TABLE civic.need_version (
    need_id uuid NOT NULL REFERENCES civic.need(need_id),
    version bigint NOT NULL,
    original_text text NOT NULL,
    public_summary text,
    language text NOT NULL DEFAULT 'es',
    content_sha256 text NOT NULL,
    route_explanation jsonb NOT NULL DEFAULT '{}'::jsonb,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (need_id, version)
);

ALTER TABLE civic.need
    ADD CONSTRAINT need_cluster_self_fk
    FOREIGN KEY (canonical_cluster_id) REFERENCES civic.need(need_id);

CREATE TABLE civic.need_support (
    need_id uuid NOT NULL REFERENCES civic.need(need_id),
    actor_id uuid NOT NULL REFERENCES civic.actor(actor_id),
    policy_version_id uuid NOT NULL REFERENCES policy.policy_version(policy_version_id),
    created_at timestamptz NOT NULL DEFAULT now(),
    revoked_at timestamptz,
    PRIMARY KEY (need_id, actor_id, policy_version_id)
);

CREATE TABLE civic.evidence (
    evidence_id uuid PRIMARY KEY,
    submitted_by uuid NOT NULL REFERENCES civic.actor(actor_id),
    evidence_type text NOT NULL,
    title text NOT NULL,
    source_uri text,
    object_storage_key text,
    content_sha256 text NOT NULL,
    verification_state text NOT NULL DEFAULT 'UNVERIFIED',
    visibility text NOT NULL DEFAULT 'PUBLIC',
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.claim (
    claim_id uuid PRIMARY KEY,
    actor_id uuid NOT NULL REFERENCES civic.actor(actor_id),
    need_id uuid REFERENCES civic.need(need_id),
    text text NOT NULL,
    status text NOT NULL DEFAULT 'ACTIVE',
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.claim_evidence (
    claim_id uuid NOT NULL REFERENCES civic.claim(claim_id),
    evidence_id uuid NOT NULL REFERENCES civic.evidence(evidence_id),
    relation text NOT NULL CHECK (relation IN ('SUPPORTS','CONTRADICTS','CONTEXT')),
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (claim_id, evidence_id, relation)
);

CREATE TABLE civic.technical_proposal (
    proposal_id uuid PRIMARY KEY,
    territory_id uuid NOT NULL REFERENCES civic.territory(territory_id),
    topic_id uuid REFERENCES civic.topic(topic_id),
    competence_node_id uuid REFERENCES civic.competence_node(competence_node_id),
    created_by uuid NOT NULL REFERENCES civic.actor(actor_id),
    status text NOT NULL,
    current_version bigint NOT NULL DEFAULT 1,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.proposal_version (
    proposal_id uuid NOT NULL REFERENCES civic.technical_proposal(proposal_id),
    version bigint NOT NULL,
    title text NOT NULL,
    impact_summary text NOT NULL,
    thesis text NOT NULL,
    technical_detail text NOT NULL,
    budget_model jsonb NOT NULL DEFAULT '{}'::jsonb,
    content_sha256 text NOT NULL,
    published_at timestamptz,
    supersedes_version bigint,
    created_at timestamptz NOT NULL DEFAULT now(),
    PRIMARY KEY (proposal_id, version)
);

CREATE TABLE civic.proposal_need_coverage (
    proposal_id uuid NOT NULL,
    proposal_version bigint NOT NULL,
    need_id uuid NOT NULL REFERENCES civic.need(need_id),
    coverage_type text NOT NULL,
    evidence_refs jsonb NOT NULL DEFAULT '[]'::jsonb,
    PRIMARY KEY (proposal_id, proposal_version, need_id),
    FOREIGN KEY (proposal_id, proposal_version) REFERENCES civic.proposal_version(proposal_id, version)
);

CREATE TABLE civic.civic_process (
    process_id uuid PRIMARY KEY,
    territory_id uuid NOT NULL REFERENCES civic.territory(territory_id),
    process_type text NOT NULL,
    legal_effect text NOT NULL,
    competent_authority_ref text,
    status text NOT NULL,
    starts_at timestamptz,
    ends_at timestamptz,
    policy_snapshot jsonb NOT NULL,
    policy_snapshot_sha256 text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE civic.eligibility_snapshot (
    process_id uuid NOT NULL REFERENCES civic.civic_process(process_id),
    actor_id uuid NOT NULL REFERENCES civic.actor(actor_id),
    eligible boolean NOT NULL,
    reason_codes text[] NOT NULL DEFAULT '{}',
    evaluated_at timestamptz NOT NULL,
    evaluation_hash text NOT NULL,
    PRIMARY KEY (process_id, actor_id)
);

CREATE TABLE civic.participation_event (
    participation_id uuid PRIMARY KEY,
    process_id uuid NOT NULL REFERENCES civic.civic_process(process_id),
    actor_id uuid NOT NULL REFERENCES civic.actor(actor_id),
    participation_kind text NOT NULL,
    target_type text NOT NULL,
    target_id uuid NOT NULL,
    target_version bigint,
    payload jsonb NOT NULL DEFAULT '{}'::jsonb,
    receipt_hash text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    revoked_at timestamptz
);

CREATE UNIQUE INDEX participation_active_unique_idx
    ON civic.participation_event(process_id, actor_id, participation_kind, target_type, target_id)
    WHERE revoked_at IS NULL;

CREATE TABLE institution.case_record (
    case_id uuid PRIMARY KEY,
    process_id uuid REFERENCES civic.civic_process(process_id),
    proposal_id uuid REFERENCES civic.technical_proposal(proposal_id),
    institution_ref text NOT NULL,
    status text NOT NULL,
    assigned_to_ref text,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE institution.official_response (
    response_id uuid PRIMARY KEY,
    case_id uuid NOT NULL REFERENCES institution.case_record(case_id),
    response_type text NOT NULL,
    body text NOT NULL,
    authority_ref text NOT NULL,
    source_document_ref text,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE institution.commitment (
    commitment_id uuid PRIMARY KEY,
    case_id uuid NOT NULL REFERENCES institution.case_record(case_id),
    title text NOT NULL,
    owner_ref text,
    budget_reference text,
    target_at timestamptz,
    status text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE institution.outcome (
    outcome_id uuid PRIMARY KEY,
    commitment_id uuid NOT NULL REFERENCES institution.commitment(commitment_id),
    status text NOT NULL,
    summary text NOT NULL,
    evidence_refs jsonb NOT NULL DEFAULT '[]'::jsonb,
    observed_at timestamptz,
    created_at timestamptz NOT NULL DEFAULT now()
);

CREATE TABLE search_projection.embedding (
    entity_id uuid NOT NULL,
    entity_type text NOT NULL,
    source_version bigint NOT NULL,
    chunk_ordinal integer NOT NULL DEFAULT 0,
    content_hash text NOT NULL,
    embedding_model_id text NOT NULL,
    embedding_model_version text NOT NULL,
    dimensions integer NOT NULL,
    vector_kind text NOT NULL,
    locale text,
    policy_classification text NOT NULL,
    embedding vector,
    created_at timestamptz NOT NULL DEFAULT now(),
    valid_until timestamptz,
    PRIMARY KEY (
      entity_id, entity_type, source_version, chunk_ordinal,
      embedding_model_id, embedding_model_version
    )
);

CREATE INDEX embedding_entity_lookup_idx
    ON search_projection.embedding(entity_type, entity_id, source_version);

CREATE TABLE audit.domain_event (
    event_id uuid PRIMARY KEY,
    aggregate_type text NOT NULL,
    aggregate_id uuid NOT NULL,
    aggregate_version bigint NOT NULL,
    event_type text NOT NULL,
    schema_version text NOT NULL,
    actor_ref uuid,
    correlation_id uuid NOT NULL,
    causation_id uuid,
    policy_refs jsonb NOT NULL DEFAULT '[]'::jsonb,
    payload jsonb NOT NULL,
    occurred_at timestamptz NOT NULL,
    recorded_at timestamptz NOT NULL DEFAULT now(),
    UNIQUE (aggregate_type, aggregate_id, aggregate_version)
);

CREATE TABLE audit.outbox (
    outbox_id uuid PRIMARY KEY,
    event_id uuid NOT NULL UNIQUE REFERENCES audit.domain_event(event_id),
    topic text NOT NULL,
    payload jsonb NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    claimed_at timestamptz,
    published_at timestamptz,
    attempt_count integer NOT NULL DEFAULT 0,
    last_error text
);

CREATE INDEX outbox_unpublished_idx
    ON audit.outbox(created_at)
    WHERE published_at IS NULL;

COMMIT;
