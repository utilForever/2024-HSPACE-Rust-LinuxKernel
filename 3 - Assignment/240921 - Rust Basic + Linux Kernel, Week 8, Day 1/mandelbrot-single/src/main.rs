use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use num::Complex;

use std::env;
use std::fs::File;
use std::str::FromStr;

/// Try to determine if `c` is in the Mandelbrot set,
/// using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered
/// on the origin. If `c` seems to be a member (more precisely,
/// if we reached the iteration limit without being able to prove that
/// `c` is not a member), return `None`.
fn escape_time(_c: Complex<f64>, _limit: usize) -> Option<usize> {
    todo!()
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`.
/// If it doesn't parse correctly, return `None`.
fn parse_pair<T: FromStr>(_s: &str, _separator: char) -> Option<(T, T)> {
    todo!()
}

/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(_s: &str) -> Option<Complex<f64>> {
    todo!()
}

/// Given the row and column of a pixel in the output image,
/// return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
    _bounds: (usize, usize),
    _pixel: (usize, usize),
    _upper_left: Complex<f64>,
    _lower_right: Complex<f64>,
) -> Complex<f64> {
    todo!()
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
fn render(
    _pixels: &mut [u8],
    _bounds: (usize, usize),
    _upper_left: Complex<f64>,
    _lower_right: Complex<f64>,
) {
    todo!()
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
fn write_image(_filename: &str, _pixels: &[u8], _bounds: (usize, usize)) {
    todo!()
}

fn main() {
    let args = env::args().collect::<Vec<_>>();

    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPER_LEFT LOWER_RIGHT", args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.00,0.20",
            args[0]
        );

        std::process::exit(-1);
    }

    let bounds = parse_pair(&args[2], 'x').expect("Error parsing image dimensions");
    let upper_left = parse_complex(&args[3]).expect("Error parsing upper left corner point");
    let lower_right = parse_complex(&args[4]).expect("Error parsing lower right corner point");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    render(&mut pixels, bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds);
}
