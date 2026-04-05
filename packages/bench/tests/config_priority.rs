use montrs_bench::BenchConfig;
use std::collections::HashMap;
use std::time::Duration;

#[test]
fn test_config_priority() {
    // 1. Default values
    let empty_env = HashMap::<String, String>::new();
    let config =
        BenchConfig::build_with_env(vec!["bench"], |key: &str| empty_env.get(key).cloned());
    assert_eq!(config.warmup_iterations, 10);
    assert_eq!(config.iterations, 100);
    assert_eq!(config.duration, Some(Duration::from_secs(5)));

    // 2. Env Vars
    let mut env = HashMap::new();
    env.insert("MONTRS_BENCH_WARMUP".to_string(), "30".to_string());

    let config = BenchConfig::build_with_env(vec!["bench"], |key: &str| env.get(key).cloned());
    assert_eq!(config.warmup_iterations, 30);

    // 3. CLI Args (Priority over Env)
    let mut env_with_val = HashMap::new();
    env_with_val.insert("MONTRS_BENCH_WARMUP".to_string(), "30".to_string());

    // We pass args explicitly
    let config = BenchConfig::build_with_env(vec!["bench", "--warmup", "40"], |key: &str| {
        env_with_val.get(key).cloned()
    });
    assert_eq!(config.warmup_iterations, 40); // Args win

    // 4. Duration parsing via CLI
    let config = BenchConfig::build_with_env(vec!["bench", "--timeout", "10"], |key: &str| {
        empty_env.get(key).cloned()
    });
    assert_eq!(config.duration, Some(Duration::from_secs(10)));

    // 5. Filter and JSON output
    let config = BenchConfig::build_with_env(
        vec![
            "bench",
            "--filter",
            "my_test",
            "--json-output",
            "report.json",
        ],
        |key: &str| empty_env.get(key).cloned(),
    );
    assert_eq!(config.filter, Some("my_test".to_string()));
    assert_eq!(config.json_output, Some("report.json".to_string()));
}
