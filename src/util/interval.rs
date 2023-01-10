/// Defines a range between min and max inclusive
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

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
