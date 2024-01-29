use std::{f32::consts::PI, mem::swap, str::from_utf8_unchecked, thread::sleep, time::Duration};

use donut::{Matrix, Vec2, Vec3};
use itertools::Itertools;
#[path = "lib.rs"]
mod lib;

const LIGHT: [u8; 17] = [
    b' ', b'.', b':', b'!', b'/', b'r', b'(', b'l', b'1', b'Z', b'4', b'H', b'9', b'W', b'8', b'$',
    b'@',
];

fn sdf_donut(p: Vec3) -> f32 {
    const T: Vec2 = Vec2 { x: 1., y: 0.5 };
    Vec2 {
        x: Vec2 { x: p.x, y: p.y }.len() - T.x,
        y: p.z,
    }
    .len()
        - T.y
}

fn sdf_icosa(p: Vec3) -> f32 {
    const S: f32 = 2.;
    let g = 5.0_f32.sqrt() / 2. + 0.5;
    let n = Vec3 { x: g, y: 1., z: 0. };
    let p = Vec3 {
        x: p.x.abs(),
        y: p.y.abs(),
        z: p.z.abs(),
    };
    0.0_f32
        .max(p.dot(&n))
        .max(p.dot(&Vec3 {
            x: n.y,
            y: n.z,
            z: n.x,
        }))
        .max(
            p.dot(
                &Vec3 {
                    x: 1.,
                    y: 1.,
                    z: 1.,
                }
                .normalized(),
            ),
        )
        - S
}

fn sdf_dodeca(p: Vec3) -> f32 {
    const S: f32 = 2.;
    let g = 5.0_f32.sqrt() / 2. + 0.5;
    let n = Vec3 { x: g, y: 1., z: 0. };
    let p = Vec3 {
        x: p.x.abs(),
        y: p.y.abs(),
        z: p.z.abs(),
    };
    0.0_f32
        .max(p.dot(&n))
        .max(p.dot(&Vec3 {
            x: n.y,
            y: n.z,
            z: n.x,
        }))
        .max(p.dot(&Vec3 {
            x: n.z,
            y: n.x,
            z: n.y,
        }))
        - S
}

fn march(ro: Vec3, rd: Vec3) -> f32 {
    let (sdf, mult) = (sdf_icosa, 1.);
    let (sdf, mult) = (sdf_dodeca, 1.);
    let (sdf, mult) = (|p| sdf_donut(p) + (rand::random::<f32>() - 0.5) / 7., 1.);
    let (sdf, mult) = (sdf_donut, 1.);

    let mut dist = 1.;
    let mut p = ro + rd;
    let mut delta = sdf(p);
    dist += delta;
    let mut p_next = Vec3 {
        x: p.x + delta * rd.x,
        y: p.y + delta * rd.y,
        z: p.z + delta * rd.z,
    };
    while dist < 100. && (p - p_next).len() > 0.005 {
        swap(&mut p, &mut p_next);
        delta = sdf(p);
        dist += delta;
        p_next = Vec3 {
            x: p.x + delta * rd.x,
            y: p.y + delta * rd.y,
            z: p.z + delta * rd.z,
        };
    }

    dist * mult
}

fn main() {
    let (width, height) = termion::terminal_size().unwrap();
    let mut frame = vec![b' '; (width * height) as usize];

    let aspect = f32::from(width) / f32::from(height);
    let pixel_aspect = 11. / 24.;

    for t in (0..9 * 7 * 100).cycle() {
        for (x, y) in (0..width).cartesian_product(0..height) {
            let uv = Vec2 {
                x: (aspect * pixel_aspect) * ((2. * x as f32) / (width as f32) - 1.),
                y: (2. * y as f32) / (height as f32) - 1.,
            };
            const SLOW: f32 = 8.;
            let a = (t as f32 / (9. * SLOW)) * PI;
            let b = (t as f32 / (7. * SLOW)) * PI;
            let (cos_a, sin_a, cos_b, sin_b) = (a.cos(), a.sin(), b.cos(), b.sin());

            let ro = Vec3 {
                x: -2.5,
                y: 0.,
                z: 0.,
            }
            .by_matrix(&Matrix {
                a: [[1., 0., 0.], [0., cos_a, -sin_a], [0., sin_a, cos_a]],
            })
            .by_matrix(&Matrix {
                a: [[cos_b, 0., -sin_b], [0., 1., 0.], [sin_b, 0., cos_b]],
            });

            let rd = Vec3 {
                x: 1.,
                y: uv.x,
                z: uv.y,
            }
            .normalized()
            .by_matrix(&Matrix {
                a: [[1., 0., 0.], [0., cos_a, -sin_a], [0., sin_a, cos_a]],
            })
            .by_matrix(&Matrix {
                a: [[cos_b, 0., -sin_b], [0., 1., 0.], [sin_b, 0., cos_b]],
            });

            let distance = march(ro, rd);
            let color = if distance < 100. {
                (distance).exp() as usize
            } else {
                LIGHT.len() - 1
            }
            .clamp(0, LIGHT.len() - 1);
            frame[(x + width * y) as usize] = LIGHT[LIGHT.len() - color - 1];
        }
        print!("\x1b[H{}", unsafe { from_utf8_unchecked(frame.as_slice()) });
        sleep(Duration::from_secs_f32(0.02));
    }
}
