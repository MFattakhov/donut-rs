use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
/* horizontal 3-vec */
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

pub trait Vec3Like {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

impl Vec3Like for Vec3 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn z(&self) -> f32 {
        self.z
    }
}

impl Vec3Like for [f32; 3] {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }

    fn z(&self) -> f32 {
        self[2]
    }
}

impl Vec3 {
    pub fn dot<P: Vec3Like>(&self, other: &P) -> f32 {
        self.x * other.x() + self.y * other.y() + self.z * other.z()
    }

    pub fn by_matrix(&self, mat: &Matrix<3>) -> Vec3 {
        Vec3 {
            x: self.dot(&mat.a[0]),
            y: self.dot(&mat.a[1]),
            z: self.dot(&mat.a[2]),
        }
    }

    pub fn by_matrix_mut(&mut self, mat: &Matrix<3>) {
        *self = self.by_matrix(mat)
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Vec3 {
        let len = self.len();
        Vec3 {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

#[derive(Debug, Clone, Copy)]
/* horizontal 2-vec */
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

pub trait Vec2Like {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl Vec2Like for Vec2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }
}

impl Vec2Like for [f32; 2] {
    fn x(&self) -> f32 {
        self[0]
    }

    fn y(&self) -> f32 {
        self[1]
    }
}

impl Vec2 {
    pub fn dot<P: Vec2Like>(&self, other: &P) -> f32 {
        self.x * other.x() + self.y * other.y()
    }

    pub fn by_matrix(&self, mat: &Matrix<2>) -> Vec2 {
        Vec2 {
            x: self.dot(&mat.a[0]),
            y: self.dot(&mat.a[1]),
        }
    }

    pub fn by_matrix_mut(&mut self, mat: &Matrix<2>) {
        *self = self.by_matrix(mat)
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Vec2 {
        let len = self.len();
        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }
}

pub struct Matrix<const N: usize> {
    /* array of columns */
    pub a: [[f32; N]; N],
}
