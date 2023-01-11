/// Defines a range between min and max inclusive
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

/// Contains nothing
pub const EMPTY_INTERVAL: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};
/// Contains everything
pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};

/// creates a new interval that is the union of the two given.
/// If there is a gap between the intervals, that is included in the returned interval.
pub fn combine_intervals(a: Interval, b: Interval) -> Interval {
    Interval {
        min: a.min.min(b.min),
        max: a.max.max(b.max),
    }
}

impl Interval {
    /// Checks if the interval contains a given value
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// returns the given value clamped to the interval
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    /// return the size of the interval
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// returns a new interval that is larger by given value delta.
    /// Delta is added equally to both sides of the interval
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    /// returns a new interval that is increased with given value.
    /// The returned interval has same size as original
    pub fn add(&self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::util::interval::{combine_intervals, Interval};

    #[test]
    fn test_combine_intervals() {
        let mut res =
            combine_intervals(Interval { min: 0., max: 2. }, Interval { min: 1., max: 3. });
        assert_eq!(Interval { min: 0., max: 3. }, res);

        res = combine_intervals(Interval { min: 0., max: 1. }, Interval { min: 2., max: 3. });
        assert_eq!(Interval { min: 0., max: 3. }, res);

        res = combine_intervals(
            Interval { min: 3., max: 3. },
            Interval { min: -1., max: -1. },
        );
        assert_eq!(Interval { min: -1., max: 3. }, res);
    }

    #[test]
    fn test_contains() {
        let interval = Interval { min: -2., max: 2. };
        assert!(!interval.contains(-3.));
        assert!(interval.contains(-2.));
        assert!(interval.contains(2.));
        assert!(!interval.contains(3.));
    }

    #[test]
    fn test_clamp() {
        let interval = Interval { min: -2., max: 2. };
        assert_eq!(-2., interval.clamp(-3.));
        assert_eq!(-2., interval.clamp(-2.));
        assert_eq!(0., interval.clamp(-0.));
        assert_eq!(2., interval.clamp(2.));
        assert_eq!(2., interval.clamp(3.));
    }

    #[test]
    fn test_size() {
        assert_eq!(0., Interval { min: 0., max: 0. }.size());
        assert_eq!(2., Interval { min: -1., max: 1. }.size());
        assert_eq!(-2., Interval { min: 1., max: -1. }.size());
    }

    #[test]
    fn test_expand() {
        let interval = Interval { min: -2., max: 2. };
        assert_eq!(Interval { min: -3., max: 3. }, interval.expand(2.));
        assert_eq!(
            Interval {
                min: -3.5,
                max: 3.5
            },
            interval.expand(3.)
        );
        assert_eq!(Interval { min: -1., max: 1. }, interval.expand(-2.));
    }

    #[test]
    fn test_add() {
        let interval = Interval { min: -2., max: 2. };
        assert_eq!(Interval { min: 0., max: 4. }, interval.add(2.));
        assert_eq!(Interval { min: 1., max: 5. }, interval.add(3.));
        assert_eq!(Interval { min: -4., max: 0. }, interval.add(-2.));
    }
}
