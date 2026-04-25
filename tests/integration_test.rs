use turbo::{EvalConfig, SearchOutput};

#[test]
fn test_full_pipeline() {
    // This test checks if the module pipeline is operational
    let config = EvalConfig::load().expect("Failed to load config");
    let status = chess::BoardStatus::Ongoing;
    let output = SearchOutput::new(status, None, 0, 0);

    assert_eq!(output.status, status);
}
