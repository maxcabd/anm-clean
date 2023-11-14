use binrw::binrw;
use std::hash::Hash;



#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Eq for Vector3 {}

impl Hash for Vector3 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        // Hash each field individually
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
    }
}


#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Eq for Vector4 {}

impl Hash for Vector4 {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        // Hash each field individually
        self.x.to_bits().hash(state);
        self.y.to_bits().hash(state);
        self.z.to_bits().hash(state);
        self.w.to_bits().hash(state);
    }
}

#[binrw]
#[derive(Debug, Clone, PartialEq)]
pub struct Vector3Short {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Eq for Vector3Short {}

impl Hash for Vector3Short {
    fn hash<H>(&self, state: &mut H)
    where
        H: std::hash::Hasher,
    {
        // Hash each field individually
        self.x.to_be().hash(state);
        self.y.to_be().hash(state);
        self.z.to_be().hash(state);
    }
}

#[binrw]
#[derive(Debug, Clone)]
pub struct KeyframeVector3 {
    pub frame: i32,
    pub value: Vector3,
}


#[binrw]
#[derive(Debug, Clone)]
pub struct KeyframeVector4 {
    pub frame: i32,
    pub value: Vector4,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct KeyframeFloat {
    pub frame: i32,
    pub value: f32,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct QuaternionShort {
    pub x: i16,
    pub y: i16,
    pub z: i16,
    pub w: i16,
}

#[binrw]
#[derive(Debug, Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}