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

    /// Mathematisch: Skalierung (Multiplikation mit einem Skalar).
    /// Verlängert oder verkürzt den Vektor um den Faktor `factor`, ohne seine Richtung zu ändern.
    /// Bei einem negativen Faktor dreht sich die Richtung des Vektors um 180 Grad um.
    fn scale(&self, factor: f32) -> Self {
        let x = factor * self.x;
        let y = factor * self.y;
        let z = factor * self.z;

        Vector3D { x, y, z }
    }

    /// Mathematisch: Vektoraddition.
    /// Berechnet die komponentenweise Summe zweier Vektoren.
    /// Geometrisch entspricht dies dem Hintereinanderhängen der beiden Vektorpfeile im Raum.
    fn add(&self, anderer: &Vector3D) -> Self {
        let x = self.x + anderer.x;
        let y = self.y + anderer.y;
        let z = self.z + anderer.z;

        Vector3D { x, y, z }
    }

    /// Mathematisch: Vektorsubtraktion.
    /// Berechnet die komponentenweise Differenz zweier Vektoren.
    /// Geometrisch ergibt dies den Richtungsvektor, der von der Spitze des Vektors `anderer`
    /// direkt zur Spitze des Vektors `self` zeigt (wichtig für Abstands- und Blickrichtungen).
    fn sub(&self, anderer: &Vector3D) -> Self {
        let x = self.x - anderer.x;
        let y = self.y - anderer.y;
        let z = self.z - anderer.z;

        Vector3D { x, y, z }
    }

    /// Mathematisch: Quadrierte Euklidische Norm (Skalarprodukt des Vektors mit sich selbst).
    /// Berechnet die Summe der quadrierten Komponenten (x² + y² + z²).
    /// Dient im Renderer als performancefreundlicher Ersatz für Längenvergleiche,
    /// da hierbei die rechenintensive Quadratwurzel entfällt.
    fn laenge_quadriert(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    /// Mathematisch: Euklidische Norm (Der Betrag des Vektors).
    /// Berechnet die reelle, geometrische Länge des Vektors im dreidimensionalen Raum
    /// durch Anwendung des Satzes von Pythagoras (Wurzel aus der quadrierten Länge).
    fn laenge(&self) -> f32 {
        self.laenge_quadriert().sqrt()
    }

    /// Mathematisch: Kreuzprodukt (Vektorprodukt).
    /// Erzeugt einen neuen Vektor, der geometrisch perfekt senkrecht (orthogonal) auf beiden
    /// Ausgangsvektoren steht. Die Richtung folgt der Rechten-Hand-Regel.
    /// Die Länge des Ergebnisvektors entspricht dem Flächeninhalt des Parallelogramms,
    /// das von beiden Vektoren aufgespannt wird (essenziell für Dreiecksnormalen und Kamerasysteme).
    fn cross(&self, anderer: &Vector3D) -> Vector3D {
        let x = (self.y * anderer.z) - (self.z * anderer.y);
        let y = (self.z * anderer.x) - (self.x * anderer.z);
        let z = (self.x * anderer.y) - (self.y * anderer.x);

        Vector3D { x, y, z }
    }

    /// Mathematisch: Vektornormalisierung (Erzeugung eines Einheitsvektors).
    /// Skaliert den Vektor so, dass seine geometrische Länge exakt `1.0` beträgt,
    /// während seine ursprüngliche Richtung im Raum unverändert bleibt.
    ///
    /// # Verwendung im Renderer
    /// Essentiell für Beleuchtungsberechnungen (z.B. Lambert-Shading), da das
    /// Skalarprodukt zweier normalisierter Vektoren direkt den Kosinus des Winkels
    /// zwischen ihnen liefert (Blickrichtung, Lichtrichtung, Oberflächennormalen).
    ///
    /// # Mathematische Formel
    /// v_normalisiert = v / ||v|| = (x / laenge, y / laenge, z / laenge)
    ///
    /// # Panics / Edge Cases
    /// Wenn der Vektor eine Länge von `0.0` hat (Nullvektor), führt die Division
    /// durch Null dazu, dass die Komponenten des Ergebnisvektors `f32::NAN` werden.
    fn normalisiere(&self) -> Vector3D {
        let laenge = self.laenge();

        if laenge == 0.0 {
            Vector3D::neu(0.0, 0.0, 0.0)
        } else {
            Vector3D {
                x: self.x / laenge,
                y: self.y / laenge,
                z: self.z / laenge,
            }
        }
    }

    /// Mathematisch: Skalarprodukt (Dot Product) zweier 3D-Vektoren.
    ///
    /// Das Skalarprodukt multipliziert zwei Vektoren komponentenweise und
    /// addiert die Ergebnisse. Es liefert als Resultat eine einzelne Zahl (Skalar).
    ///
    /// # Mathematische Formel
    /// Algebraisch:
    /// a · b = (a.x * b.x) + (a.y * b.y) + (a.z * b.z)
    ///
    /// Geometrisch:
    /// a · b = ||a|| * ||b|| * cos(θ)  (wobei θ der Winkel zwischen den Vektoren ist)
    ///
    /// # Geometrische Bedeutung (bei normalisierten Vektoren)
    /// * ** 1.0**: Vektoren sind absolut identisch (Parallel, Winkel 0°).
    /// * ** 0.0**: Vektoren stehen exakt senkrecht aufeinander (Orthogonal, Winkel 90°).
    /// * **-1.0**: Vektoren zeigen in exakt entgegengesetzte Richtungen (Winkel 180°).
    fn skalarprodukt(&self, anderer: &Vector3D) -> f32 {
        (self.x * anderer.x) + (self.y * anderer.y) + (self.z * anderer.z)
    }
}

#[derive(Debug, PartialEq)]
struct Matrix4D {
    m: [[f32; 4]; 4],
}

impl Matrix4D {
    //konstruktor -> Einheitsmatrix
    fn identitaet() -> Self {
        Matrix4D {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    //konstruktor -> Nullmatrix
    fn nuller() -> Self {
        Matrix4D { m: [[0.0; 4]; 4] }
    }

    /// Mathematisch: Erstellung einer Translationsmatrix (Verschiebungsmatrix).
    ///
    /// Platziert die Verschiebungswerte (x, y, z) in die vierte Spalte der Matrix.
    /// Bei der Multiplikation mit einem Punkt (w = 1.0) werden diese Werte linear aufaddiert.
    ///
    /// # Verwendung im Renderer
    /// Verschiebt 3D-Objekte oder die Kamera frei durch den virtuellen Raum.
    fn translation(vector: &Vector3D) -> Self {
        let mut mat = Matrix4D::identitaet();

        mat.m[0][3] = vector.x;
        mat.m[1][3] = vector.y;
        mat.m[2][3] = vector.z;

        mat
    }

    /// Mathematisch: Erstellung einer Skalierungsmatrix.
    ///
    /// Multipliziert die Hauptdiagonale der Matrix mit den jeweiligen Skalierungsfaktoren.
    /// Die vierte Komponente (w) bleibt unverändert auf 1.0.
    ///
    /// # Verwendung im Renderer
    /// Erlaubt es, 3D-Modelle in den drei Raumachsen unabhängig voneinander zu strecken (Faktor > 1.0)
    /// oder zu stauchen (Faktor < 1.0).
    fn skalierung(skal_x: f32, skal_y: f32, skal_z: f32) -> Self {
        let mut mat = Matrix4D::identitaet();

        mat.m[0][0] = skal_x;
        mat.m[1][1] = skal_y;
        mat.m[2][2] = skal_z;

        mat
    }

    /// Mathematisch: Erstellung einer Rotationsmatrix um die Z-Achse.
    ///
    /// Dreht alle Punkte in der X-Y-Ebene um den Ursprung. Die Z-Koordinate bleibt dabei unverändert.
    /// Die Berechnung nutzt die trigonometrischen Funktionen `cos` und `sin` des Winkels.
    ///
    /// # Verwendung im Renderer
    /// Dreht ein Objekt (oder die Kamera) im Uhrzeigersinn bzw. Gegenuhrzeigersinn,
    /// wenn man direkt von vorne auf den Bildschirm blickt (wichtig für Roll-Bewegungen).
    ///
    /// # Winkelmaß
    /// Erwartet den Winkel im **Bogenmaß** (Radians). Ein Vollkreis entspricht 2 * PI.
    fn rotation_z(rot_winkel: f32) -> Self {
        let mut mat = Matrix4D::identitaet();
        let cos = rot_winkel.cos();
        let sin = rot_winkel.sin();

        mat.m[0][0] = cos;
        mat.m[0][1] = -sin;
        mat.m[1][0] = sin;
        mat.m[1][1] = cos;

        mat
    }

    /// Mathematisch: Multiplikation der Matrix mit einem 3D-Punkt über homogene Koordinaten (w = 1.0).
    ///
    /// (Skalierung, Rotation und Verschiebung/Translation) auf einen dreidimensionalen Punkt an.
    ///
    /// # Verwendung im Renderer
    /// Das ist die zentrale Funktion, um die Eckpunkte (Vertices) deiner 3D-Modelle
    /// durch den Raum zu bewegen, zu drehen und schließlich in das Sichtfeld der Kamera zu rücken.
    ///
    /// # Mathematischer Hintergrund (Homogene Koordinaten)
    /// Ein 3D-Punkt besitzt eigentlich nur drei Komponenten (x, y, z). Damit er mit einer
    /// 4x4-Matrix multipliziert werden kann, wird er virtuell um eine vierte Komponente
    /// erweitert: die homogene Koordinate w = 1.0.
    ///
    /// Die Berechnung für jede Koordinate entspricht dem Skalarprodukt einer Matrixzeile
    /// mit dem erweiterten Vektor (x, y, z, 1.0):
    ///
    /// Da am Ende die vierte Komponente (w) bei Standardtransformationen 1.0 bleibt,
    /// gibt die Funktion direkt einen klassischen `Vector3D` mit den neuen Raumkoordinaten zurück.
    fn multipliziere_punkt(&self, punkt: &Vector3D) -> Vector3D {
        let x = (self.m[0][0] * punkt.x)
            + (self.m[0][1] * punkt.y)
            + (self.m[0][2] * punkt.z)
            + self.m[0][3];

        let y = (self.m[1][0] * punkt.x)
            + (self.m[1][1] * punkt.y)
            + (self.m[1][2] * punkt.z)
            + self.m[1][3];

        let z = (self.m[2][0] * punkt.x)
            + (self.m[2][1] * punkt.y)
            + (self.m[2][2] * punkt.z)
            + self.m[2][3];

        Vector3D { x, y, z }
    }

    /// Mathematisch: Multiplikation zweier 4x4-Matrizen (Matrixprodukt).
    ///
    /// Berechnet die Kombination zweier geometrischer Transformationen. Das Ergebnis
    /// ist eine neue Matrix, die beide Transformationen nacheinander ausführt.
    ///
    /// # Mathematische Formel & Prinzip (Zeile mal Spalte)
    ///
    /// # Wichtige Eigenschaft (Nicht kommutativ)
    /// Die Matrixmultiplikation ist nicht kommutativ (`A * B != B * A`). Die Reihenfolge
    /// ist entscheidend dafür, ob ein Objekt erst rotiert und dann verschoben wird,
    /// oder umgekehrt.
    fn multipliziere_matrix(&self, andere: &Matrix4D) -> Matrix4D {
        let mut mtrx = Matrix4D::nuller();
        for zln in 0..=3 {
            for spltn in 0..=3 {
                for indx in 0..=3 {
                    mtrx.m[zln][spltn] += self.m[zln][indx] * andere.m[indx][spltn];
                }
            }
        }

        mtrx
    }
}

//#########################################################################
//###########################-----UNIT-TESTS-----##########################
//#########################################################################
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

    #[test]
    fn test_vector_kreuzprodukt_standard_achsen() {
        // Given: Die Standard-Achsen des Koordinatensystems
        let x_achse = Vector3D::neu(1.0, 0.0, 0.0);
        let y_achse = Vector3D::neu(0.0, 1.0, 0.0);

        // When: Wir das Kreuzprodukt (Cross Product) aus X und Y berechnen
        let result = x_achse.cross(&y_achse);
        dbg!(&result);

        // Then: In einem rechtshändigen Koordinatensystem MUSS das die Z-Achse ergeben
        let expected = Vector3D::neu(0.0, 0.0, 1.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_kreuzprodukt_beliebig() {
        // Given: Zwei beliebige Vektoren im Raum
        let v1 = Vector3D::neu(2.0, 1.0, 3.0);
        let v2 = Vector3D::neu(4.0, -2.0, 5.0);

        // When: Wir das Kreuzprodukt berechnen
        // Mathematisch:
        // cx = (1 * 5) - (3 * -2) = 5 - (-6) = 11
        // cy = (3 * 4) - (2 * 5)  = 12 - 10   = 2
        // cz = (2 * -2) - (1 * 4) = -4 - 4    = -8
        let result = v1.cross(&v2);
        dbg!(&result);

        // Then: Erwarten wir den Vektor (11.0, 2.0, -8.0)
        let expected = Vector3D::neu(11.0, 2.0, -8.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_identitaet_erzeugen() {
        // When: Wir eine Identitätsmatrix anfordern
        let m = Matrix4D::identitaet();

        // Then: Müssen die Hauptdiagonalen 1.0 sein, der Rest 0.0
        assert_eq!(m.m[0][0], 1.0);
        assert_eq!(m.m[1][1], 1.0);
        assert_eq!(m.m[2][2], 1.0);
        assert_eq!(m.m[3][3], 1.0);

        assert_eq!(m.m[0][1], 0.0);
        assert_eq!(m.m[3][2], 0.0);
    }

    #[test]
    fn test_matrix_multiplikation_mit_identitaet() {
        // Given: Eine Identitätsmatrix und ein beliebiger Vektor (als Punkt, w = 1.0)
        let m = Matrix4D::identitaet();
        let v = Vector3D::neu(2.5, -3.0, 4.2);

        // When: Wir den Vektor mit der Identitätsmatrix multiplizieren
        let result = m.multipliziere_punkt(&v);

        // Then: Der Vektor darf sich absolut nicht verändert haben
        assert_eq!(result, v);
    }

    #[test]
    fn test_matrix_punkt_verschiebung_translation() {
        // Given: Eine Translationsmatrix, die um X=2, Y=3, Z=-1 verschiebt
        // (Die Verschiebewerte stehen in der letzten Spalte der Matrix)
        let mut m = Matrix4D::identitaet();
        m.m[0][3] = 2.0; // Translation X
        m.m[1][3] = 3.0; // Translation Y
        m.m[2][3] = -1.0; // Translation Z

        let punkt = Vector3D::neu(1.0, 1.0, 1.0);

        // When: Wir den Punkt transformieren
        let result = m.multipliziere_punkt(&punkt);

        // Then: Erwarten wir die addierten Verschiebungen
        let expected = Vector3D::neu(3.0, 4.0, 0.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_normalisieren() {
        // Given: Ein unnormierter Vektor
        let v = Vector3D::neu(3.0, 0.0, 4.0); // Länge ist 5.0

        // When: Wir den Vektor normalisieren (Richtung bleibt gleich, Länge wird 1.0)
        let result = v.normalisiere();

        // Then: Jede Komponente muss durch die Länge (5.0) geteilt worden sein
        let expected = Vector3D::neu(0.6, 0.0, 0.8);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_vector_skalarprodukt_orthogonal_und_parallel() {
        // Given: Zwei orthogonale (senkrechte) Vektoren
        let v1 = Vector3D::neu(1.0, 0.0, 0.0);
        let v2 = Vector3D::neu(0.0, 1.0, 0.0);

        // When/Then: Das Skalarprodukt (Dot Product) von senkrechten Vektoren MUSS 0.0 sein
        assert_eq!(
            v1.skalarprodukt(&v2),
            0.0,
            "skalarprodukt von senkrechten vektor ist nicht 0!"
        );

        // Given: Zwei parallele Vektoren
        let v3 = Vector3D::neu(2.0, 3.0, -1.0);

        // When/Then: Das Skalarprodukt mit sich selbst entspricht der quadrierten Länge
        assert_eq!(
            v3.skalarprodukt(&v3),
            v3.laenge_quadriert(),
            "skalarprodukt mit sich selbst hat falsche länge!"
        );
    }

    #[test]
    fn test_matrix_multiplikation_isolierte_berechnung() {
        // Given: Eine beliebige Test-Matrix mit bekannten Werten
        let mut m1 = Matrix4D::identitaet();
        m1.m[0][0] = 1.0;
        m1.m[0][1] = 2.0;
        m1.m[0][2] = 3.0;
        m1.m[0][3] = 4.0;
        m1.m[1][0] = 5.0;
        m1.m[1][1] = 6.0;
        m1.m[1][2] = 7.0;
        m1.m[1][3] = 8.0;
        // ... (Rest ist Einheitsmatrix)

        // Und Given: Die mathematische Einheitsmatrix (Identität)
        let i = Matrix4D::identitaet();

        // When: Wir eine Matrix mit der Einheitsmatrix multiplizieren (M * I = M)
        let result = m1.multipliziere_matrix(&i);

        // Then: Das Ergebnis MUSS exakt der ursprünglichen Matrix entsprechen
        assert_eq!(result, m1);
    }

    #[test]
    fn test_matrix_multiplikation_vollstaendig() {
        // Given: Zwei Matrizen. Eine skaliert, die andere verschiebt.
        let mut m_skalierung = Matrix4D::identitaet();
        m_skalierung.m[0][0] = 2.0; // Skaliere X mal 2
        m_skalierung.m[1][1] = 2.0; // Skaliere Y mal 2
        m_skalierung.m[2][2] = 2.0; // Skaliere Z mal 2

        let mut m_translation = Matrix4D::identitaet();
        m_translation.m[0][3] = 5.0; // Verschiebe X um 5

        // When: Wir beide Matrizen miteinander multiplizieren (Kombination)
        // Mathematisch: kombinierte_matrix = m_translation * m_skalierung
        let kombinierte_matrix = m_translation.multipliziere_matrix(&m_skalierung);

        // Ein Punkt bei (1, 1, 1) sollte erst auf (2, 2, 2) skaliert
        // und dann auf X um 5 verschoben werden -> (7, 2, 2)
        let punkt = Vector3D::neu(1.0, 1.0, 1.0);
        let result = kombinierte_matrix.multipliziere_punkt(&punkt);

        // Then: Das Ergebnis muss die kombinierte Transformation widerspiegeln
        let expected = Vector3D::neu(7.0, 2.0, 2.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_erstelle_translation() {
        // Given: Ein Verschiebungsvektor
        let verschiebung = Vector3D::neu(3.0, -2.5, 7.0);

        // When: Wir eine dedizierte Translationsmatrix erstellen
        let m = Matrix4D::translation(&verschiebung);

        // Then: Muss ein Punkt bei (0,0,0) exakt auf den Verschiebungsvektor wandern
        let start_punkt = Vector3D::neu(0.0, 0.0, 0.0);
        let result = m.multipliziere_punkt(&start_punkt);

        assert_eq!(result, verschiebung);
    }

    #[test]
    fn test_matrix_erstelle_skalierung() {
        // Given: Ein Skalierungswert für alle 3 Achsen
        let m = Matrix4D::skalierung(2.0, 0.5, 3.0);
        let punkt = Vector3D::neu(10.0, 10.0, 10.0);

        // When: Wir den Punkt skalieren
        let result = m.multipliziere_punkt(&punkt);

        // Then: Erwarten wir die komponentenweise Multiplikation
        let expected = Vector3D::neu(20.0, 5.0, 30.0);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_rotation_z_achse() {
        // Given: Eine Rotation um 90 Grad (PI / 2 Bogenmaß) um die Z-Achse
        use std::f32::consts::PI;
        let winkel = PI / 2.0;
        let m = Matrix4D::rotation_z(winkel);

        // Ein Punkt auf der X-Achse (1, 0, 0)
        let punkt = Vector3D::neu(1.0, 0.0, 0.0);

        // When: Wir den Punkt rotieren
        let result = m.multipliziere_punkt(&punkt);

        // Then: Dreht sich der Punkt im Gegenuhrzeigersinn auf die Y-Achse (0, 1, 0)
        // Hinweis: Da f32 Ungenauigkeiten besitzt, prüfen wir mit einer kleinen Toleranz (Epsilon)
        let epsilon = 0.00001;
        assert!((result.x - 0.0).abs() < epsilon, "X war: {}", result.x);
        assert!((result.y - 1.0).abs() < epsilon, "Y war: {}", result.y);
        assert!((result.z - 0.0).abs() < epsilon, "Z war: {}", result.z);
    }
}
