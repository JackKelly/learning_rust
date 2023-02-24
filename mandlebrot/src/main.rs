use num::Complex;
use std::str::FromStr;
use std::env;
use image;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!(
            "Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT",
            args[0]);
        eprintln!(
            "Example: {} mandel.png 1000x750 -1.20,0.35 -1.0,0.20",
            args[0]);
        std::process::exit(1);
    }

    let bounds = parse_pair(&args[2], 'x')
        .expect("Error parsing image dimensions");
    let bounds = Bound{width: bounds.0, height: bounds.1};

    let upper_left = parse_complex(&args[3])
        .expect("Error parsing upper left corner point");

    let lower_right = parse_complex(&args[4])
        .expect("Error parsing lower right corner pair");

    let mut pixels = vec![0; bounds.width * bounds.height];

    render(&mut pixels, &bounds, upper_left, lower_right);

    write_image(&args[1], &pixels, bounds).expect("Error writing image file");

}

/// Try to determine if `c` is in the Mandelbrot set, using at most `limit`
/// iterations to decide.
/// 
/// If `c` is not a member, return `Some(i)`, where `i` is the number of
/// iterations it took for `c` to leave the circle of radius 2, centered on the
/// origin. If `c` seems to be a member (more precisely, if we reached the iteration
/// limit without being able to prove that `c` is not a member),
/// return `None`.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
let mut z = Complex {re: 0.0, im: 0.0};
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
/// both strings that can be parsed by `T::from_str`. `separator` must be an
/// ASCII character.
/// 
/// If `s` has the proper form, return `Some(x, y)`. If it doesn't parse
/// correctly then return `None`.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[0..index]), T::from_str(&s[index+1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            }
        }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair("100x200", 'x'), Some((100, 200)));
    assert_eq!(parse_pair::<i32>("100x200", 's'), None);
    assert_eq!(parse_pair::<f32>("1.0,3.4", ','), Some((1.0, 3.4)));

}

/// Parse a pair of floating-point numbers separated by a comma as a complex
/// number.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair::<f64>(s, ',') {
        None => None,
        Some((re, im)) => Some(Complex{re, im}),
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1,0"), Some(Complex{re: 1.0, im: 0.0}));
    assert_eq!(parse_complex("1,x"), None);
}


struct Bound {pub width: usize, pub height: usize}
struct Point {pub x: usize, pub y: usize}


/// Given the row and column of a pixel in the output image, return the
/// corresponding point on the complex plane.
/// 
/// `bounds` is a `Bound` giving the width and height of the image in pixels.
/// `pixel` is a `Point` indicating a particular pixel in that image.
/// The `upper_left` and `lower_right` parameters are points on the complex
/// plane designating the area our image covers.
fn pixel_to_point(
    bounds: &Bound,
    pixel: &Point,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>)
-> Complex<f64> {
    let width = lower_right.re - upper_left.re;
    let height = upper_left.im - lower_right.im;

    Complex {
        re: upper_left.re + pixel.x as f64 * width / bounds.width as f64,
        im: upper_left.im - pixel.y as f64 * height / bounds.height as f64
    }
}

#[test]
fn test_pixel_to_point() {
    assert_eq!(
        pixel_to_point(
            &Bound{width: 100, height: 200},
            &Point{x: 25, y: 175},
            Complex{ re: -1.0, im: 1.0},
            Complex{ re: 1.0, im: -1.0}),
        Complex { re: -0.5, im: -0.75 });
}

/// Render a rectangle of the Mandelbrot set into a buffer of pixels.
/// 
/// The `bounds` argument gives the width and height of the buffer of `pixels`,
/// which holds one grayscale pixel per byte. The `upper_left` and `lower_right`
/// arguments specify points on the complex plane corresponding to the upper-left
/// and lower-right corners of the pixel buffer.
fn render(
    pixels: &mut [u8],
    bounds: &Bound,
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
) {
    assert_eq!(pixels.len(), bounds.width * bounds.height);

    for row in 0..bounds.height {
        for column in 0..bounds.width {
            let point = pixel_to_point(
                bounds,
                &Point{x: column, y: row},
                upper_left,
                lower_right,
            );
            pixels[row * bounds.width + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                };
        }
    }
}

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
fn write_image(filename: &str, pixels: &[u8], bounds: Bound)
-> image::ImageResult<()> {
    image::save_buffer(
        filename, 
        pixels, 
        bounds.width as u32, 
        bounds.height as u32, 
        image::ColorType::L8)
}