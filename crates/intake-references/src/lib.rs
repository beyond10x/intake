//! Deterministic extraction of an intent's key references (story intent-references).
//!
//! The types mirror `intake.routing.ReferenceKind` and `intake.routing.ExtractedReference` in
//! `ess/domains/routing.yaml`. A reference carries its kind and canonical value; its identity and
//! owning intent are assigned where the intent is recorded.

/// What a first-level extractor recognises in an intent (`intake.routing.ReferenceKind`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReferenceKind {
    JiraIssue,
    SlackMessage,
    GitlabIssue,
    GitlabMergeRequest,
    GithubIssue,
    GithubPullRequest,
    Url,
}

/// A key element found in an intent's text, with its canonical value
/// (`intake.routing.ExtractedReference`).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ExtractedReference {
    pub kind: ReferenceKind,
    pub value: String,
}
