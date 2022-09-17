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
    return result;
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
}