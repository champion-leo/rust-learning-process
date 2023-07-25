use rand::Rng;

// Constants

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

// Utility Functions

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..=max)
}

#[cfg(test)]
mod helper {
    use super::*;

    #[test]
    fn test_random() {
        let x = random();
        assert!(x >= 0.0 && x <= 1.0);
    }

    #[test]
    fn test_random_range() {
        let x = random_range(0.0, 1.0);
        assert!(x >= 0.0 && x <= 1.0);
    }
}