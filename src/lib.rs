use std::f32::consts::E;

pub trait Interpolator {
    /// Evaluates the interpolation function (f) at x: f(x).
    fn eval(&self, x: f32) -> f32;

    /// Checks whether x is greater than the domain of operation of the
    /// interpolation function. It's guaranteed that if this returns true for
    /// x', f(x'+e) = f(x) where e >= 0.
    fn exceeds_domain(&self, x: f32) -> bool;
}

pub struct ClosedInterval {
    bound: (f32, f32),
    length: f32,
}

impl ClosedInterval {
    fn new(bound: (f32, f32)) -> ClosedInterval {
        Self::check_bound(&bound);
        ClosedInterval {
            bound,
            length: bound.1 - bound.0,
        }
    }

    fn check_bound(bound: &(f32, f32)) {
        if bound.0 >= bound.1 {
            // Degenerate & empty intervals not allowed.
            panic!("Invalid interval: {} !< {}", bound.0, bound.1);
        }
    }
}

pub struct StepInterpolator {
    domain: ClosedInterval,
    range: ClosedInterval,
}

impl StepInterpolator {
    pub fn new(domain: (f32, f32), range: (f32, f32)) -> StepInterpolator {
        let domain_interval = ClosedInterval::new(domain);
        let range_interval = ClosedInterval::new(range);
        StepInterpolator {
            domain: domain_interval,
            range: range_interval,
        }
    }
}

impl Interpolator for StepInterpolator {
    fn eval(&self, x: f32) -> f32 {
        if x <= self.domain.bound.0 {
            self.range.bound.0
        } else {
            self.range.bound.1
        }
    }

    fn exceeds_domain(&self, x: f32) -> bool {
        x >= self.domain.bound.1
    }
}

pub struct NearestNeighborInterpolator {
    domain: ClosedInterval,
    range: ClosedInterval,
    midpoint: f32,
}

impl NearestNeighborInterpolator {
    pub fn new(domain: (f32, f32), range: (f32, f32)) -> NearestNeighborInterpolator {
        let domain_interval = ClosedInterval::new(domain);
        let range_interval = ClosedInterval::new(range);
        let midpoint = (domain_interval.bound.1 - domain_interval.bound.0) / 2.0
            + domain_interval.bound.0;
        NearestNeighborInterpolator {
            domain: domain_interval,
            range: range_interval,
            midpoint,
        }
    }
}

impl Interpolator for NearestNeighborInterpolator {
    fn eval(&self, x: f32) -> f32 {
        if x <= self.midpoint {
            self.range.bound.0
        } else {
            self.range.bound.1
        }
    }

    fn exceeds_domain(&self, x: f32) -> bool {
        x >= self.domain.bound.1
    }
}

pub struct LinearInterpolator {
    domain: ClosedInterval,
    range: ClosedInterval,
    slope: f32,
}

impl LinearInterpolator {
    pub fn new(domain: (f32, f32), range: (f32, f32)) -> LinearInterpolator {
        let domain_interval = ClosedInterval::new(domain);
        let range_interval = ClosedInterval::new(range);
        let slope = range_interval.length / domain_interval.length;
        LinearInterpolator {
            domain: domain_interval,
            range: range_interval,
            slope,
        }
    }
}

impl Interpolator for LinearInterpolator {
    fn eval(&self, x: f32) -> f32 {
        if x <= self.domain.bound.0 {
            return self.range.bound.0;
        } else if  x>= self.domain.bound.1 {
            return self.range.bound.1;
        }
        (x - self.domain.bound.0) * self.slope + self.range.bound.0
    }

    fn exceeds_domain(&self, x: f32) -> bool {
        x >= self.domain.bound.1
    }
}

pub struct SigmoidInterpolator {
    domain: ClosedInterval,
    range: ClosedInterval,
}

impl SigmoidInterpolator {
    pub fn new(domain: (f32, f32), range: (f32, f32)) -> SigmoidInterpolator {
        let domain_interval = ClosedInterval::new(domain);
        let range_interval = ClosedInterval::new(range);
        SigmoidInterpolator {
            domain: domain_interval,
            range: range_interval,
        }
    }
}

impl Interpolator for SigmoidInterpolator {
    fn eval(&self, x: f32) -> f32 {
        if x <= self.domain.bound.0 {
            return self.range.bound.0;
        } else if  x>= self.domain.bound.1 {
            return self.range.bound.1;
        }
        fn sigmoid(x: f32) -> f32 {
            1.0 / (1.0 + E.powf(-x))
        }
        let x_prime = (x - self.domain.bound.0)/self.domain.length * 8.0 - 4.0;
        sigmoid(x_prime) * self.range.length + self.range.bound.0
    }

    fn exceeds_domain(&self, x: f32) -> bool {
        x >= self.domain.bound.1
    }
}

#[cfg(test)]
mod tests {
    use super::Interpolator;

    #[test]
    fn step() {
        use super::StepInterpolator;
        let si = StepInterpolator::new((10.0, 20.0), (100.0, 200.0));
        assert_eq!(si.eval(9.0), 100.0);
        assert_eq!(si.eval(10.0), 100.0);
        assert_eq!(si.eval(11.0), 200.0);
        assert_eq!(si.eval(21.0), 200.0);

        assert_eq!(si.exceeds_domain(1.0), false);
        assert_eq!(si.exceeds_domain(15.0), false);
        assert_eq!(si.exceeds_domain(21.0), true);
    }

    #[test]
    fn nearest_neighbor() {
        use super::NearestNeighborInterpolator;
        let nni = NearestNeighborInterpolator::new((10.0, 20.0), (100.0, 200.0));
        assert_eq!(nni.eval(9.0), 100.0);
        assert_eq!(nni.eval(10.0), 100.0);
        assert_eq!(nni.eval(14.0), 100.0);
        assert_eq!(nni.eval(15.0), 100.0);
        assert_eq!(nni.eval(15.1), 200.0);
        assert_eq!(nni.eval(16.0), 200.0);
        assert_eq!(nni.eval(21.0), 200.0);

        assert_eq!(nni.exceeds_domain(1.0), false);
        assert_eq!(nni.exceeds_domain(15.0), false);
        assert_eq!(nni.exceeds_domain(21.0), true);
    }

    #[test]
    fn linear() {
        use super::LinearInterpolator;
        let li = LinearInterpolator::new((10.0, 20.0), (100.0, 200.0));
        assert_eq!(li.eval(9.0), 100.0);
        assert_eq!(li.eval(10.0), 100.0);
        assert_eq!(li.eval(12.5), 125.0);
        assert_eq!(li.eval(15.0), 150.0);
        assert_eq!(li.eval(20.0), 200.0);
        assert_eq!(li.eval(21.0), 200.0);

        assert_eq!(li.exceeds_domain(1.0), false);
        assert_eq!(li.exceeds_domain(15.0), false);
        assert_eq!(li.exceeds_domain(21.0), true);
    }

    #[test]
    fn sigmoid() {
        use super::SigmoidInterpolator;
        let si = SigmoidInterpolator::new((10.0, 18.0), (100.0, 200.0));
        assert_eq!(si.eval(9.0), 100.0);
        assert_eq!(si.eval(10.0), 100.0);
        assert_eq!(si.eval(11.0), 104.74258731);
        assert_eq!(si.eval(12.0), 111.9202922);
        assert_eq!(si.eval(14.0), 150.0);
        assert_eq!(si.eval(15.0), 173.105857863);
        assert_eq!(si.eval(18.0), 200.0);
        assert_eq!(si.eval(19.0), 200.0);

        assert_eq!(si.exceeds_domain(1.0), false);
        assert_eq!(si.exceeds_domain(15.0), false);
        assert_eq!(si.exceeds_domain(21.0), true);
    }
}
