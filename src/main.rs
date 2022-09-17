struct Tuple {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

fn main() {
}

fn point(x: f32, y: f32, z: f32) -> Tuple {
    let point: Tuple = Tuple {x: x, y: y, z: z, w: 1.0};
    return point;
}

fn vector(x: f32, y: f32, z: f32) -> Tuple {
    let vector: Tuple = Tuple {x: x, y: y, z: z, w: 0.0};
    return vector;
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
}