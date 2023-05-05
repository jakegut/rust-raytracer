pub mod vec3 {
    use std::ops::{AddAssign, DivAssign, MulAssign, Neg};

    pub struct Vec3 {
        e: [f32; 3],
    }

    impl Vec3 {
        pub fn new_empty() -> Vec3 {
            Vec3 { e: [0.0; 3] }
        }

        pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
            Vec3 { e: [x, y, z] }
        }
    }

    impl AddAssign for Vec3 {
        fn add_assign(&mut self, rhs: Self) {
            self.e[0] += rhs.e[0];
            self.e[1] += rhs.e[1];
            self.e[2] += rhs.e[2];
        }
    }

    impl MulAssign<f32> for Vec3 {
        fn mul_assign(&mut self, rhs: f32) {
            self.e[0] *= rhs;
            self.e[1] *= rhs;
            self.e[2] *= rhs;
        }
    }

    impl DivAssign<f32> for Vec3 {
        fn div_assign(&mut self, rhs: f32) {
            self.mul_assign(1.0 / rhs);
        }
    }

    impl Neg for Vec3 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Vec3::new(-self.e[0], -self.e[1], -self.e[2])
        }
    }

    #[cfg(test)]
    mod tests {
        use super::Vec3;

        #[test]
        fn new_test() {
            let v = Vec3::new(1.0, 1.0, 1.0);
            assert_eq!(v.e, [1.0, 1.0, 1.0]);
        }

        #[test]
        fn add_assign() {
            let mut v = Vec3::new(1.0, 2.0, 3.0);
            v += Vec3::new(3.0, 2.0, 1.0);
            assert_eq!(v.e, [4.0, 4.0, 4.0]);
        }

        #[test]
        fn div_assign() {
            let mut v = Vec3::new(3.0, 2.0, 1.0);
            v /= 2.0;
            assert_eq!(v.e, [1.5, 1.0, 0.5]);
        }

        #[test]
        fn mul_assign() {
            let mut v = Vec3::new(2.0, 4.0, 8.0);
            v *= 2.0;
            assert_eq!(v.e, [4.0, 8.0, 16.0]);
        }

        #[test]
        fn neg() {
            let v = Vec3::new(2.0, -3.0, 5.0);
            let v2 = -v;
            assert_eq!(v2.e, [-2.0, 3.0, -5.0]);
        }
    }
}
