struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

fn main() {
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    let point: Tuple = Tuple { x: x, y: y, z: z, w: 1.0 };
    return point;
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    let vector: Tuple = Tuple { x: x, y: y, z: z, w: 0.0 };
    return vector;
}

fn tuple_is_point(tuple: Tuple) -> bool {
    // Returns true if the tuple is a point, false if it's a vector
    if tuple.w == 1.0 {
        return true;
    }
    else if tuple.w == 0.0 {
        return false;
    }

    panic!["This tuple is neither a point nor a vector!"];
}

fn add_tuples(tuple1: Tuple, tuple2: Tuple) -> Tuple {
    let result: Tuple = Tuple {
        x: tuple1.x + tuple2.x,
        y: tuple1.y + tuple2.y,
        z: tuple1.z + tuple2.z,
        w: tuple1.w + tuple2.w,
    };
    
    result
}

fn sub_tuples(tuple1: Tuple, tuple2: Tuple) -> Tuple {
    let result: Tuple = Tuple {
        x: tuple1.x - tuple2.x,
        y: tuple1.y - tuple2.y,
        z: tuple1.z - tuple2.z,
        w: tuple1.w - tuple2.w,
    };

    result
}

fn negate(tuple: &Tuple) -> Tuple {
    Tuple { x: -tuple.x, y: -tuple.y, z: -tuple.z, w: -tuple.w }
}

fn mult_scalar(scalar: f32, tuple: &Tuple) -> Tuple {
    Tuple { x: scalar * tuple.x, y: scalar * tuple.y, z: scalar * tuple.z, w: scalar * tuple.w }
}

fn div_scalar(scalar: f32, tuple: &Tuple) -> Tuple {
    Tuple { x: tuple.x / scalar, y: tuple.y / scalar, z: tuple.z / scalar, w: tuple.w / scalar }
}

fn magnitude(vector: &Tuple) -> f32 {
    (vector.x.powf(2.0) + vector.y.powf(2.0) + vector.z.powf(2.0) + vector.w.powf(2.0)).sqrt()
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
    fn test_tuple_is_point() {
        let point = point(4.3, -4.2, 3.1);
        assert_eq!(tuple_is_point(point), true);

        let vector = vector(4.3, -4.2, 3.1);
        assert_eq!(tuple_is_point(vector), false);
    }

    #[test]
    fn test_add_tuples() {
        let point = point(3.0, -2.0, 5.0);
        let vector = vector(-2.0, 3.0, 1.0);
        let result = add_tuples(point, vector);
        assert_eq!(result.x, 1.0);
        assert_eq!(result.y, 1.0);
        assert_eq!(result.z, 6.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_subtract_points() {
        let point1 = point(3.0, 2.0, 1.0);
        let point2 = point(5.0, 6.0, 7.0);
        let result = sub_tuples(point1, point2);
        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_subtract_vector_from_point() {
        let point = point(3.0, 2.0, 1.0);
        let vector = vector(5.0, 6.0, 7.0);
        let result = sub_tuples(point, vector);
        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.w, 1.0);
    }

    #[test]
    fn test_subtract_vectors() {
        let vector1 = vector(3.0, 2.0, 1.0);
        let vector2 = vector(5.0, 6.0, 7.0);
        let result = sub_tuples(vector1, vector2);
        assert_eq!(result.x, -2.0);
        assert_eq!(result.y, -4.0);
        assert_eq!(result.z, -6.0);
        assert_eq!(result.w, 0.0);
    }

    #[test]
    fn test_subtracting_a_vector_from_a_zero_vector() {
        let zero_vector = vector(0.0, 0.0, 0.0);
        let vector = vector(1.0, -2.0, 3.0);
        let result = sub_tuples(zero_vector, vector);
        assert_eq!(result.x, -1.0);
        assert_eq!(result.y, 2.0);
        assert_eq!(result.z, -3.0);
    }

    #[test]
    fn test_negate_a_tuple() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let negated_tuple : Tuple = negate(&tuple);

        assert_eq!(-tuple.x, negated_tuple.x);
        assert_eq!(-tuple.y, negated_tuple.y);
        assert_eq!(-tuple.z, negated_tuple.z);
        assert_eq!(-tuple.w, negated_tuple.w);
    }

    #[test]
    fn test_multiply_a_tuple_by_a_scalar() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = mult_scalar(3.5, &tuple);
        assert_eq!(result.x, 3.5);
        assert_eq!(result.y, -7.0);
        assert_eq!(result.z, 10.5);
        assert_eq!(result.w, -14.0);
    }

    #[test]
    fn test_multiply_a_tuple_by_a_fraction() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = mult_scalar(0.5, &tuple);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, 1.5);
        assert_eq!(result.w, -2.0);
    }

    #[test]
    fn test_divide_a_tuple_by_a_scalar() {
        let tuple : Tuple = Tuple { x: 1.0, y: -2.0, z: 3.0, w: -4.0 };
        let result : Tuple = div_scalar(2.0, &tuple);
        assert_eq!(result.x, 0.5);
        assert_eq!(result.y, -1.0);
        assert_eq!(result.z, 1.5);
        assert_eq!(result.w, -2.0);
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
}