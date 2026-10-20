#![allow(dead_code)]

pub struct Mesh {
    pub vertices: Vec<Vector3D>,
    pub edges: Vec<(usize, usize)>,
}

impl Mesh {
    pub fn neu(vertices: Vec<Vector3D>, edges: Vec<(usize, usize)>) -> Self {
        Mesh { vertices, edges }
    }

    /// Erzeugt einen Standard-Würfel mit Kantenlänge 1.0 im Ursprung.
    /// Perfekt als Standard-Geometrie für den Renderer.
    pub fn wuerfel() -> Self {
        let vertices = vec![
            Vector3D::neu(-0.5, -0.5, -0.5), // 0
            Vector3D::neu(0.5, -0.5, -0.5),  // 1
            Vector3D::neu(0.5, 0.5, -0.5),   // 2
            Vector3D::neu(-0.5, 0.5, -0.5),  // 3
            Vector3D::neu(-0.5, -0.5, 0.5),  // 4
            Vector3D::neu(0.5, -0.5, 0.5),   // 5
            Vector3D::neu(0.5, 0.5, 0.5),    // 6
            Vector3D::neu(-0.5, 0.5, 0.5),   // 7
        ];

        let edges = vec![
            (0, 1), // Hintere Seite
            (1, 2), // Hintere Seite
            (2, 3), // Hintere Seite
            (3, 0), // Hintere Seite
            (4, 5), // Vordere Seite
            (5, 6), // Vordere Seite
            (6, 7), // Vordere Seite
            (7, 4), // Vordere Seite
            (0, 4), // Verbindungen
            (1, 5), // Verbindungen
            (2, 6), // Verbindungen
            (3, 7), // Verbindungen
        ];

        Mesh { vertices, edges }
    }
}

#[derive(Debug, PartialEq)]
pub struct Vector3D {
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
//#########-----rechteHandRegel-----###################
// # Das rechtshändige Koordinatensystem (Ausgangslage)
// * **Daumen:** X-Achse (zeigt nach rechts)
// * **Zeigefinger:** Y-Achse (zeigt nach oben)
// * **Mittelfinger:** Z-Achse (zeigt aus dem Bildschirm heraus auf den Betrachter)

#[derive(Debug, PartialEq)]
struct Vector4D {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl Vector4D {
    /// Mathematisch: Perspektivische Division (De-Homogenisierung des Vektors).
    ///
    /// Teilt die räumlichen Koordinaten (x, y, z) komponentenweise durch die homogene
    /// vierte Komponente (w). Dadurch wird der Punkt aus dem vierdimensionalen Clip-Space
    /// in den standardisierten, dreidimensionalen NDC-Raum (Normalized Device Coordinates) überführt.
    ///
    /// # Verwendung im Renderer
    /// Dies ist der entscheidende Schritt zur Erzeugung des 3D-Effekts: Da die `w`-Komponente
    /// zuvor durch die Projektionsmatrix die Tiefeninformation des Punktes (Z-Wert) gespeichert hat,
    /// bewirkt die Division, dass weiter entfernte Objekte automatisch kleiner dargestellt werden.
    /// Nach diesem Schritt liegen alle sichtbaren Punkte in einem Wertebereich von `-1.0` bis `1.0`.
    ///
    /// # Mathematische Formel
    /// v_ndc = (x / w, y / w, z / w)
    ///
    /// # Panics / Edge Cases
    /// Wenn die homogene Komponente `w` exakt `0.0` beträgt (Punkt befindet sich im Ursprung
    /// der Kameraebene), wird eine Division durch Null verhindert und stattdessen ein
    /// sicherer Nullvektor `Vector3D::neu(0.0, 0.0, 0.0)` zurückgegeben.
    fn zu_ndc_space(&self) -> Vector3D {
        let tiefenwert_w = self.w;

        if tiefenwert_w == 0.0 {
            Vector3D::neu(0.0, 0.0, 0.0)
        } else {
            Vector3D {
                x: self.x / tiefenwert_w,
                y: self.y / tiefenwert_w,
                z: self.z / tiefenwert_w,
            }
        }
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

    /// Mathematisch: Erstellung einer Rotationsmatrix um die Y-Achse.
    ///
    /// Dreht alle Punkte in der X-Z-Ebene um den Ursprung. Die Y-Koordinate bleibt dabei unverändert.
    /// Die Berechnung nutzt die trigonometrischen Funktionen `cos` und `sin` des Winkels.
    ///
    /// # Verwendung im Renderer
    /// Entspricht der Yaw-Bewegung. Dreht ein Objekt oder die Kamera nach links bzw. rechts
    /// (Gieren), wenn man von oben auf die X-Z-Ebene blickt.
    ///
    /// # Winkelmaß
    /// Erwartet den Winkel im **Bogenmaß** (Radians). Ein Vollkreis entspricht 2 * PI.
    fn rotation_y(rot_winkel: f32) -> Self {
        let mut mat = Matrix4D::identitaet();
        let cos = rot_winkel.cos();
        let sin = rot_winkel.sin();

        mat.m[0][0] = cos;
        mat.m[0][2] = sin;
        mat.m[2][0] = -sin;
        mat.m[2][2] = cos;

        mat
    }

    /// Mathematisch: Erstellung einer Rotationsmatrix um die X-Achse.
    ///
    /// Dreht alle Punkte in der Y-Z-Ebene um den Ursprung. Die X-Koordinate bleibt dabei unverändert.
    /// Die Berechnung nutzt die trigonometrischen Funktionen `cos` und `sin` des Winkels.
    ///
    /// # Verwendung im Renderer
    /// Entspricht der Pitch-Bewegung. Neigt ein Objekt oder die Kamera nach oben bzw. unten,
    /// wenn man von der Seite auf das Koordinatensystem blickt.
    ///
    /// # Winkelmaß
    /// Erwartet den Winkel im **Bogenmaß** (Radians). Ein Vollkreis entspricht 2 * PI.
    fn rotation_x(rot_winkel: f32) -> Self {
        let mut mat = Matrix4D::identitaet();
        let cos = rot_winkel.cos();
        let sin = rot_winkel.sin();

        mat.m[1][1] = cos;
        mat.m[1][2] = -sin;
        mat.m[2][1] = sin;
        mat.m[2][2] = cos;

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

    /// Mathematisch: Multiplikation der Matrix mit einem 3D-Punkt in homogenen Koordinaten (w = 1.0).
    ///
    /// Berechnet die vollständige Transformation inklusive der vierten Zeile der Matrix.
    /// Im Gegensatz zur Standard-Punktmultiplikation wird die homogene Koordinate `w` nicht verworfen,
    /// sondern als eigenständiger Wert im Ergebnis-Vektor berechnet.
    ///
    /// # Verwendung im Renderer
    /// Dies ist der essenzielle Schritt für die **perspektivische Projektion**. Die Matrix codiert
    /// hierbei die Tiefeninformation des Punktes in die `w`-Komponente des resultierenden `Vector4D`.
    /// Dieser Wert wird im nächsten Schritt der Pipeline für die perspektivische Division benötigt.
    ///
    /// # Mathematischer Hintergrund (Homogene Koordinaten)
    /// Der eingehende `Vector3D` wird virtuell um die Komponente `w = 1.0` erweitert. Das Ergebnis
    /// bildet die exakte Zeile-mal-Spalte-Multiplikation der 4x4-Matrix ab:
    /// * `x`, `y`, `z` fangen Translation, Rotation und Skalierung ein.
    /// * `w` berechnet sich aus der vierten Zeile und speichert bei Projektionsmatrizen die relative Z-Tiefe.
    fn multipliziere_vektor_homogen(&self, punkt: &Vector3D) -> Vector4D {
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
        let w = (self.m[3][0] * punkt.x)
            + (self.m[3][1] * punkt.y)
            + (self.m[3][2] * punkt.z)
            + self.m[3][3];

        Vector4D { x, y, z, w }
    }

    /// Mathematisch: Erstellung einer standardisierten perspektivischen Projektionsmatrix.
    ///
    /// # Parameter
    /// * `fov_rad`: Das vertikale Sichtfeld (Field of View) im Bogenmaß (Radians).
    /// * `aspect`: Das Seitenverhältnis des Bildschirms (Breite / Höhe).
    /// * `near`: Der Abstand vom Auge zur nahen Clipping-Ebene (Sichtgrenze vorne).
    /// * `far`: Der Abstand vom Auge zur fernen Clipping-Ebene (Sichtgrenze hinten).
    ///
    /// # Verwendung im Renderer
    /// Bildet die dreidimensionale Pyramide des Kamerasichtfelds (Frustum o Sichtstumpf) auf einen
    /// standardisierten, würfelförmigen Raum ab (Normalized Device Coordinates / NDC, von -1.0 bis 1.0).
    /// Sie verzerrt den Raum so, dass weiter entfernte Objekte nach der Division durch `w` kleiner werden.
    fn perspektive(fov_rad: f32, aspect: f32, near: f32, far: f32) -> Self {
        let mut mat = Matrix4D::nuller();

        // Berechne den Skalierungsfaktor basierend auf dem Sichtfeld
        let tan_half_fov = (fov_rad / 2.0).tan();

        // Erste Zeile: Skalierung für X (inklusive Seitenverhältnis)
        mat.m[0][0] = 1.0 / (aspect * tan_half_fov);

        // Zweite Zeile: Skalierung für Y
        mat.m[1][1] = 1.0 / tan_half_fov;

        // Dritte Zeile: Transformation und Mapping der Z-Tiefe
        mat.m[2][2] = -(far + near) / (far - near);
        mat.m[2][3] = -(2.0 * far * near) / (far - near);

        // Vierte Zeile: Speicher die Z-Tiefe im homogenen W-Wert ab (w = -z)
        mat.m[3][2] = -1.0;

        mat
    }
}

struct Viewport {
    breite: u32,
    hoehe: u32,
}

impl Viewport {
    fn neu(breite: u32, hoehe: u32) -> Self {
        Viewport { breite, hoehe }
    }

    /// Mathematisch: Viewport-Transformation (Skalierung und Translation in den Screen-Space).
    ///
    /// Rechnet die normalisierten dreidimensionalen Raumkoordinaten (NDC, von -1.0 bis 1.0)
    /// in zweidimensionale, ganzzahlige Pixel-Koordinaten des Fensters um.
    ///
    /// # Verwendung im Renderer
    /// Diese Methode schließt die 3D-Transformationskette ab. Sie mappt das mathematische
    /// Sichtfenster auf die realen Pixelabmessungen deines `minifb`-Fensters. Da der Nullpunkt
    /// auf Bildschirmen oben links liegt und die Y-Achse nach unten verläuft, wird die
    /// vertikale Y-Koordinate bei der Berechnung invertiert.
    ///
    /// # Mathematische Formel
    /// pixel_x = (x_ndc + 1.0) * 0.5 * bildschirm_breite
    /// pixel_y = (1.0 - y_ndc) * 0.5 * bildschirm_hoehe
    ///
    /// # Edge Cases
    /// Die Ergebnisse werden mittels `as i32` gecastet. Punkte außerhalb des sichtbaren
    /// Bereichs (NDC < -1.0 oder > 1.0) erzeugen planmäßig negative Pixelkoordinaten oder
    /// Werte größer als die Fensterauflösung, welche im späteren Line-Clipper abgefangen werden.
    fn transformiere(&self, ndc_pkt: &Vector3D) -> (i32, i32) {
        let x_pxl = ((ndc_pkt.x + 1.0) * 0.5 * self.breite as f32) as i32;
        let y_pxl = ((1.0 - ndc_pkt.y) * 0.5 * self.hoehe as f32) as i32;

        (x_pxl, y_pxl)
    }
}

/// Mathematisch: Rasterung einer idealen mathematischen Linie auf einem diskreten Pixelraster.
///
/// Implementiert den verallgemeinerten Bresenham-Linienalgorithmus, der Linien in alle
/// 8 Oktanten (flach, steil, steigend, fallend, nach links oder nach rechts) fehlerfrei zeichnet.
/// Der Algorithmus arbeitet vollständig ohne Fließkommazahlen (Floats) und Divisionen,
/// was ihn extrem effizient macht.
///
/// # Funktionsweise & mathematischer Hintergrund
/// Anstatt für jedes Pixel die Geradengleichung `y = m * x + b` mit langsamen Fließkommawerten
/// zu berechnen und zu runden, nutzt der Algorithmus eine reine Ganzzahl-Fehlervariable (`fehler_0`).
/// Diese misst den kumulierten Abstand zur idealen mathematischen Linie.
/// Durch die interne Multiplikation der Gleichung mit `2 * dx` bzw. `2 * dy` werden alle Brüche
/// eliminiert. Die Multiplikation `2 * fehler_0` wird vom Prozessor hardwareseitig über eine
/// extrem schnelle Bit-Verschiebung (Bit-Shift) gelöst.
///
/// # Parameter
/// * `x_start`: Die horizontale Startkoordinate im Screen-Space.
/// * `y_start`: Die vertikale Startkoordinate im Screen-Space.
/// * `x_end`: Die horizontale Endkoordinate im Screen-Space.
/// * `y_end`: Die vertikale Endkoordinate im Screen-Space.
///
/// # Rückgabewert
/// Gibt einen Vektor (`Vec<(i32, i32)>`) zurück, der sequenziell alle berechneten Pixelkoordinaten
/// vom Startpunkt bis zum Endpunkt (jeweils inklusive) enthält.
///
/// # Edge Cases (Abgedeckte Spezialfälle)
/// * **Horizontale Linien:** `dy == 0`. Der Algorithmus läuft stabil ohne Division-by-Zero-Risiko.
/// * **Vertikale Linien:** `dx == 0`. Ebenfalls stabil, es wird nur auf der Y-Achse geschritten.
/// * **Rückwärts-Linien:** Wenn der Endpunkt links vom Startpunkt liegt (`x_start > x_end`),
///   wird die Schrittrichtung automatisch invertiert (`sx = -1`).
/// * **Identische Punkte:** Wenn Start- und Endpunkt übereinstimmen, wird ein einzelnes Pixel zurückgegeben.
fn bresenham_linie(x_start: i32, y_start: i32, x_end: i32, y_end: i32) -> Vec<(i32, i32)> {
    let mut punkte = Vec::new();

    //abstände berechnen
    let dx = (x_start - x_end).abs();
    let dy = (y_start - y_end).abs();

    //schrittrichtung bestimmen (vorzeichen)
    let sx = if x_start < x_end { 1 } else { -1 };
    let sy = if y_start < y_end { 1 } else { -1 };

    //startfehler
    let mut fehler_0 = dx - dy;

    //startpunkte
    let mut x = x_start;
    let mut y = y_start;

    loop {
        //aktuelles pixel setzen
        punkte.push((x, y));

        //wenn ziel erreicht dann abbruch
        if x == x_end && y == y_end {
            break;
        }

        let fehler_1 = 2 * fehler_0;

        //schritt in x-richtung
        if fehler_1 > -dy {
            fehler_0 -= dy;
            x += sx;
        }

        //schritt in y-richtung
        if fehler_1 < dx {
            fehler_0 += dx;
            y += sy;
        }
    }

    punkte
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

    use std::f32::consts::PI;
    const EPSILON: f32 = 1e-5; // = 0.00001 -> differenz zum ergebnis noch ok
    #[test]
    fn test_matrix_rotation_z_achse() {
        // Given: Eine Rotation um 90 Grad (PI / 2 Bogenmaß) um die Z-Achse
        let winkel = PI / 2.0;
        let m = Matrix4D::rotation_z(winkel);

        // Ein Punkt auf der X-Achse (1, 0, 0)
        let punkt = Vector3D::neu(1.0, 0.0, 0.0);

        // When: Wir den Punkt rotieren
        let result = m.multipliziere_punkt(&punkt);

        // Then: Dreht sich der Punkt im Gegenuhrzeigersinn auf die Y-Achse (0, 1, 0)
        // Hinweis: Da f32 Ungenauigkeiten besitzt, prüfen wir mit einer kleinen Toleranz (Epsilon)
        assert!((result.x - 0.0).abs() < EPSILON, "X war: {}", result.x);
        assert!((result.y - 1.0).abs() < EPSILON, "Y war: {}", result.y);
        assert!((result.z - 0.0).abs() < EPSILON, "Z war: {}", result.z);
    }

    #[test]
    fn test_matrix_rotation_y_achse() {
        // Given: Eine Rotation um 90 Grad (PI / 2 Bogenmaß) um die Y-Achse
        let winkel = PI / 2.0;
        let m = Matrix4D::rotation_y(winkel);

        // Ein Punkt auf der X-Achse (1, 0, 0)
        let punkt = Vector3D::neu(1.0, 0.0, 0.0);

        // When: Wir den Punkt rotieren
        let result = m.multipliziere_punkt(&punkt);

        // Then: Dreht sich der Punkt im Gegenuhrzeigersinn auf die Z-Achse (0, 0, -1)
        // Hinweis: Da f32 Ungenauigkeiten besitzt, prüfen wir mit einer kleinen Toleranz (Epsilon)
        assert!((result.x - 0.0).abs() < EPSILON, "X war: {}", result.x);
        assert!((result.y - 0.0).abs() < EPSILON, "Y war: {}", result.y);
        assert!((result.z + 1.0).abs() < EPSILON, "Z war: {}", result.z);
    }

    #[test]
    fn test_matrix_rotation_x_achse() {
        // Given: Eine Rotation um 90 Grad (PI / 2 Bogenmaß) um die X-Achse
        let winkel = PI / 2.0;
        let m = Matrix4D::rotation_x(winkel);

        // Ein Punkt auf der Y-Achse (0, 1, 0)
        let punkt = Vector3D::neu(0.0, 1.0, 0.0);

        // When: Wir den Punkt rotieren
        let result = m.multipliziere_punkt(&punkt);

        // Then: Dreht sich der Punkt im Gegenuhrzeigersinn auf die Z-Achse (0, 0, 1)
        // Hinweis: Da f32 Ungenauigkeiten besitzt, prüfen wir mit einer kleinen Toleranz (Epsilon)
        assert!((result.x - 0.0).abs() < EPSILON, "X war: {}", result.x);
        assert!((result.y - 0.0).abs() < EPSILON, "Y war: {}", result.y);
        assert!((result.z - 1.0).abs() < EPSILON, "Z war: {}", result.z);
    }

    #[test]
    fn test_matrix_multiplikation_vektor_homogen_vollstaendig() {
        // Given: Eine Matrix mit eindeutig identifizierbaren Werten für jede Position
        // Zeile 0: [ 1,  2,  3,  4]
        // Zeile 1: [ 5,  6,  7,  8]
        // Zeile 2: [ 9, 10, 11, 12]
        // Zeile 3: [13, 14, 15, 16]
        let mut test_matrix = Matrix4D::nuller();
        test_matrix.m = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0],
        ];

        // Ein Vektor mit eindeutigen Koordinaten (w wird implizit als 1.0 angenommen)
        let punkt = Vector3D::neu(2.0, 3.0, 4.0);

        // When: Wir den Vektor homogen multiplizieren
        let result = test_matrix.multipliziere_vektor_homogen(&punkt);

        // Then: Erwarten wir die mathematisch exakten Skalarprodukte pro Zeile:
        // x = (1*2)  + (2*3)  + (3*4)  + (4*1)  = 2  + 6  + 12 + 4  = 24
        // y = (5*2)  + (6*3)  + (7*4)  + (8*1)  = 10 + 18 + 28 + 8  = 64
        // z = (9*2)  + (10*3) + (11*4) + (12*1) = 18 + 30 + 44 + 12 = 104
        // w = (13*2) + (14*3) + (15*4) + (16*1) = 26 + 42 + 60 + 16 = 144

        let expected = Vector4D {
            x: 24.0,
            y: 64.0,
            z: 104.0,
            w: 144.0,
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn test_projektionsmatrix_nullpunkt() {
        // Given: Eine Kamera mit 90° FOV, quadratischem Bildschirm (1:1)
        let fov = PI / 2.0;
        let aspect = 1.0;
        let near = 0.1;
        let far = 100.0;
        let proj_matrix = Matrix4D::perspektive(fov, aspect, near, far);

        // Ein Punkt liegt exakt auf der Blickachse (Z-Achse)
        // Hinweis: Da deine Kamera standardmäßig in den Raum blickt, testen wir mit Z = -5.0
        let punkt_auf_achse = Vector3D::neu(0.0, 0.0, -5.0);

        // When: Der Punkt projiziert wird
        let res = proj_matrix.multipliziere_vektor_homogen(&punkt_auf_achse);

        // Then: Müssen X und Y im NDC-Raum exakt zentriert bei 0.0 bleiben
        assert!(
            (res.x - 0.0).abs() < EPSILON,
            "X-Projektion abgewichen: {}",
            res.x
        );
        assert!(
            (res.y - 0.0).abs() < EPSILON,
            "Y-Projektion abgewichen: {}",
            res.y
        );
    }

    #[test]
    fn test_projektionsmatrix_seitenverhaeltnis_skalierung() {
        // Given: Ein typischer Breitbild-Monitor (16:9)
        let fov = PI / 3.0; // 60 Grad
        let aspect = 16.0 / 9.0;
        let proj_matrix = Matrix4D::perspektive(fov, aspect, 0.1, 100.0);

        // Ein perfekt symmetrischer Punkt im Raum
        let punkt = Vector3D::neu(2.0, 2.0, -5.0);

        // When: Wir den Punkt projizieren
        let res = proj_matrix.multipliziere_vektor_homogen(&punkt);

        // Then: Da der Bildschirm breiter als hoch ist, müssen die Koordinaten horizontal (X)
        // stärker gestaucht werden als vertikal (Y), um ein "Breitgezerrtes" Bild zu verhindern.
        assert!(
            res.x.abs() < res.y.abs(),
            "X-Wert ({}) wurde nicht stärker gestaucht als Y-Wert ({})",
            res.x,
            res.y
        );
    }

    #[test]
    fn test_projektionsmatrix_fov_einfluss() {
        // Given: Ein Punkt im Raum und zwei unterschiedliche Sichtfelder
        let punkt = Vector3D::neu(1.0, 1.0, -5.0);

        let fov_eng = PI / 4.0; // 45 Grad (Zoom)
        let fov_weit = PI / 2.0; // 90 Grad (Weitwinkel)

        let mat_eng = Matrix4D::perspektive(fov_eng, 1.0, 0.1, 100.0);
        let mat_weit = Matrix4D::perspektive(fov_weit, 1.0, 0.1, 100.0);

        // When: Beide Matrizen denselben Punkt verarbeiten
        let res_eng = mat_eng.multipliziere_vektor_homogen(&punkt);
        let res_weit = mat_weit.multipliziere_vektor_homogen(&punkt);

        // Then: Im Weitwinkel sieht man mehr von der Welt, Objekte rücken
        // optisch weiter in die Mitte (Koordinaten im NDC-Raum müssen kleiner sein).
        assert!(
            res_weit.x.abs() < res_eng.x.abs(),
            "Weitwinkel X ({}) ist nicht kleiner als Zoom X ({})",
            res_weit.x,
            res_eng.x
        );
        assert!(
            res_weit.y.abs() < res_eng.y.abs(),
            "Weitwinkel Y ({}) ist nicht kleiner als Zoom Y ({})",
            res_weit.y,
            res_eng.y
        );
    }

    #[test]
    fn test_projektionsmatrix_w_komponente() {
        // Given: Eine perspektivische Matrix und ein Punkt mit Tiefe (Z = -10.0)
        let z_tiefe = -10.0;
        let punkt = Vector3D::neu(1.0, 1.0, z_tiefe);
        let proj_matrix = Matrix4D::perspektive(PI / 2.0, 1.0, 0.1, 100.0);

        // When: Der Punkt transformiert wird
        let res = proj_matrix.multipliziere_vektor_homogen(&punkt);

        // Then: Die w-Komponente darf NICHT 1.0 bleiben. Sie muss den invertierten
        // originalen Z-Wert beinhalten, damit die spätere Division die Perspektive erzeugt.
        assert_ne!(
            res.w, 1.0,
            "w-Komponente wurde nicht modifiziert. Perspektive wird fehlschlagen."
        );
        assert!(
            (res.w - z_tiefe.abs()).abs() < EPSILON,
            "w-Komponente hält nicht die Tiefe. Erwartet: {}, War: {}",
            z_tiefe.abs(),
            res.w
        );
    }

    #[test]
    fn test_vector4d_zu_ndc_vektor3d() {
        // Given: Ein Vector4D aus der Projektionsmatrix (w hält die Tiefe)
        let homogener_punkt = Vector4D {
            x: 2.5,
            y: -1.25,
            z: 3.0,
            w: 5.0,
        };

        // When: den punkt in ndc-raum wandeln
        let ndc_punkt = homogener_punkt.zu_ndc_space();

        // Then: Erwarten wir einen Vector3D, bei dem alle Komponenten durch w geteilt wurden
        // 2.5 / 5.0 = 0.5
        // -1.25 / 5.0 = -0.25
        // 3.0 / 5.0 = 0.6
        let expected = Vector3D::neu(0.5, -0.25, 0.6);
        assert_eq!(ndc_punkt, expected);
    }

    #[test]
    fn test_viewport_transformation_auf_pixel_koordinaten() {
        // Given: Ein Viewport mit einer typischen Auflösung (z. B. 800x600 Pixel)
        let breite = 800;
        let hoehe = 600;
        let viewport = Viewport::neu(breite, hoehe);

        // Und Given: Ein Punkt im NDC-Raum (-1.0 bis 1.0)
        // x = 0.5 (im rechten Viertel des Bildschirms)
        // y = -0.25 (im NDC-Raum leicht unter der Mitte)
        let ndc_punkt = Vector3D::neu(0.5, -0.25, 0.0);

        // When: Wir den Punkt auf Pixel-Koordinaten transformieren
        let (pixel_x, pixel_y) = viewport.transformiere(&ndc_punkt);

        // Then: Erwarten wir die korrekte Pixel-Position auf dem Bildschirm
        // X-Berechnung: (0.5 + 1.0) * 0.5 * 800 = 1.5 * 400 = 600
        // Y-Berechnung: In der Computergrafik zeigt Y auf dem Bildschirm nach UNTEN.
        // Ein negativer NDC-Y-Wert wandert also in die untere Bildschirmhälfte:
        // (1.0 - (-0.25)) * 0.5 * 600 = 1.25 * 300 = 375
        assert_eq!(pixel_x, 600);
        assert_eq!(pixel_y, 375);
    }

    #[test]
    fn test_bresenham_horizontale_linie_links_nach_rechts() {
        // Zeichne Linie von (10, 20) nach (15, 20)
        let punkte = bresenham_linie(10, 20, 15, 20);
        let erwartet = vec![(10, 20), (11, 20), (12, 20), (13, 20), (14, 20), (15, 20)];
        assert_eq!(punkte, erwartet);
    }

    #[test]
    fn test_bresenham_vertikale_linie_unten_nach_oben() {
        // Zeichne Linie von (5, 5) nach (5, 8)
        let punkte = bresenham_linie(5, 5, 5, 8);
        let erwartet = vec![(5, 5), (5, 6), (5, 7), (5, 8)];
        assert_eq!(punkte, erwartet);
    }

    #[test]
    fn test_bresenham_diagonale_linie_steil() {
        // Teste eine steile Diagonale (Anstieg > 1), um zu prüfen, ob die Achsen-Spiegelung funktioniert
        let punkte = bresenham_linie(0, 0, 2, 5);
        // Der Algorithmus muss für jedes Y genau ein Pixel setzen
        assert_eq!(punkte.len(), 6); // Start bis Ende = 6 Pixel
        assert_eq!(punkte[0], (0, 0));
        assert_eq!(punkte[5], (2, 5));
    }

    #[test]
    fn test_mesh_wuerfel_erzeugung() {
        let wuerfel = Mesh::wuerfel();
        assert_eq!(wuerfel.vertices.len(), 8);
        assert_eq!(wuerfel.edges.len(), 12);
    }
}
