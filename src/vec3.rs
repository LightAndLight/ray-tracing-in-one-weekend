use rand::{distributions::uniform::SampleRange, Rng};

use crate::axis::Axis3;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Euclidian norm squared.
    pub fn norm_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Euclidian norm.
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Dot product.
    pub fn dot(&self, v: Vec3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    /// Cross product.
    pub fn cross(&self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    /// Returns a vector in the same direction, but with length 1.
    pub fn unit(&self) -> Vec3 {
        let norm = self.norm();
        debug_assert!(norm != 0.0, "divide by zero");

        *self / norm
    }

    /// Generate a random vector with all components in the specified range.
    pub fn gen_range<R: Rng, S: SampleRange<f64> + Clone>(rng: &mut R, range: S) -> Self {
        Vec3 {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    /// Returns `true` if the vector is near zero in all dimensions.
    pub fn near_zero(&self) -> bool {
        const TOLERANCE: f64 = 1e-8;
        self.x.abs() < TOLERANCE && self.y.abs() < TOLERANCE && self.z.abs() < TOLERANCE
    }

    /// Negate the vector.
    pub fn negate(&self) -> Self {
        -1.0 * *self
    }

    /// Reflect the vector around a surface `normal.`
    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - 2.0 * self.dot(*normal) * *normal
    }

    /**
    Refract the vector, if possible.

    Returns `None` when no solution exists for Snell's law (a ray travelling from a high refractive
    index to a low refractive index).

    # Arguments

    * `normal` - the surface normal (points toward the 'outside')
    * `eta_from` - the refractive index of the outer substance
    * `eta_to` - the refractive index of the inner substance
    */
    pub fn refract(&self, normal: &Vec3, eta_from: f64, eta_to: f64) -> Option<Vec3> {
        assert!(
            (1.0 - self.norm()).abs() < 0.001,
            "expected self to have a norm of 1.0, got {}",
            self.norm()
        );
        assert!(
            (1.0 - normal.norm()).abs() < 0.001,
            "expected normal to have a norm of 1.0, got {}",
            normal.norm()
        );

        // theta is the angle between the `self` (the incoming ray) and the surface normal
        let cos_theta: f64 = self.negate().dot(*normal);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let refraction_ratio = eta_from / eta_to;
        if refraction_ratio * sin_theta > 1.0 {
            None
        } else {
            // The component of the refracted ray that is parallel to the surface
            let refracted_parallel: Vec3 = refraction_ratio * (*self + cos_theta * *normal);
            // The component of the refracted ray that is perpendicular to the surface
            let refracted_perpendicular: Vec3 =
                -(1.0 - refracted_parallel.norm_squared()).sqrt() * *normal;
            Some(refracted_parallel + refracted_perpendicular)
        }
    }

    /// `(0, 0, 0)`
    pub fn origin() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn contains_nan(&self) -> bool {
        [Axis3::X, Axis3::Y, Axis3::Z]
            .into_iter()
            .any(|axis| self[axis].is_nan())
    }
}

/// Pointwise addition.
impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

/// Pointwise subtraction.
impl std::ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

/// Left-scalar multiplication.
impl std::ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

/// Left-scalar multiplication.
impl std::ops::Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

/// Right-scalar multiplication.
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

/// Pointwise negation.
impl std::ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        -1.0 * self
    }
}

/// Scalar division.
impl std::ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        1.0 / rhs * self
    }
}

impl rand::distributions::Distribution<Vec3> for rand::distributions::Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3 {
            x: rng.sample(self),
            y: rng.sample(self),
            z: rng.sample(self),
        }
    }
}

impl std::ops::Index<Axis3> for Vec3 {
    type Output = f64;

    fn index(&self, index: Axis3) -> &Self::Output {
        match index {
            Axis3::X => &self.x,
            Axis3::Y => &self.y,
            Axis3::Z => &self.z,
        }
    }
}
