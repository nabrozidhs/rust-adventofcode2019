use std::ops::{AddAssign, Add, Sub};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector2(pub i64, pub i64);

impl Vector2 {
    pub const ZERO: Vector2 = Vector2(0, 0);

    pub fn new(x: i64, y: i64) -> Vector2 { Vector2(x, y) }

    pub fn manhattan_distance(&self, other: &Vector2) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
        }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

#[cfg(test)]
mod tests_util_vector2 {
    use crate::util::Vector2;

    #[test]
    fn manhattan_distance() {
        assert_eq!(2, Vector2(1, 1).manhattan_distance(&Vector2(2, 2)));
        assert_eq!(4, Vector2(0, 0).manhattan_distance(&Vector2(2, 2)));
        assert_eq!(6, Vector2(-1, -1).manhattan_distance(&Vector2(2, 2)));
    }

    #[test]
    fn add() {
        assert_eq!(Vector2(-1, -1), Vector2(-1, -1) + Vector2(0, 0));
        assert_eq!(Vector2(-1, 0), Vector2(-1, -1) + Vector2(0, 1));
        assert_eq!(Vector2(0, 1), Vector2(-1, 0) + Vector2(1, 1));
        assert_eq!(Vector2(-2, -1), Vector2(0, 1) + Vector2(-2, -2));
    }

    #[test]
    fn add_assign() {
        let mut v = Vector2(-1, -1);
        v += Vector2(0, 0);
        assert_eq!(Vector2(-1, -1), v);
        v += Vector2(0, 1);
        assert_eq!(Vector2(-1, 0), v);
        v += Vector2(1, 1);
        assert_eq!(Vector2(0, 1), v);
        v += Vector2(-2, -2);
        assert_eq!(Vector2(-2, -1), v);
    }

    #[test]
    fn sub() {
        assert_eq!(Vector2(-1, -1), Vector2(-1, -1) - Vector2(0, 0));
        assert_eq!(Vector2(-1, -2), Vector2(-1, -1) - Vector2(0, 1));
        assert_eq!(Vector2(-2, -1), Vector2(-1, 0) - Vector2(1, 1));
        assert_eq!(Vector2(2, 3), Vector2(0, 1) - Vector2(-2, -2));
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Vector3(pub i64, pub i64, pub i64);

impl Vector3 {
    pub const ZERO: Vector3 = Vector3(0, 0, 0);

    pub fn new(x: i64, y: i64, z: i64) -> Vector3 { Vector3(x, y, z) }

    pub fn manhattan_distance(&self, other: &Vector3) -> i64 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }
}

impl Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            0: self.0 + other.0,
            1: self.1 + other.1,
            2: self.2 + other.2,
        }
    }
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            0: self.0 - other.0,
            1: self.1 - other.1,
            2: self.2 - other.2,
        }
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

#[cfg(test)]
mod tests_util_vector3 {
    use crate::util::Vector3;

    #[test]
    fn manhattan_distance() {
        assert_eq!(3, Vector3(1, 1, 1).manhattan_distance(&Vector3(2, 2, 2)));
        assert_eq!(6, Vector3(0, 0, 0).manhattan_distance(&Vector3(2, 2, 2)));
        assert_eq!(9, Vector3(-1, -1, -1).manhattan_distance(&Vector3(2, 2, 2)));
    }

    #[test]
    fn add() {
        assert_eq!(Vector3(-1, -1, -1), Vector3(-1, -1, -1) + Vector3(0, 0, 0));
        assert_eq!(Vector3(-1, 0, 0), Vector3(-1, -1, -1) + Vector3(0, 1, 1));
        assert_eq!(Vector3(0, 1, 2), Vector3(-1, 0, 1) + Vector3(1, 1, 1));
        assert_eq!(Vector3(-2, -1, -1), Vector3(0, 1, 0) + Vector3(-2, -2, -1));
    }

    #[test]
    fn add_assign() {
        let mut v = Vector3(-1, -1, -1);
        v += Vector3(0, 0, 0);
        assert_eq!(Vector3(-1, -1, -1), v);
        v += Vector3(0, 1, 1);
        assert_eq!(Vector3(-1, 0, 0), v);
        v += Vector3(1, 1, -1);
        assert_eq!(Vector3(0, 1, -1), v);
        v += Vector3(-2, -2, 2);
        assert_eq!(Vector3(-2, -1, 1), v);
    }

    #[test]
    fn sub() {
        assert_eq!(Vector3(-1, -1, -1), Vector3(-1, -1, -1) - Vector3(0, 0, 0));
        assert_eq!(Vector3(-1, -2, -2), Vector3(-1, -1, -1) - Vector3(0, 1, 1));
        assert_eq!(Vector3(-2, -1, 0), Vector3(-1, 0, 1) - Vector3(1, 1, 1));
        assert_eq!(Vector3(2, 3, 1), Vector3(0, 1, 0) - Vector3(-2, -2, -1));
    }
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b.abs();
    } else if b == 0 {
        return a.abs();
    }

    a = a.abs();
    b = b.abs();
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests_util {
    use crate::util::{gcd, lcm};

    #[test]
    fn util_gcd() {
        assert_eq!(1071, gcd(1071, 0));
        assert_eq!(462, gcd(0, 462));

        assert_eq!(1071, gcd(-1071, 0));
        assert_eq!(462, gcd(0, -462));

        assert_eq!(21, gcd(1071, 462));
        assert_eq!(1, gcd(1, 462));
        assert_eq!(1, gcd(-1, 462));
    }

    #[test]
    fn util_lcm() {
        assert_eq!(2, lcm(2, 1));
        assert_eq!(3, lcm(1, 3));
        assert_eq!(10, lcm(5, 2));
        assert_eq!(12, lcm(4, 6));
    }
}
