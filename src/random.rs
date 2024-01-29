//! A wrapper for the random number generator to be used by ray tracer.

/// returns a random float 0 to <1
pub fn random_normal_float() -> f64 {
    fastrand::f64()
}

/// returns a random float min to <max
pub fn random_float(min: f64, max: f64) -> f64 {
    fastrand::f64() * (max - min) + min
}

/// returns a random uint32 0 to <max
pub fn random_uint32(max: u32) -> u32 {
    fastrand::u32(0..max)
}

/// Returns a random element from the given vector
pub fn random_element_index<T>(v: &[T]) -> usize {
    fastrand::usize(..v.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_normal_float() {
        for _ in 0..100 {
            let r = random_normal_float();
            assert!((0. ..1.).contains(&r))
        }
    }

    #[test]
    fn test_random_float() {
        for _ in 0..100 {
            let r = random_float(-2., 2.);
            assert!((-2. ..2.).contains(&r))
        }
    }

    #[test]
    fn test_random_uint32() {
        for _ in 0..100 {
            let r = random_uint32(100);
            assert!(r < 100)
        }
    }

    #[test]
    fn test_random_element_index() {
        let list = vec![1, 2, 3, 4, 5];

        for _ in 0..100 {
            let r = random_element_index(&list);
            assert!(r < list.len())
        }
    }
}
