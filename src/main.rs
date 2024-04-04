#![allow(non_snake_case)]

use std::{thread::sleep, time::Duration};

fn rotate(multiplier: i32, shift: i32, x: &mut i32, y: &mut i32) {
    let mut temp: i32 = *x;
    *x -= multiplier * *y >> shift;
    *y += multiplier * temp >> shift;
    temp = 3145728 - *x * *x - *y * *y >> 11;
    *x = *x * temp >> 10;
    *y = *y * temp >> 10;
}

const BUFFER_SIZE: usize = 1760;
const LUMINANCE_CHARS: [char; 12] = ['.', ',', '-', '~', ':', ';', '=', '!', '*', '#', '$', '@'];

fn main() {
    let mut buffer: [char; BUFFER_SIZE];
    let mut z_buffer: [i8; BUFFER_SIZE];

    let mut sin_A: i32 = 1024;
    let mut cos_A: i32 = 0;
    let mut sin_B: i32 = 1024;
    let mut cos_B: i32 = 0;

    loop {
        buffer = [' '; BUFFER_SIZE];
        z_buffer = [i8::MAX; BUFFER_SIZE];

        let mut sin_j: i32 = 0;
        let mut cos_j: i32 = 1024;

        for _j in 0..90 {
            let mut sin_i: i32 = 0;
            let mut cos_i: i32 = 1024;

            for _i in 0..324 {
                let minor_radius_r1: i32 = 1;
                let major_radius_r2: i32 = 2048;
                let distance_constant_k2: i32 = 5120 * 1024;

                let x0 = minor_radius_r1 * cos_j + major_radius_r2;
                let x1 = cos_i * x0 >> 10;
                let x2 = cos_A * sin_j >> 10;
                let x3 = sin_i * x0 >> 10;
                let x4 = minor_radius_r1 * x2 - (sin_A * x3 >> 10);
                let x5 = sin_A * sin_j >> 10;
                let x6 = distance_constant_k2 + minor_radius_r1 * 1024 * x5 + cos_A * x3;
                let x7 = cos_j * sin_i >> 10;

                let x: i32 = 40 + 30 * (cos_B * x1 - sin_B * x4) / x6;
                let y: i32 = 12 + 15 * (cos_B * x4 + sin_A * x1) / x6;

                let luminance_index: i32 = (-1 * cos_A * x7
                    - cos_B * ((-1 * sin_A * x7 >> 10) + x2)
                    - cos_i * (cos_j * sin_B >> 10)
                    >> 10)
                    - x5
                    >> 7;
                let luminance_index: usize = usize::try_from(luminance_index).unwrap_or(0);

                let o: usize =
                    (x as usize) + ((y as isize).wrapping_mul(80) as usize) % BUFFER_SIZE;

                let zz: i8 = i8::try_from((x6 - distance_constant_k2) >> 15)
                    .expect("Couldn't convert zz to i8!");

                if 22 > y && y > 0 && x > 0 && 80 > x && zz < z_buffer[o] {
                    z_buffer[o] = zz;
                    buffer[o] = LUMINANCE_CHARS[luminance_index];
                }
                rotate(5, 8, &mut cos_i, &mut sin_i);
            }
            rotate(9, 7, &mut cos_j, &mut sin_j);
        }

        for k in 0..1761 {
            match k % 80 {
                0 => print!("\n"),
                1.. => print!("{}", buffer[k]),
            }
        }

        rotate(5, 7, &mut cos_A, &mut sin_A);
        rotate(5, 8, &mut cos_B, &mut sin_B);

        sleep(Duration::from_millis(35));
        print!("\x1b[23A");
    }
}
