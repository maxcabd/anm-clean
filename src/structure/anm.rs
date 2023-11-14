use std::io::{Read, Seek};
use binrw::{binrw, BinRead, BinResult, ReadOptions};


use crate::utils::util::*;

#[binrw]
#[derive(Debug, Clone)]
pub struct NuccAnm {
    pub anm_length: u32,
    pub frame_size: u32,
    pub entry_count: u16,
    pub looped: u16,

    pub clump_count: u16,

    pub other_entry_count: u16,
    pub other_index_count: u16,

    pub coord_count: u16,

    #[br(count = clump_count)]
    pub clumps: Vec<AnmClump>,

    #[br(count = other_entry_count + other_index_count)]
    pub other_entries: Vec<u32>,


    #[br(count = coord_count)]
    pub coord_parents: Vec<CoordParent>,

    #[br(count = entry_count)]
    pub entries: Vec<AnmEntry>

}


#[binrw]
#[derive(Debug, Clone)]
pub struct AnmClump {
    pub clump_index: u32,
    pub bone_material_count: u16,
    pub model_count: u16,

    #[br(count = bone_material_count)]
    pub bone_material_indices: Vec<u32>,
    
    #[br(count = model_count)]
    pub model_indices: Vec<u32>,


}

#[binrw]
#[derive(Debug, Clone)]
pub struct CoordParent {
    pub parent: AnmCoord,
    pub child: AnmCoord,

}

#[binrw]
#[derive(Debug, Clone)]
pub struct AnmCoord {
    pub clump_index: i16,
    pub coord_index: u16
}

#[binrw]
#[brw(repr(u16))]
#[derive(Debug, Clone)]
pub enum AnmEntryFormat {
    BONE = 1,
    CAMERA = 2,
    MATERIAL = 4,
    LIGHTDIRC = 5,
    LIGHTPOINT = 6,
    AMBIENT = 8,

}

#[binrw]
#[derive(Debug, Clone)]
pub struct AnmEntry {
    pub coord: AnmCoord,
    pub entry_format: u16,
    pub curve_count: u16,

    #[br(count = curve_count)]
    pub curve_headers: Vec<CurveHeader>,

    #[br(parse_with = from_iterator_args(curve_headers.iter()))]
    pub curves: Vec<Curve>
}


#[binrw]
#[derive(Debug, Clone)]
pub struct CurveHeader {
    pub curve_index: u16,
    pub curve_format: u16,
    pub frame_count: u16,
    pub curve_size: u16,  
}

#[binrw]
#[brw(repr(u16))]
#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AnmCurveFormat {
    FLOAT3 = 0x05,  // location/scale
    INT1_FLOAT3 = 0x06,  // location/scale (with keyframe)
    FLOAT3ALT = 0x08,  // rotation
    INT1_FLOAT4 = 0x0A,  // rotation quaternions (with keyframe)
    FLOAT1 = 0x0B,  // "toggled"
    INT1_FLOAT1 = 0x0C,  // camera
    SHORT1 = 0x0F,  // "toggled"
    SHORT3 = 0x10,  // scale
    SHORT4 = 0x11,  // rotation quaternions
    BYTE3 = 0x14,  // lightdirc
    FLOAT3ALT2 = 0x15,  // scale
    FLOAT1ALT = 0x16,  // lightdirc
    FLOAT1ALT2 = 0x18  // material
}


#[binrw]
#[derive(Debug, Clone)]
#[br(import_raw(header: CurveHeader))]
pub enum Curve {
    #[br(pre_assert(matches!(header.curve_format, 0x05 | 0x08 | 0x15)))]
    Vector3(#[br(count = header.frame_count)] Vec<Vector3>),

    #[br(pre_assert(matches!(header.curve_format, 0x06)))]
    KeyframeVector3(#[br(count = header.frame_count)] Vec<KeyframeVector3>),

    #[br(pre_assert(matches!(header.curve_format, 0x0A)))]
    KeyframeVector4(#[br(count = header.frame_count)] Vec<KeyframeVector4>),

    #[br(pre_assert(matches!(header.curve_format, 0x0B | 0x16 | 0x18)))]
    Float(#[br(count = header.frame_count)] Vec<f32>),

    #[br(pre_assert(matches!(header.curve_format, 0x0C)))]
    KeyframeFloat(#[br(count = header.frame_count)] Vec<KeyframeFloat>),

    #[br(pre_assert(matches!(header.curve_format, 0x0F)))]
    Short(#[br(count = header.frame_count, if(header.frame_count % 2 == 1), pad_after = 2)] Vec<i16>),

    #[br(pre_assert(matches!(header.curve_format, 0x10)))]
    Vector3Short(#[br(count = header.frame_count, if(header.frame_count % 2 == 1), pad_after = 2)] Vec<Vector3Short>), // #[br(if(header.frame_count % 2 == 0), pad_after = 2)]

    #[br(pre_assert(matches!(header.curve_format, 0x11)))]
    QuaternionShort(#[br(count = header.frame_count)] Vec<QuaternionShort>),

    #[br(pre_assert(matches!(header.curve_format, 0x14)))]
    RGB(#[br(count = header.frame_count, pad_after = header.frame_count % 4)] Vec<RGB>),

    // Handle unknown curve formats
    #[br(pre_assert(false))]
    Unknown(#[br(count = header.curve_size)] Vec<u8>)
}

impl Curve {
    pub fn get_frame_count(&self) -> u16 {
        match self {
            Curve::Vector3(curve) => curve.len() as u16,
            Curve::KeyframeVector3(curve) => curve.len() as u16,
            Curve::KeyframeVector4(curve) => curve.len() as u16,
            Curve::Float(curve) => curve.len() as u16,
            Curve::KeyframeFloat(curve) => curve.len() as u16,
            Curve::Short(curve) => curve.len() as u16,
            Curve::Vector3Short(curve) => curve.len() as u16,
            Curve::QuaternionShort(curve) => curve.len() as u16,
            Curve::RGB(curve) => curve.len() as u16,
            Curve::Unknown(curve) => curve.len() as u16,
        }
    }

    pub fn has_keyframes(&self) -> bool {
        if &self.get_frame_count() > &1 {
            return true;
        }
        return false;
    } 

    pub fn get_curve_format(&self) -> u16 {
        match self {
            Curve::Vector3(_) => AnmCurveFormat::FLOAT3 as u16,
            Curve::KeyframeVector3(_) => AnmCurveFormat::INT1_FLOAT3 as u16,
            Curve::KeyframeVector4(_) => AnmCurveFormat::INT1_FLOAT4 as u16,
            Curve::Float(_) => AnmCurveFormat::FLOAT1 as u16,
            Curve::KeyframeFloat(_) => AnmCurveFormat::INT1_FLOAT1 as u16,
            Curve::Short(_) => AnmCurveFormat::SHORT1 as u16,
            Curve::Vector3Short(_) => AnmCurveFormat::SHORT3 as u16,
            Curve::QuaternionShort(_) => AnmCurveFormat::SHORT4 as u16,
            Curve::RGB(_) => AnmCurveFormat::BYTE3 as u16,
            Curve::Unknown(_) => AnmCurveFormat::FLOAT1ALT2 as u16,
        }
    }

    pub fn pad_color_values(&mut self) {
        match self {
            Curve::RGB(values) => {
                let len = values.len();
                
                if len % 4 != 0 {
                    let last_color = match values.last() {
                        Some(color) => color.clone(),
                        None => RGB { r: 255, g: 255, b: 255 }, // Provide a default color if the curve is empty
                    };

                    for _ in len % 4..4 {
                        values.push(last_color.clone());
                    }
                }
                
                
            }
            _ => {}
        }
    }

    // Method to append a null keyframe to the curve
    pub fn append_null_keyframe(&mut self) {
        match self {
            Curve::KeyframeVector3(keyframes) => {
                if let Some(last_frame) = keyframes.last().map(|keyframe| keyframe.frame) {
                    if last_frame != -1 {
                        let null_keyframe = KeyframeVector3 {
                            frame: -1,
                            value: keyframes.last().unwrap().value.clone(),
                        };
                        keyframes.push(null_keyframe);
                    }
                }
            }
            Curve::KeyframeVector4(keyframes) => {
                if let Some(last_frame) = keyframes.last().map(|keyframe| keyframe.frame) {
                    if last_frame != -1 {
                        let null_keyframe = KeyframeVector4 {
                            frame: -1,
                            value: keyframes.last().unwrap().value.clone(),
                        };
                        keyframes.push(null_keyframe);
                    }
                }
            }
            Curve::KeyframeFloat(keyframes) => {
                if let Some(last_frame) = keyframes.last().map(|keyframe| keyframe.frame) {
                    if last_frame != -1 {
                        let null_keyframe = KeyframeFloat {
                            frame: -1,
                            value: keyframes.last().unwrap().value.clone(),
                        };
                        keyframes.push(null_keyframe);
                    }
                }
            }
            _ => {} // No null keyframe for other curve variants
        }
    }
}



fn from_iterator_args<'it, R, T, Arg, Ret, It>(it: It) -> impl FnOnce(&mut R, &ReadOptions, ()) -> BinResult<Ret>
where
  T: BinRead<Args = Arg>,
  R: Read + Seek,
  Arg: Clone + 'static,
  Ret: FromIterator<T> + 'static,
  It: Iterator<Item = &'it Arg> + 'it,
{
  move |reader, options, _| {
    it.map(|arg| T::read_options(reader, options, arg.clone())).collect()
  }
}