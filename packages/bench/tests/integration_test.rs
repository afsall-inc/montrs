use montrs_bench::{Parameter, Weight, stats::BenchStats};

#[test]
fn test_linear_regression_stats() {
    // Perfect linear correlation: y = 10x + 100
    // x: 10, 20, 30
    // y: 200, 300, 400 (ns) -> 0.2, 0.3, 0.4 (µs) -> 2e-7, 3e-7, 4e-7 (s)

    // BenchStats expects durations in seconds
    let durations = vec![
        std::time::Duration::from_nanos(200),
        std::time::Duration::from_nanos(300),
        std::time::Duration::from_nanos(400),
    ];

    let params = vec![10, 20, 30];

    let stats = BenchStats::with_params(&durations, Some(&params));

    // Check Slope (should be 10ns -> 1e-8 s)
    let slope = stats.slope.expect("Slope should be calculated");
    assert!(
        (slope - 1e-8).abs() < 1e-12,
        "Slope mismatch: {} != 1e-8",
        slope
    );

    // Check Intercept (should be 100ns -> 1e-7 s)
    let intercept = stats.intercept.expect("Intercept should be calculated");
    assert!(
        (intercept - 1e-7).abs() < 1e-12,
        "Intercept mismatch: {} != 1e-7",
        intercept
    );
}

#[test]
fn test_weight_calculation() {
    // Cost = Base + (Slope * N)
    // Base = 1000ns
    // Slope = 10ns/item

    let weight = Weight::from_ns(1000, 10);

    assert_eq!(weight.calc(0), 1000);
    assert_eq!(weight.calc(1), 1010);
    assert_eq!(weight.calc(100), 2000);

    // Test saturation
    let huge_weight = Weight::from_ns(u64::MAX - 10, 10);
    assert_eq!(huge_weight.calc(10), u64::MAX); // Should saturate
}

#[test]
fn test_parameter_iteration() {
    let param = Parameter::new("test", 1..=5).with_step(2);
    let values = param.values();

    assert_eq!(values, vec![1, 3, 5]);
}
