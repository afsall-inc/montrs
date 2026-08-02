/// Spring physics simulation.
///
/// Based on Motion's spring.ts — uses Newton's method to approximate spring
/// roots for duration-based springs, or simulates a damped harmonic oscillator.
///
/// # Example
/// ```rust,ignore
/// use montrs_motion::Spring;
///
/// let spring = Spring::new(100.0, 10.0, 1.0);  // stiffness, damping, mass
/// let value = spring.solve(0.5);  // value at time 0.5s
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Spring {
    stiffness: f64,
    damping: f64,
    mass: f64,
    velocity: f64,
    initial: f64,
    target: f64,
}

impl Default for Spring {
    fn default() -> Self {
        Self {
            stiffness: 100.0,
            damping: 10.0,
            mass: 1.0,
            velocity: 0.0,
            initial: 0.0,
            target: 1.0,
        }
    }
}

impl Spring {
    pub fn new(stiffness: f64, damping: f64, mass: f64) -> Self {
        Self {
            stiffness,
            damping,
            mass,
            ..Default::default()
        }
    }

    pub fn with_velocity(mut self, velocity: f64) -> Self {
        self.velocity = velocity;
        self
    }

    pub fn with_range(mut self, from: f64, to: f64) -> Self {
        self.initial = from;
        self.target = to;
        self
    }

    /// Solve the spring at time `t` (seconds).
    /// Returns the interpolated value between `initial` and `target`.
    pub fn solve(&self, t: f64) -> f64 {
        let w0 = (self.stiffness / self.mass).sqrt();
        let zeta = self.damping / (2.0 * (self.stiffness * self.mass).sqrt());
        let range = self.target - self.initial;

        if zeta < 1.0 {
            // Underdamped
            let wd = w0 * (1.0 - zeta * zeta).sqrt();
            let a = 1.0;
            let b = (self.velocity + zeta * w0) / wd;
            let envelope = (-zeta * w0 * t).exp();
            let result = range * (1.0 - envelope * (a * (wd * t).cos() + b * (wd * t).sin()));
            self.initial + result
        } else {
            // Critically damped or overdamped
            let c1 = 1.0;
            let c2 = self.velocity + zeta * w0;
            let envelope = (-zeta * w0 * t).exp();
            let result = range * (1.0 - envelope * (c1 + c2 * t));
            self.initial + result
        }
    }

    /// Estimate the duration needed for the spring to settle within `threshold`.
    pub fn duration(&self, threshold: f64) -> f64 {
        let mut t = 0.0;
        let dt = 1.0 / 60.0;
        loop {
            let val = self.solve(t);
            if (val - self.target).abs() < threshold {
                return t;
            }
            t += dt;
            if t > 10.0 {
                return 10.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_default() {
        let spring = Spring::default();
        let start = spring.solve(0.0);
        let end = spring.solve(2.0);
        assert!((start - 0.0).abs() < 0.001);
        assert!((end - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_spring_stiff() {
        let spring = Spring::new(300.0, 20.0, 1.0);
        let mid = spring.solve(0.1);
        assert!(mid > 0.0);
        assert!(spring.duration(0.01) < 2.0);
    }

    #[test]
    fn test_spring_velocity() {
        let spring = Spring::new(100.0, 10.0, 1.0).with_velocity(50.0);
        let val = spring.solve(0.01);
        assert!(val > 0.0);
    }
}