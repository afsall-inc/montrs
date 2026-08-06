/// SVG path animation helpers.
///
/// Provides utilities for path drawing (stroke animation) and morphing,
/// inspired by how Lucide Animated Icons uses `pathLength` and `pathOffset`.
/// Parse an SVG path `d` attribute and compute its total length.
/// Supports M/m, L/l, C/c, Q/q, A/a, Z/z commands.
pub fn approximate_path_length(path_d: &str) -> f64 {
    let tokens = tokenize(path_d);
    let mut total = 0.0;
    let mut i = 0;
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let mut start_x = 0.0;
    let mut start_y = 0.0;
    let mut first = true;

    while i < tokens.len() {
        let cmd = tokens[i].as_str();
        i += 1;

        match cmd {
            "M" | "m" => {
                let rel = cmd == "m";
                if i + 1 < tokens.len() {
                    let x = parse_f64(&tokens[i]);
                    let y = parse_f64(&tokens[i + 1]);
                    i += 2;
                    if rel {
                        current_x += x;
                        current_y += y;
                    } else {
                        current_x = x;
                        current_y = y;
                    }
                    if first {
                        start_x = current_x;
                        start_y = current_y;
                        first = false;
                    }
                }
                // Followed by implicit L/l commands
                while i + 1 < tokens.len() && is_number(&tokens[i]) {
                    let x = parse_f64(&tokens[i]);
                    let y = parse_f64(&tokens[i + 1]);
                    i += 2;
                    let nx = if rel { current_x + x } else { x };
                    let ny = if rel { current_y + y } else { y };
                    total += line_length(current_x, current_y, nx, ny);
                    current_x = nx;
                    current_y = ny;
                }
            }
            "L" | "l" => {
                let rel = cmd == "l";
                while i + 1 < tokens.len() && is_number(&tokens[i]) {
                    let x = parse_f64(&tokens[i]);
                    let y = parse_f64(&tokens[i + 1]);
                    i += 2;
                    let nx = if rel { current_x + x } else { x };
                    let ny = if rel { current_y + y } else { y };
                    total += line_length(current_x, current_y, nx, ny);
                    current_x = nx;
                    current_y = ny;
                }
            }
            "C" | "c" => {
                let rel = cmd == "c";
                while i + 5 < tokens.len() && is_number(&tokens[i]) {
                    let c1x = parse_f64(&tokens[i]);
                    let c1y = parse_f64(&tokens[i + 1]);
                    let c2x = parse_f64(&tokens[i + 2]);
                    let c2y = parse_f64(&tokens[i + 3]);
                    let ex = parse_f64(&tokens[i + 4]);
                    let ey = parse_f64(&tokens[i + 5]);
                    i += 6;
                    let nc1x = if rel { current_x + c1x } else { c1x };
                    let nc1y = if rel { current_y + c1y } else { c1y };
                    let nc2x = if rel { current_x + c2x } else { c2x };
                    let nc2y = if rel { current_y + c2y } else { c2y };
                    let nex = if rel { current_x + ex } else { ex };
                    let ney = if rel { current_y + ey } else { ey };
                    total += cubic_bezier_length(
                        current_x, current_y, nc1x, nc1y, nc2x, nc2y, nex, ney,
                    );
                    current_x = nex;
                    current_y = ney;
                }
            }
            "Q" | "q" => {
                let rel = cmd == "q";
                while i + 3 < tokens.len() && is_number(&tokens[i]) {
                    let cx = parse_f64(&tokens[i]);
                    let cy = parse_f64(&tokens[i + 1]);
                    let ex = parse_f64(&tokens[i + 2]);
                    let ey = parse_f64(&tokens[i + 3]);
                    i += 4;
                    let ncx = if rel { current_x + cx } else { cx };
                    let ncy = if rel { current_y + cy } else { cy };
                    let nex = if rel { current_x + ex } else { ex };
                    let ney = if rel { current_y + ey } else { ey };
                    total += quad_bezier_length(
                        current_x, current_y, ncx, ncy, nex, ney,
                    );
                    current_x = nex;
                    current_y = ney;
                }
            }
            "A" | "a" => {
                let rel = cmd == "a";
                while i + 6 < tokens.len() && is_number(&tokens[i]) {
                    let rx = parse_f64(&tokens[i]);
                    let ry = parse_f64(&tokens[i + 1]);
                    let _x_rot = parse_f64(&tokens[i + 2]);
                    let _large = parse_f64(&tokens[i + 3]);
                    let _sweep = parse_f64(&tokens[i + 4]);
                    let ex = parse_f64(&tokens[i + 5]);
                    let ey = parse_f64(&tokens[i + 6]);
                    i += 7;
                    let nex = if rel { current_x + ex } else { ex };
                    let ney = if rel { current_y + ey } else { ey };
                    // Approximate arc length as elliptical perimeter segment
                    let approx = (rx + ry) * std::f64::consts::PI / 4.0;
                    total += approx;
                    current_x = nex;
                    current_y = ney;
                }
            }
            "Z" | "z" => {
                if !first {
                    total +=
                        line_length(current_x, current_y, start_x, start_y);
                    current_x = start_x;
                    current_y = start_y;
                }
            }
            _ => {
                // Unknown command, skip
            }
        }
    }

    total
}

fn tokenize(d: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut last_was_cmd = false;

    for ch in d.chars() {
        if ch.is_ascii_alphabetic() {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(ch.to_string());
            last_was_cmd = true;
        } else if ch == '-' || ch == '.' || ch.is_ascii_digit() {
            if last_was_cmd && ch == '-' {
                // Start of negative number after command
                current.push(ch);
            } else if ch == '.' && current.contains('.') {
                // Second dot, push current and start new number
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                current.push(ch);
            } else {
                current.push(ch);
            }
            last_was_cmd = false;
        } else if ch == ','
            || ch == ' '
            || ch == '\t'
            || ch == '\n'
            || ch == '\r'
        {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            last_was_cmd = false;
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn is_number(s: &str) -> bool {
    !s.is_empty()
        && (s.as_bytes()[0].is_ascii_digit()
            || s == "-"
            || s.starts_with('-')
            || s == ".")
}

fn parse_f64(s: &str) -> f64 {
    s.parse().unwrap_or(0.0)
}

fn line_length(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let dx = x2 - x1;
    let dy = y2 - y1;
    (dx * dx + dy * dy).sqrt()
}

#[allow(clippy::too_many_arguments)]
fn cubic_bezier_length(
    x1: f64,
    y1: f64,
    cx1: f64,
    cy1: f64,
    cx2: f64,
    cy2: f64,
    x2: f64,
    y2: f64,
) -> f64 {
    // Adaptive subdivision: sample at 16 segments
    let steps = 16;
    let mut total = 0.0;
    let mut prev_x = x1;
    let mut prev_y = y1;
    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let mt = 1.0 - t;
        let x = mt * mt * mt * x1
            + 3.0 * mt * mt * t * cx1
            + 3.0 * mt * t * t * cx2
            + t * t * t * x2;
        let y = mt * mt * mt * y1
            + 3.0 * mt * mt * t * cy1
            + 3.0 * mt * t * t * cy2
            + t * t * t * y2;
        total += line_length(prev_x, prev_y, x, y);
        prev_x = x;
        prev_y = y;
    }
    total
}

fn quad_bezier_length(
    x1: f64,
    y1: f64,
    cx: f64,
    cy: f64,
    x2: f64,
    y2: f64,
) -> f64 {
    let steps = 12;
    let mut total = 0.0;
    let mut prev_x = x1;
    let mut prev_y = y1;
    for i in 1..=steps {
        let t = i as f64 / steps as f64;
        let mt = 1.0 - t;
        let x = mt * mt * x1 + 2.0 * mt * t * cx + t * t * x2;
        let y = mt * mt * y1 + 2.0 * mt * t * cy + t * t * y2;
        total += line_length(prev_x, prev_y, x, y);
        prev_x = x;
        prev_y = y;
    }
    total
}

/// Generate a CSS `stroke-dasharray` value for path drawing animation.
pub fn stroke_dasharray(length: f64) -> String {
    format!("{} {}", length, length)
}

/// Generate a CSS `stroke-dashoffset` value for a given progress (0.0 to 1.0).
pub fn stroke_dashoffset(length: f64, progress: f64) -> String {
    format!("{}", length * (1.0 - progress))
}

/// Animation variants for SVG path drawing.
/// Use with `MotionValue` or `Tween` to animate SVG path draw-in.
pub struct PathDrawAnimation {
    pub path_length: f64,
    pub progress: f64,
}

impl PathDrawAnimation {
    pub fn new(path_d: &str) -> Self {
        Self {
            path_length: approximate_path_length(path_d),
            progress: 0.0,
        }
    }

    pub fn from_length(path_length: f64) -> Self {
        Self {
            path_length,
            progress: 0.0,
        }
    }

    pub fn stroke_dasharray(&self) -> String {
        stroke_dasharray(self.path_length)
    }

    pub fn stroke_dashoffset(&self) -> String {
        stroke_dashoffset(self.path_length, self.progress)
    }
}
