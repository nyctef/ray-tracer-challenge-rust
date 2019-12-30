// random list of numbers from 0 to 255, used for the gradient hash function
static PERMUTATION: [u8; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

#[inline]
fn p(i: u8) -> u8 {
    PERMUTATION[i as usize]
}

fn fade(t: f32) -> f32 {
    // Fade function as defined by Ken Perlin.  This eases coordinate values
    // so that they will ease towards integral values.  This ends up smoothing
    // the final output.

    // 6t^5 - 15t^4 + 10t^3
    t * t * t * (t * (t * 6. - 15.) + 10.)
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}

fn grad(h: u8, x: f32, y: f32, z: f32) -> f32 {
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h == 12 || h == 14 {
        x
    } else {
        z
    };

    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}

pub fn perlin(x: f32, y: f32, z: f32) -> f32 {
    // based on https://flafla2.github.io/2014/08/09/perlinnoise.html and
    // http://riven8192.blogspot.com/2010/08/calculate-perlinnoise-twice-as-fast.html

    // we work with cubes of size 255(?)
    let xi = (x as i32 & 255) as u8;
    let yi = (y as i32 & 255) as u8;
    let zi = (z as i32 & 255) as u8;
    let xf = x.fract();
    let yf = y.fract();
    let zf = z.fract();

    let u = fade(xf);
    let v = fade(yf);
    let w = fade(zf);

    let a0 = yi + p(xi);
    let aa = zi + p(a0);
    let ab = zi + p(a0 + 1);
    let b0 = yi + p(xi + 1);
    let ba = zi + p(b0);
    let bb = zi + p(b0 + 1);

    let aa0 = p(aa);
    let aa1 = p(aa + 1);
    let ab0 = p(ab);
    let ab1 = p(ab + 1);
    let ba0 = p(ba);
    let ba1 = p(ba + 1);
    let bb0 = p(bb);
    let bb1 = p(bb + 1);

    let a1 = grad(bb1, xf - 1., yf - 1., zf - 1.);
    let a2 = grad(ab1, xf, yf - 1., zf - 1.);
    let a3 = grad(ba1, xf - 1., yf, zf - 1.);
    let a4 = grad(aa1, xf, yf, zf - 1.);
    let a5 = grad(bb0, xf - 1., yf - 1., zf);
    let a6 = grad(ab0, xf, yf - 1., zf);
    let a7 = grad(ba0, xf - 1., yf, zf);
    let a8 = grad(aa0, xf, yf, zf);

    let a8_5 = lerp(v, lerp(u, a8, a7), lerp(u, a6, a5));
    let a4_1 = lerp(v, lerp(u, a4, a3), lerp(u, a2, a1));
    return lerp(w, a8_5, a4_1);
}
