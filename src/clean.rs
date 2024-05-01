use std::collections::HashSet;


use xfbin::nucc_chunk::nucc_chunk_anm::{Curve, CurveHeader, AnmCurveFormat};
use xfbin::nucc_chunk::nucc_helper::*;
use xfbin::nucc::NuccAnm;

pub fn clean_anm(nucc_anm: &mut NuccAnm) {
    for entry in nucc_anm.entries.iter_mut() {
        for (curve, curve_header) in entry.curves.iter_mut().zip(&mut entry.curve_headers) {
            clean_curve(curve, curve_header);
        }
    }
}

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

        Curve::QuaternionShort(keyframes) => {
            let values: Vec<QuaternionShort> = keyframes.iter_mut().map(|keyframe| keyframe.clone()).collect();
            let set: HashSet<QuaternionShort> = values.into_iter().collect();

            if set.len() == 1 {
                let first = keyframes.first().unwrap().clone();
                let clean_curve = Curve::QuaternionShort(vec![first.clone()]);
                *curve = clean_curve;
                curve_header.curve_format = AnmCurveFormat::SHORT4 as u16
            }
        }

        Curve::RGB(values) => {
            let rgb_values: Vec<RGB> = values.iter_mut().map(|v| v.clone()).collect();
            let set: HashSet<RGB> = rgb_values.into_iter().collect();

            if set.len() == 1 {
                let first = values.first().unwrap().clone();
                let clean_curve = Curve::RGB(vec![first.clone()]);
                *curve = clean_curve;
                curve_header.curve_format = AnmCurveFormat::BYTE3 as u16
            }
        }
        _ => {}
    }
}

