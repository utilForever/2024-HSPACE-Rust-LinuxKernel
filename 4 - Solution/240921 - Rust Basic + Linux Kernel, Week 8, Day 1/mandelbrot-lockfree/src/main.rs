use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder};
use num::Complex;

use std::env;
use std::fs::File;
use std::str::FromStr;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::*;

/// Try to determine if `c` is in the Mandelbrot set,
/// using at most `limit` iterations to decide.
///
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius two centered
/// on the origin. If `c` seems to be a member (more precisely,
/// if we reached the iteration limit without being able to prove that
/// `c` is not a member), return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }

        z = z * z + c;
    }

    None
}

/// Parse the string `s` as a coordinate pair, like `"400x600"` or `"1.0,0.5"`.
///
/// Specifically, `s` should have the form <left><sep><right>, where <sep> is
/// the character given by the `separator` argument, and <left> and <right> are
/// both strings that can be parsed by `T::from_str`.
///
/// If `s` has the proper form, return `Some<(x, y)>`.
/// If it doesn't parse correctly, return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(left), Ok(right)) => Some((left, right)),
            _ => None,
        },
        None => None,
    }
}

/// Parse a pair of floating-point numbers separated by a comma as a complex number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex { re, im })
}

/// Given the row and column of a pixel in the output image,
/// return the corresponding point on the complex plane.
///
/// `bounds` is a pair giving the width and height of the image in pixels.
/// `pixel` is a (column, row) pair indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex plane
/// designating the area our image covers.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );

    // Why subtraction here? pixel.1 increases as we go down,
    // but the imaginary component increases as we go up.
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    }
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
///
/// The `bounds` argument gives the width and height of the buffer `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert_eq!(pixels.len(), bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);

            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                Some(count) => 255 - count as u8,
                None => 0,
            }
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`,
/// to the file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) {
    let output = File::create(filename).expect("Error creating PNG file");
    let encoder = PngEncoder::new(output);

    encoder
        .write_image(
            pixels,
            bounds.0 as u32,
            bounds.1 as u32,
            ExtendedColorType::L8,
        )
        .expect("Error writing PNG file");
}

pub struct AtomicChunksMut<'a, T> {
    slice: &'a [T],
    step: usize,
    next: AtomicUsize,
}

impl<'a, T> AtomicChunksMut<'a, T> {
    pub fn new(slice: &'a mut [T], step: usize) -> AtomicChunksMut<'a, T> {
        AtomicChunksMut {
            slice,
            step,
            next: AtomicUsize::new(0),
        }
    }

    #[allow(mutable_transmutes)]
    unsafe fn next(&self) -> Option<(usize, &'a mut [T])> {
        loop {
            let current = self.next.load(SeqCst);

            assert!(current <= self.slice.len());

            if current == self.slice.len() {
                return None;
            }

            let end = std::cmp::min(current + self.step, self.slice.len());

            if self
                .next
                .compare_exchange(current, end, SeqCst, SeqCst)
                .is_ok()
            {
                return Some((
                    current / self.step,
                    std::mem::transmute(&self.slice[current..end]),
                ));
            }
        }
    }
}

impl<'a, 'b, T> Iterator for &'b AtomicChunksMut<'a, T> {
    type Item = (usize, &'a mut [T]);
    fn next(&mut self) -> Option<Self::Item> {
        unsafe { (*self).next() }
    }
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

    let threads = 8;
    let rows_per_band = bounds.1 / 400 + 1;

    {
        let bands = AtomicChunksMut::new(&mut pixels, rows_per_band * bounds.0);

        crossbeam::scope(|scope| {
            for _ in 0..threads {
                scope.spawn(|_| {
                    for (i, band) in &bands {
                        let top = i * rows_per_band;
                        let height = band.len() / bounds.0;
                        let band_bounds = (bounds.0, height);
                        let band_upper_left =
                            pixel_to_point(bounds, (0, top), upper_left, lower_right);
                        let band_lower_right = pixel_to_point(
                            bounds,
                            (bounds.0, top + height),
                            upper_left,
                            lower_right,
                        );

                        render(band, band_bounds, band_upper_left, band_lower_right);
                    }
                });
            }
        })
        .expect("Error rendering image");
    }

    write_image(&args[1], &pixels, bounds);
}
