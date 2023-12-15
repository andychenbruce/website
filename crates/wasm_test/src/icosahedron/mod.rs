use crate::normalize;
use crate::PHI;
use cgmath::SquareMatrix;
pub fn generate_verticies() -> Vec<[cgmath::Vector4<f32>; 3]> {
    let rotated_1 = cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 2.0),
    ) * cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 4.0),
    );
    let rotated_2 = cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 2.0),
    ) * cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        }),
        cgmath::Rad(std::f32::consts::TAU / 4.0),
    );
    let rotate_idk = cgmath::Matrix4::from_axis_angle(
        normalize(cgmath::Vector3 {
            x: 0.0,
            y: 1.0,
            z: PHI,
        }),
        cgmath::Rad(std::f32::consts::TAU / 5.0),
    );

    let first = cgmath::Matrix4::identity();
    let second = cgmath::Matrix4::from_nonuniform_scale(1.0, 1.0, -1.0);
    let normal = cgmath::Matrix4::identity();
    let flipped = cgmath::Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);

    let mut verts: Vec<[cgmath::Vector4<f32>; 3]> = vec![];
    let base_verts: [cgmath::Vector4<f32>; 3] = [
        [-1.0, PHI, 0.0, 1.0].into(),
        [1.0, PHI, 0.0, 1.0].into(),
        [0.0, 1.0, PHI, 1.0].into(),
    ];
    for s in [cgmath::Matrix4::identity(), rotated_1, rotated_2].iter() {
        for i in [normal, flipped] {
            for o in [first, second] {
                let matrix = s * o * i;
                verts.push(base_verts.map(|x| matrix * x));
            }
        }
    }

    let base = cgmath::Matrix4::identity();
    let flipped_x = cgmath::Matrix4::from_nonuniform_scale(-1.0, 1.0, 1.0);
    let flipped_y = cgmath::Matrix4::from_nonuniform_scale(1.0, -1.0, 1.0);
    let flipped_z = cgmath::Matrix4::from_nonuniform_scale(1.0, 1.0, -1.0);

    for x in [false, true] {
        for y in [false, true] {
            for z in [false, true] {
                let thing = (if x { base } else { flipped_x })
                    * (if y { base } else { flipped_y })
                    * (if z { base } else { flipped_z });

                let matrix = thing * rotate_idk;
                verts.push(base_verts.map(|x| matrix * x));
            }
        }
    }

    assert!(verts.len() == 20);

    verts
}