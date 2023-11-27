use std::collections::HashSet;
use std::mem;

use crate::structure::anm::{AnmCurveFormat, Curve, CurveHeader};
use crate::utils::util::*;

pub fn clean_curve(curve: &mut Curve, curve_header: &mut CurveHeader) {
    match curve {
        Curve::KeyframeVector3(keyframes) => {
            let values: Vec<Vector3> = keyframes.iter_mut().map(|keyframe| keyframe.value.clone()).collect();

            // We'll need to use a set to check if the keyframes are filled with redundant / duplicate values
            let set: HashSet<Vector3> = values.into_iter().collect();

            // If the length of the set is 1, or if all the values in the list are the same, then we only need to use the first keyframe
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

            // We could save more space and instead just convert to a Euler rotation, but Quaternions are more precise anyways
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
    // We need to update the size (not important since the engine ignores it) and the frame count to account for the new keyframes amount
    
    // TODO: Use the actual size of the struct in bytes, this method doesn't return the actual size on disk
    curve_header.curve_size += mem::size_of_val(&curve) as u16;
    curve_header.frame_count = curve.get_frame_count() as u16;
}
