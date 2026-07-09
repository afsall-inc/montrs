//! Test command implementation for MontRS.

//! This plate handles the execution of unit and integration tests. It wraps `cargo test`
//! but adds MontRS-specific capabilities like custom reporting (JSON/JUnit) and
//! automated environment setup.

use crate::config::MontrsConfig;
use quick_xml::{
    Writer,
    events::{BytesDecl, BytesStart, Event},
};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};

/// Runs the test suite for the current project.
///
/// This function:
/// 1. Loads the `MontrsConfig` to verify the project context.
/// 2. Constructs arguments for `cargo test`.
/// 3. Spawns `cargo test` as a subprocess.
/// 4. Optionally captures JSON output to generate JUnit/JSON reports.
///
/// # Arguments
///
/// * `filter` - Optional filter string to run specific tests (passed to `cargo test`).
/// * `report` - The format of the report to generate ("human", "json", "junit").
/// * `output` - Optional path to write the report file.
/// * `jobs` - Number of parallel jobs to run.
pub async fn run(
    filter: Option<String>,
    report: String,
    output: Option<String>,
    jobs: Option<usize>,
) -> anyhow::Result<()> {
    // If human report and no special processing, and no filter/jobs,
    // delegate to cargo-leptos to handle wasm/server split correctly if possible.
    // However, for consistency with 'unit testing', we prefer standard cargo test.
    // cargo-leptos test_all is good but we want control.
    // We'll run cargo test directly.

    // Load config just to ensure valid project
    let _ = MontrsConfig::load()?;

    println!("Running MontRS Unit Tests...");

    let mut args = vec!["test".to_string(), "--workspace".to_string()];

    if let Some(f) = filter {
        args.push(f);
    }

    if let Some(j) = jobs {
        args.push("-j".to_string());
        args.push(j.to_string());
    }

    // Always use JSON format internally if we need to generate reports
    let use_json_internal = report == "json" || report == "junit";
    if use_json_internal {
        args.push("--message-format=json".to_string());
    }

    let mut cmd = tokio::process::Command::new("cargo");
    cmd.args(&args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::inherit()); // Let build logs show up on stderr

    let mut child = cmd
        .spawn()
        .map_err(|e| anyhow::anyhow!("Failed to spawn cargo test: {}", e))?;

    if !use_json_internal {
        // Just wait for it
        let status = child.wait().await?;
        if !status.success() {
            anyhow::bail!("Tests failed");
        }
        return Ok(());
    }

    // Process JSON output
    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout).lines();

    let mut test_suites = Vec::new();
    let mut current_suite = TestSuite::default();

    // Simple parser for cargo test json
    while let Some(line) = reader.next_line().await? {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
            if let Some(type_field) = json.get("type").and_then(|v| v.as_str())
            {
                if type_field == "test" {
                    // Handle test event
                    let event = json
                        .get("event")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let name = json
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown");

                    match event {
                        "ok" => {
                            current_suite.tests.push(TestCase {
                                name: name.to_string(),
                                status: TestStatus::Pass,
                                message: None,
                                duration: json
                                    .get("exec_time")
                                    .and_then(|v| v.as_f64())
                                    .unwrap_or(0.0),
                            });
                            println!("PASS: {}", name);
                        }
                        "failed" => {
                            let stdout =
                                json.get("stdout").and_then(|v| v.as_str());
                            current_suite.tests.push(TestCase {
                                name: name.to_string(),
                                status: TestStatus::Fail,
                                message: stdout.map(|s| s.to_string()),
                                duration: 0.0,
                            });
                            println!("FAIL: {}", name);
                        }
                        _ => {}
                    }
                } else if type_field == "suite" {
                    let event = json
                        .get("event")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if event == "started" {
                        // New suite? cargo test often runs multiple binaries
                        if !current_suite.tests.is_empty() {
                            test_suites.push(current_suite);
                            current_suite = TestSuite::default();
                        }
                        // Try to get suite name from artifact? hard with just stream
                    } else if event == "ok" || event == "failed" {
                        // Suite finished
                    }
                }
            }
        }
    }

    if !current_suite.tests.is_empty() {
        test_suites.push(current_suite);
    }

    let status = child.wait().await?;

    if report == "junit" {
        let output_path = output.unwrap_or_else(|| "report.xml".to_string());
        generate_junit_report(&test_suites, &output_path)?;
        println!("JUnit report generated at {}", output_path);
    } else if report == "json" {
        let output_path = output.unwrap_or_else(|| "report.json".to_string());
        let f = std::fs::File::create(&output_path)?;
        serde_json::to_writer_pretty(f, &test_suites)?;
        println!("JSON report generated at {}", output_path);
    }

    if !status.success() {
        anyhow::bail!("Tests failed");
    }

    Ok(())
}

#[derive(Default, serde::Serialize)]
struct TestSuite {
    name: String,
    tests: Vec<TestCase>,
}

#[derive(serde::Serialize)]
struct TestCase {
    name: String,
    status: TestStatus,
    message: Option<String>,
    duration: f64,
}

#[derive(serde::Serialize)]
enum TestStatus {
    Pass,
    Fail,
    #[allow(dead_code)]
    Ignored,
}

/// Generates a JUnit XML report from the test results.
fn generate_junit_report(
    suites: &[TestSuite],
    path: &str,
) -> anyhow::Result<()> {
    let mut writer =
        Writer::new_with_indent(std::fs::File::create(path)?, b' ', 4);

    writer.write_event(Event::Decl(BytesDecl::new(
        "1.0",
        Some("UTF-8"),
        None,
    )))?;

    let root = BytesStart::new("testsuites");
    writer.write_event(Event::Start(root.clone()))?;

    for (i, suite) in suites.iter().enumerate() {
        let mut elem = BytesStart::new("testsuite");
        elem.push_attribute(("name", format!("suite-{}", i).as_str()));
        elem.push_attribute(("tests", suite.tests.len().to_string().as_str()));
        elem.push_attribute((
            "failures",
            suite
                .tests
                .iter()
                .filter(|t| matches!(t.status, TestStatus::Fail))
                .count()
                .to_string()
                .as_str(),
        ));

        writer.write_event(Event::Start(elem.clone()))?;

        for test in &suite.tests {
            let mut t = BytesStart::new("testcase");
            t.push_attribute(("name", test.name.as_str()));
            t.push_attribute(("time", test.duration.to_string().as_str()));

            if let TestStatus::Fail = test.status {
                writer.write_event(Event::Start(t.clone()))?;

                let mut failure = BytesStart::new("failure");
                failure.push_attribute(("message", "Test failed"));
                writer.write_event(Event::Start(failure.clone()))?;
                if let Some(msg) = &test.message {
                    writer.write_event(Event::Text(
                        quick_xml::events::BytesText::new(msg),
                    ))?;
                }
                writer.write_event(Event::End(failure.to_end()))?;

                writer.write_event(Event::End(t.to_end()))?;
            } else {
                writer.write_event(Event::Empty(t))?;
            }
        }

        writer.write_event(Event::End(elem.to_end()))?;
    }

    writer.write_event(Event::End(root.to_end()))?;
    Ok(())
}
