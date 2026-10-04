use intake_references::{ExtractedReference, ReferenceKind, references};

fn found(intent: &str) -> Vec<(ReferenceKind, String)> {
    references(intent)
        .into_iter()
        .map(|ExtractedReference { kind, value }| (kind, value))
        .collect()
}

#[test]
fn intent_references_are_extracted_in_order() {
    let intent = "fix DEV-630, see https://acme.atlassian.net/browse/DEV-630 and \
https://gitlab.example.com/team/app/-/merge_requests/42 and the thread \
https://example.slack.com/archives/C01/p1700000000000100, also https://acme.atlassian.net/browse/OPS-7, \
https://gitlab.example.com/team/app/-/issues/9, https://github.com/beyond10x/intake/pull/3 and \
https://example.org/design.pdf";

    let expected = vec![
        (ReferenceKind::JiraIssue, "DEV-630".to_owned()),
        (ReferenceKind::GitlabMergeRequest, "team/app!42".to_owned()),
        (
            ReferenceKind::SlackMessage,
            "C01/1700000000.000100".to_owned(),
        ),
        (ReferenceKind::JiraIssue, "OPS-7".to_owned()),
        (ReferenceKind::GitlabIssue, "team/app#9".to_owned()),
        (
            ReferenceKind::GithubPullRequest,
            "beyond10x/intake#3".to_owned(),
        ),
        (
            ReferenceKind::Url,
            "https://example.org/design.pdf".to_owned(),
        ),
    ];

    assert_eq!(found(intent), expected);
}

#[test]
fn lower_case_keys_and_email_addresses_yield_nothing() {
    assert_eq!(found("fix dev-630 please"), Vec::new());
    assert_eq!(found("write to ops.team@example.org about it"), Vec::new());
}
