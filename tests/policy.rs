use vertex_dev_generator_plane::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 83, capacity: 101, latency: 26, risk: 6, weight: 9 };
    assert_eq!(score(signal), 209);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 64, capacity: 92, latency: 9, risk: 8, weight: 12 };
    assert_eq!(score(signal), 194);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 76, capacity: 81, latency: 25, risk: 23, weight: 11 };
    assert_eq!(score(signal), 113);
    assert_eq!(classify(signal), "review");
}
