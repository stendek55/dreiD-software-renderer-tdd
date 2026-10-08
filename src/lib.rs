#![allow(dead_code)]

struct Vector3D {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3D {
    //konstruktor
    fn neu(x: f32, y: f32, z: f32) -> Self {
        Vector3D { x, y, z }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_drei_dimensionen_neu_erzeugen() {
        let x_wert = 5.5;
        let y_wert = 2.3;
        let z_wert = 4.2;

        let vector = Vector3D::neu(x_wert, y_wert, z_wert);

        assert_eq!(vector.x, x_wert);
        assert_eq!(vector.y, y_wert);
        assert_eq!(vector.z, z_wert);
    }
}
