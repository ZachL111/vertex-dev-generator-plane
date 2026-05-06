use vertex_dev_generator_plane::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 74, slack: 38, drag: 24, confidence: 61 };
    assert_eq!(review_score(case), 175);
    assert_eq!(review_lane(case), "ship");
}
