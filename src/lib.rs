#![allow(dead_code)]

#[derive(Debug, PartialEq)]
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

    fn scale(&self, factor: f32) -> Self {
        let x = factor * self.x;
        let y = factor * self.y;
        let z = factor * self.z;

        Vector3D { x, y, z }
    }

    fn add(&self, anderer: &Vector3D) -> Self {
        let x = self.x + anderer.x;
        let y = self.y + anderer.y;
        let z = self.z + anderer.z;

        Vector3D { x, y, z }
    }

    fn sub(&self, anderer: &Vector3D) -> Self {
        let x = self.x - anderer.x;
        let y = self.y - anderer.y;
        let z = self.z - anderer.z;

        Vector3D { x, y, z }
    }

    fn laenge_quadriert(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    fn laenge(&self) -> f32 {
        self.laenge_quadriert().sqrt()
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

    #[test]
    fn test_vector_scaling() {
        // Given: Ein Ausgangsvektor und ein Skalierungsfaktor
        let v = Vector3D::neu(2.0, -4.0, 5.0);
        let factor = 3.0;

        // When: Wir die scale-Funktion aufrufen
        let result = v.scale(factor);

        // Then: Erwarten wir, dass alle Werte mit 3 multipliziert wurden
        let expected = Vector3D::neu(6.0, -12.0, 15.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_addition() {
        // Given: Zwei Vektoren im Raum
        let v1 = Vector3D::neu(1.0, 2.0, 3.0);
        let v2 = Vector3D::neu(4.0, 5.0, 6.0);

        // When: Wir v1 und v2 addieren
        let result = v1.add(&v2);

        // Then: Erwarten wir die komponentenweise Summe
        let expected = Vector3D::neu(5.0, 7.0, 9.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_subtraction() {
        // Given: Zwei Vektoren im Raum
        let v1 = Vector3D::neu(5.0, 5.0, 5.0);
        let v2 = Vector3D::neu(1.0, 2.0, 3.0);

        // When: Wir v2 von v1 abziehen
        let result = v1.sub(&v2);

        // Then: Erwarten wir die komponentenweise Differenz
        let expected = Vector3D::neu(4.0, 3.0, 2.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_laenge_quadriert() {
        // Given: Ein Vektor im Raum
        let v = Vector3D::neu(3.0, 4.0, 0.0);

        // When: Wir das Quadrat der Länge berechnen (x² + y² + z²)
        let result = v.laenge_quadriert();

        // Then: Erwarten wir 9.0 + 16.0 + 0.0 = 25.0
        assert_eq!(result, 25.0);
    }

    #[test]
    fn test_vector_laenge() {
        // Given: Ein Vektor (klassisches 3-4-5 Dreieck)
        let v = Vector3D::neu(3.0, 4.0, 0.0);

        // When: Wir die echte Länge berechnen
        let result = v.laenge();

        // Then: Erwarten wir die Quadratwurzel aus 25.0, also 5.0
        assert_eq!(result, 5.0);
    }
}
