//! Ready checks — determine when a service is ready.

use crate::config::ReadyCheck;
use crate::ServiceId;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

/// Checks whether a service has become ready according to its config.
pub async fn wait_ready(
    id: &ServiceId,
    ready_checks: &[ReadyCheck],
    ready_delay: u64,
    mut output_matcher: impl FnMut(&str) -> bool,
) -> anyhow::Result<()> {
    // Apply initial delay.
    if ready_delay > 0 {
        sleep(Duration::from_secs(ready_delay)).await;
    }

    for check in ready_checks {
        match check {
            ReadyCheck::Delay(secs) => {
                sleep(Duration::from_secs(*secs)).await;
            }
            ReadyCheck::Output(pattern) => {
                let re = regex::Regex::new(pattern)?;
                // Poll output matcher until the pattern matches.
                let deadline = Duration::from_secs(30);
                let start = std::time::Instant::now();
                loop {
                    // sample accumulated output
                    if output_matcher(&re.to_string()) {
                        break;
                    }
                    if start.elapsed() > deadline {
                        anyhow::bail!(
                            "service {}: timed out waiting for ready output '{}'",
                            id,
                            pattern
                        );
                    }
                    sleep(Duration::from_millis(200)).await;
                }
            }
            ReadyCheck::Http {
                url,
                timeout_secs,
                expected_status,
            } => {
                wait_http(url, *timeout_secs, *expected_status).await?;
            }
            ReadyCheck::Port { port, timeout_secs } => {
                wait_port(*port, *timeout_secs).await?;
            }
            ReadyCheck::Cmd(cmd) => {
                let status = tokio::process::Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .status()
                    .await?;
                if !status.success() {
                    anyhow::bail!(
                        "service {}: ready command failed with {:?}",
                        id,
                        status.code()
                    );
                }
            }
        }
    }
    Ok(())
}

/// Wait until an HTTP endpoint responds successfully.
async fn wait_http(
    url: &str,
    timeout_secs: u64,
    expected_status: Option<u16>,
) -> anyhow::Result<()> {
    let deadline = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();

    // Parse the URL to get host and port.
    let (host, port) = if let Some(rest) = url.strip_prefix("http://") {
        let (h, rest) = rest.split_once(':').unwrap_or((rest, "80"));
        let port: u16 = rest.split('/').next().unwrap_or("80").parse().unwrap_or(80);
        (h.to_string(), port)
    } else if let Some(rest) = url.strip_prefix("https://") {
        let (h, rest) = rest.split_once(':').unwrap_or((rest, "443"));
        let port: u16 = rest.split('/').next().unwrap_or("443").parse().unwrap_or(443);
        (h.to_string(), port)
    } else {
        anyhow::bail!("unsupported URL scheme: {url}");
    };

    loop {
        match TcpStream::connect((&host[..], port)).await {
            Ok(mut stream) => {
                // Send a minimal HTTP GET request.
                let request = format!(
                    "GET / HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\n\r\n"
                );
                if stream.write_all(request.as_bytes()).await.is_ok() {
                    // Read the response status line.
                    let mut buf = [0u8; 4096];
                    if stream.read(&mut buf).await.is_ok() {
                        let response = String::from_utf8_lossy(&buf);
                        if let Some(status_line) = response.lines().next() {
                            if let Some(status_str) = status_line.split_whitespace().nth(1) {
                                if let Ok(status) = status_str.parse::<u16>() {
                                    if let Some(exp) = expected_status {
                                        if status == exp {
                                            return Ok(());
                                        }
                                    } else if status < 500 {
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Err(_) => {}
        }
        if start.elapsed() > deadline {
            anyhow::bail!("timed out waiting for HTTP endpoint {url}");
        }
        sleep(Duration::from_millis(300)).await;
    }
}

/// Wait until a TCP port is open.
async fn wait_port(port: u16, timeout_secs: u64) -> anyhow::Result<()> {
    let deadline = Duration::from_secs(timeout_secs);
    let start = std::time::Instant::now();
    loop {
        if let Ok(stream) = TcpStream::connect(("127.0.0.1", port)).await {
            drop(stream);
            return Ok(());
        }
        if start.elapsed() > deadline {
            anyhow::bail!("timed out waiting for TCP port {port}");
        }
        sleep(Duration::from_millis(300)).await;
    }
}

/// Check if a port is currently in use.
pub async fn is_port_open(port: u16) -> bool {
    TcpStream::connect(("127.0.0.1", port))
        .await
        .map(|_| true)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_port() {
        // Port 59999 is unlikely to be open; should timeout quickly.
        let result = wait_port(59999, 1).await;
        assert!(result.is_err());
    }
}