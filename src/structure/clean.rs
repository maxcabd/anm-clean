use std::collections::HashSet;
use std::mem;

use crate::structure::anm::{AnmCurveFormat, Curve, CurveHeader};
use crate::utils::util::*;

pub fn clean_curve(curve: &mut Curve, curve_header: &mut CurveHeader) {
    match curve {
        Curve::KeyframeVector3(keyframes) => {
            let values: Vec<Vector3> = keyframes.iter_mut().map(|keyframe| keyframe.value.clone()).collect();
            let set: HashSet<Vector3> = values.into_iter().collect();

            if set.len() == 1 {
                let first = keyframes.first().unwrap().clone();
                let clean_curve = Curve::Vector3(vec![first.value.clone()]);
                *curve = clean_curve;
                curve_header.curve_format = AnmCurveFormat::FLOAT3 as u16
            }
        }

        Curve::KeyframeVector4(keyframes) => {
            let values: Vec<Vector4> = keyframes.iter_mut().map(|keyframe| keyframe.value.clone()).collect();
            let set: HashSet<Vector4> = values.into_iter().collect();

            if set.len() == 1 {
                let first = keyframes.first().unwrap().clone();
                keyframes.clear();
                keyframes.push(first);
            }

            curve.append_null_keyframe();
            curve_header.frame_count += 1;
        }

        Curve::KeyframeFloat(keyframes) => {
            let values: Vec<f32> = keyframes.iter_mut().map(|keyframe| keyframe.value.clone()).collect();
            let set: HashSet<u64> = values.into_iter().map(|x| x as u64).collect();

            if set.len() == 1 {
                let first = keyframes.first().unwrap().clone();
                let clean_curve = Curve::Float(vec![first.value.clone()]);
                *curve = clean_curve;
                curve_header.curve_format = AnmCurveFormat::FLOAT1 as u16
            }
        }
        _ => {}
    }
}

pub fn update_header(curve: &Curve, curve_header: &mut CurveHeader) {
    curve_header.curve_size += mem::size_of_val(&curve) as u16;
    curve_header.frame_count = curve.get_frame_count() as u16;
}