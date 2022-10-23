#![allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

/// I'll want to refactor this eventually. Colors should inherit from Tuples
/// but inheritance doesn't exist in rust and it's all weird.

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
struct Color {
    red: f32,
    green: f32,
    blue: f32,
}

#[derive(Debug)]
#[derive(PartialEq)]
struct Canvas {
    width: i32,
    height: i32,
    pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new( width: i32, height: i32) -> Self {
        let mut pixels = Vec::new();
        for _i in 0..width {
            let mut vector = Vec::new();
            for _j in 2..height { // FIXME: Wait what the heck? 2?
                vector.push(color(0.0, 0.0, 0.0));
            }
            pixels.push(vector);
        }

        Canvas { width: width, height: height, pixels: pixels }
    }

    pub fn write_pixel( &mut self, x: i32, y: i32, color: Color) -> &Self {
        self.pixels[x as usize][y as usize] = color;
        return self;
    }

    pub fn pixel_at( &self, x: i32, y: i32 ) -> Color {
        self.pixels[x as usize][y as usize]
    }

    pub fn canvas_to_ppm(&self) -> &str {
"P3
5 3
255"
    }
}

fn main() {
    // let mut p = Projectile {
    //     position: point(0.0, 1.0, 0.0),
    //     velocity: normalize(&vector(1.0, 1.0, 0.0))
    // };

    // let e = Environment {
    //     gravity: vector(0.0, -0.1, 0.0),
    //     wind: vector(-0.01, 0.0, 0.0)
    // };

    // p = tick(&e, &p);
    // println!("{:?}", p);

    // p = tick(&e, &p);
    // println!("{:?}", p);

    // p = tick(&e, &p);
    // println!("{:?}", p);
}

/// BEGIN: for tick function

// struct Environment {
//     gravity: Tuple,
//     wind: Tuple,
// }

// #[derive(Debug)]
// struct Projectile {
//     position: Tuple,
//     velocity: Tuple,
// }

// fn tick(env: &Environment, proj: &Projectile) -> Projectile {
//     let position = add_tuples(&proj.position, &proj.velocity);
//     let mut velocity = add_tuples(&proj.velocity, &env.gravity);
//     // also have to add env.wind... currently can't add multiple things
//     velocity = add_tuples(&velocity, &env.wind);

//     Projectile { position: position, velocity: velocity }
// }

/// END: for tick functions
///
fn point(x: f32, y: f32, z: f32) -> Tuple {
    let point: Tuple = Tuple { x: x, y: y, z: z, w: 1.0 };
    return point;
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    let vector: Tuple = Tuple { x: x, y: y, z: z, w: 0.0 };
    return vector;
}

fn color(red: f32, green: f32, blue: f32) -> Color {
    let color: Color = Color { red: red, green: green, blue: blue};
    color
}

fn tuple_is_point(tuple: &Tuple) -> bool {
    // Returns true if the tuple is a point, false if it's a vector
    if tuple.w == 1.0 {
        return true;
    }
    else if tuple.w == 0.0 {
        return false;
    }

    panic!["This tuple is neither a point nor a vector!"];
}

fn add_tuples(tuple1: &Tuple, tuple2: &Tuple) -> Tuple {
    let result: Tuple = Tuple {
        x: tuple1.x + tuple2.x,
        y: tuple1.y + tuple2.y,
        z: tuple1.z + tuple2.z,
        w: tuple1.w + tuple2.w,
    };
    
    result
}

fn add_colors(color1: &Color, color2: &Color) -> Color {
    Color {
        red: round_number(color1.red + color2.red, 10.0),
        green: round_number(color1.green + color2.green, 10.0),
        blue: round_number(color1.blue + color2.blue, 10.0),
    }
}

fn sub_tuples(tuple1: &Tuple, tuple2: &Tuple) -> Tuple {
    let result: Tuple = Tuple {
        x: tuple1.x - tuple2.x,
        y: tuple1.y - tuple2.y,
        z: tuple1.z - tuple2.z,
        w: tuple1.w - tuple2.w,
    };

    result
}

fn sub_colors(color1: &Color, color2: &Color) -> Color {
    Color {
        red: round_number(color1.red - color2.red, 10.0),
        green: round_number(color1.green - color2.green, 10.0),
        blue: round_number(color1.blue - color2.blue, 10.0),
    }
}

fn round_number(float: f32, factor: f32) -> f32 {
    (float * factor).round() / factor
}

fn negate(tuple: &Tuple) -> Tuple {
    Tuple { x: -tuple.x, y: -tuple.y, z: -tuple.z, w: -tuple.w }
}

fn mult_scalar(scalar: f32, tuple: &Tuple) -> Tuple {
    Tuple { x: scalar * tuple.x, y: scalar * tuple.y, z: scalar * tuple.z, w: scalar * tuple.w }
}

fn mult_color(scalar: f32, color: &Color) -> Color {
    Color { red: scalar * color.red, green: scalar * color.green, blue: scalar * color.blue }
}

fn hadamard(color1: &Color, color2: &Color) -> Color {
    Color {
        red: round_number(color1.red * color2.red, 100.00),
        green: round_number(color1.green * color2.green, 100.00),
        blue: round_number(color1.blue * color2.blue, 100.00)
    }
}

fn div_scalar(scalar: f32, tuple: &Tuple) -> Tuple {
    Tuple { x: tuple.x / scalar, y: tuple.y / scalar, z: tuple.z / scalar, w: tuple.w / scalar }
}

fn magnitude(vector: &Tuple) -> f32 {
    (vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0) + vector.w.powf(2.0)).sqrt()
}

fn normalize(vector: &Tuple) -> Tuple {
    let mag = magnitude(vector);
    Tuple {
        x: vector.x / mag,
        y: vector.y / mag,
        z: vector.z / mag,
        w: vector.w / mag
    }
}

fn dot(tuple1: Tuple, tuple2: Tuple) -> f32 {
    tuple1.x * tuple2.x + tuple1.y * tuple2.y + tuple1.z * tuple2.z + tuple1.w * tuple2.w
}

fn cross(vector1: &Tuple, vector2: &Tuple) -> Tuple {
    vector(vector1.y * vector2.z - vector1.z * vector2.y,
           vector1.z * vector2.x - vector1.x * vector2.z,
           vector1.x * vector2.y - vector1.y * vector2.x
        )
}

#[cfg(test)]
mod tests {
    use super:: *;

    #[test]
    fn test_point() {
        let point = point(4.3, -4.2, 3.1);
        assert_eq!(point.x, 4.3);
        assert_eq!(point.y, -4.2);
        assert_eq!(point.z, 3.1);
        assert_eq!(point.w, 1.0);
    }

    #[test]
    fn test_vector() {
        let vector = vector(4.3, -4.2, 3.1);
        assert_eq!(vector.x, 4.3);
        assert_eq!(vector.y, -4.2);
        assert_eq!(vector.z, 3.1);
        assert_eq!(vector.w, 0.0);
    }

    #[test]
    fn test_color() {
        let new_color = color(-0.5, 0.4, 1.7);
        assert_eq!(new_color.red, -0.5);
        assert_eq!(new_color.green, 0.4);
        assert_eq!(new_color.blue, 1.7);
    }

    #[test]
    fn test_round_numbers() {
        let mut rounded_number = round_number(123.456789, 10.0);
        assert_eq!(rounded_number, 123.5);

        rounded_number = round_number(123.456789, 100.0);
        assert_eq!(rounded_number, 123.46);

        rounded_number = round_number(123.456789, 0.01);
        assert_eq!(rounded_number, 100.0);
    }

    #[test]
    fn test_tuple_is_point() {
        let point = point(4.3, -4.2, 3.1);
        assert_eq!(tuple_is_point(&point), true);

        let vector = vector(4.3, -4.2, 3.1);
        assert_eq!(tuple_is_point(&vector), false);
    }

    #[test]
    fn test_add_tuples() {
        let point = point(3.0, -2.0, 5.0);
        let vector = vector(-2.0, 3.0, 1.0);
        let result = add_tuples(&point, &vector);
        assert_eq!(result, Tuple { x: 1.0, y: 1.0, z: 6.0, w: 1.0 });
    }

    #[test]
    fn test_add_colors() {
        let color1 = color(0.9, 0.6, 0.75);
        let color2 = color(0.7, 0.1, 0.25);
        assert_eq!(add_colors(&color1, &color2), color(1.6, 0.7, 1.0));
    }

    #[test]
    fn test_subtract_points() {
        let point1 = point(3.0, 2.0, 1.0);
        let point2 = point(5.0, 6.0, 7.0);
        let result = sub_tuples(&point1, &point2);
        assert_eq!(result, Tuple { x: -2.0, y: -4.0, z: -6.0, w: 0.0 });
    }

    #[test]
    fn test_subtract_colors() {
        let color1 = color(0.9, 0.6, 0.75);
        let color2 = color(0.7, 0.1, 0.25);
        assert_eq!(sub_colors(&color1, &color2), color(0.2, 0.5, 0.5));
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let point = point(3.0, 2.0, 1.0);
        let vector = vector(5.0, 6.0, 7.0);
        let result = sub_tuples(&point, &vector);
        assert_eq!(result, Tuple { x: -2.0, y: -4.0, z: -6.0, w: 1.0 });
    }

    #[test]
    fn test_subtract_vectors() {
        let vector1 = vector(3.0, 2.0, 1.0);
        let vector2 = vector(5.0, 6.0, 7.0);
        let result = sub_tuples(&vector1, &vector2);
        assert_eq!(result, Tuple { x: -2.0, y: -4.0, z: -6.0, w: 0.0 });
    }

    #[test]
    fn test_subtracting_a_vector_from_a_zero_vector() {
        let zero_vector = vector(0.0, 0.0, 0.0);
        let vector = vector(1.0, -2.0, 3.0);
        let result = sub_tuples(&zero_vector, &vector);
        assert_eq!(result, Tuple { x: -1.0, y: 2.0, z: -3.0, w: 0.0 });
    }

    #[test]
    fn test_negate_a_tuple() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = negate(&tuple);
        assert_eq!(result, Tuple { x: -1.0, y: 2.0, z: -3.0, w: 4.0 });
    }

    #[test]
    fn test_multiply_a_tuple_by_a_scalar() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = mult_scalar(3.5, &tuple);
        assert_eq!(result, Tuple { x: 3.5, y: -7.0, z: 10.5, w: -14.0 });
    }

    #[test]
    fn test_multiply_a_color_by_a_scalar() {
        let new_color = color(0.2, 0.3, 0.4);
        assert_eq!(mult_color(2.0, &new_color), color(0.4, 0.6, 0.8));
    }
    
    #[test]
    fn test_hadarmard_product() {
        let color1 = color(1.0, 0.2, 0.4);
        let color2 = color(0.9, 1.0, 0.1);
        assert_eq!(hadamard(&color1, &color2), color(0.9, 0.2, 0.04));
    }

    #[test]
    fn test_multiply_a_tuple_by_a_fraction() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = mult_scalar(0.5, &tuple);
        assert_eq!(result, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 });
    }

    #[test]
    fn test_divide_a_tuple_by_a_scalar() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = div_scalar(2.0, &tuple);
        assert_eq!(result, Tuple { x: 0.5, y: -1.0, z: 1.5, w: -2.0 });
    }

    #[test]
    fn test_computing_the_magnitude_of_vectors() {
        let vector1 = vector(1.0, 0.0, 0.0);
        let result1 = magnitude(&vector1);
        assert_eq!(result1, 1.0);

        let vector2 = vector(0.0, 1.0, 0.0);
        let result2 = magnitude(&vector2);
        assert_eq!(result2, 1.0);

        let vector3 = vector(0.0, 0.0, 1.0);
        let result3 = magnitude(&vector3);
        assert_eq!(result3, 1.0);

        let vector4 = vector(1.0, 2.0, 3.0);
        let result4 = magnitude(&vector4);
        assert_eq!(result4, 14.0_f32.sqrt());

        let vector5 = vector(-1.0, -2.0, -3.0);
        let result5 = magnitude(&vector5);
        assert_eq!(result5, 14.0_f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        let vec = vector(4.0, 0.0, 0.0);
        assert_eq!(normalize(&vec), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_dot_product() {
        let vector1 = vector(1.0, 2.0, 3.0);
        let vector2 = vector(2.0, 3.0, 4.0);

        assert_eq!(dot(vector1, vector2), 20.0);
    }

    #[test]
    fn test_cross_product() {
        let vector1 = vector(1.0, 2.0, 3.0);
        let vector2 = vector(2.0, 3.0, 4.0);

        assert_eq!(cross(&vector1, &vector2), vector(-1.0, 2.0, -1.0));
        assert_eq!(cross(&vector2, &vector1), vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_creating_a_canvas() {
        // let canvas1 = Canvas { width: 10, height: 20};
        let canvas1 = Canvas::new(10, 20);
        assert_eq!(canvas1.width, 10);
        assert_eq!(canvas1.height, 20);
        for pixel in canvas1.pixels {
            for p in pixel {
                assert_eq!(p, color(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn test_writing_pixels_to_a_canvas() {
        let mut canvas1 = Canvas::new(10, 20);
        let red = color(1.0, 0.0, 0.0);
        canvas1.write_pixel(2, 3, red);
        assert_eq!(canvas1.pixel_at(2, 3), red);
    }

    #[test]
    fn test_constructing_the_ppm_header() {
        let canvas1 = Canvas::new(5, 3);
        let ppm = canvas1.canvas_to_ppm();
        assert_eq!{
            ppm,
"P3
5 3
255"
        }
    }

}