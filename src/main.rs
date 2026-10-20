use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
// Importiert die mathematischen Strukturen und Algorithmen aus Ihrer TDD-Bibliothek
use drei_d_software_renderer_tdd::{Matrix4D, Mesh, Vector3D, Viewport, bresenham_linie};

// Definition der Fensterauflösung in Pixeln
const BREITE: usize = 800;
const HOEHE: usize = 600;

fn main() {
    // Erstellt ein neues Betriebssystem-Fenster über minifb.
    // unwrap() fängt Fehler ab, falls der Grafiktreiber das Fenster nicht öffnen kann.
    let mut window = Window::new("3D Renderer", BREITE, HOEHE, WindowOptions::default()).unwrap();

    // Begrenzt die Hauptschleife auf maximal 60 Durchläufe pro Sekunde (Frames per Second).
    // Das verhindert, dass die CPU unbemerkt auf 100% Last läuft und Strom verschwendet.
    window.set_target_fps(60);

    // =========================================================================
    // 1. EINMALIGE INITIALISIERUNG DER 3D-GEOMETRIE & KAMERA (VOR DER SCHLEIFE)
    // =========================================================================

    // Lädt das Würfel-Mesh (8 Eckpunkte im Raum, 12 Verbindungs-Kanten) aus Ihrer lib.rs.
    let wuerfel = Mesh::wuerfel();

    // Der Viewport ist für das finale Mapping auf die echten Monitor-Pixel zuständig.
    let viewport = Viewport::neu(BREITE as u32, HOEHE as u32);

    // Erstellt die perspektivische Projektionsmatrix.
    // Parameter: FOV (90° als Bogenmaß), Seitenverhältnis (800/600), Nahe Clipping-Ebene, Ferne Ebene.
    // WARUM: Sie sorgt dafür, dass entfernte Objekte kleiner wirken (Z-Achse verzerrt den Raum).
    let projektion = Matrix4D::perspektive(PI / 2.0, BREITE as f32 / HOEHE as f32, 0.1, 100.0);

    // ERSTELLUNG DER WELT-MATRIX (Kombination aus Rotation und Translation):
    // Da wir den Würfel schräg sehen wollen, rotieren wir ihn einmalig um 45 Grad (0.785 Radians)
    // um die X- und Y-Achse. Danach verschieben wir ihn um 3 Einheiten nach HINTEN auf der Z-Achse (translation).
    // WARUM: Die Kamera sitzt mathematisch im Ursprung (0,0,0). Ohne die Translation nach -3.0
    // würde der Würfel mitten im Auge der Kamera stecken und weggeschnitten (geclippt) werden.
    /*
    let welt_matrix = Matrix4D::translation(&Vector3D::neu(0.0, 0.0, -3.0)).multipliziere_matrix(
        &Matrix4D::rotation_x(0.785).multipliziere_matrix(&Matrix4D::rotation_y(0.785)),
    );
    */
    // Schritt A: Erstelle die Rotation um die Y-Achse (45 Grad)
    let rot_y = Matrix4D::rotation_y(0.785);

    // Schritt B: Erstelle die Rotation um die X-Achse (45 Grad)
    let rot_x = Matrix4D::rotation_x(0.785);

    // Schritt C: Kombiniere beide Rotationen zu einer einzigen Rotationsmatrix.
    // Mathematisch: Erst wird Y rotiert, dann X.
    let gesamt_rotation = rot_x.multipliziere_matrix(&rot_y);

    // Schritt D: Erstelle die Translationsmatrix für den Abstand nach hinten (Z = -3.0)
    let verschiebung = Matrix4D::translation(&Vector3D::neu(0.0, 0.0, -3.0));

    // Schritt E: Kombiniere die Verschiebung mit der Rotation zur finalen Welt-Matrix.
    // WICHTIG: Die Verschiebung steht LINKS. Das bedeutet für die Pipeline:
    // Der Würfel wird ERST im Ursprung rotiert und DANACH als fertiges Objekt nach hinten verschoben.
    let welt_matrix = verschiebung.multipliziere_matrix(&gesamt_rotation);

    // =========================================================================
    // 2. DIE GRAPHICS PIPELINE: TRANSFORMATION VON DREIDIMENSIONAL ZU ZWEIDIMENSIONAL
    // =========================================================================

    // .map() durchläuft jeden 3D-Eckpunkt (Vertex) des Würfels und schickt ihn durch die Pipeline.
    let screen_punkte: Vec<(i32, i32)> = wuerfel
        .vertices
        .iter()
        .map(|v| {
            // Schritt A: Multiplikation mit der Welt-Matrix. Transformiert den Punkt vom lokalen
            // Objekt-Raum in den globalen Welt-Raum (rotiert und verschiebt ihn).
            let welt_punkt = welt_matrix.multipliziere_punkt(v);

            // Schritt B: Multiplikation mit der Projektionsmatrix. Überführt den Punkt in den
            // 4D-Clip-Space. Die originale Z-Tiefe wird in der homogenen 'w'-Komponente zwischengespeichert.
            let homogen_punkt = projektion.multipliziere_vektor_homogen(&welt_punkt);

            // Schritt C: Perspektivische Division (.zu_ndc_space()). Teilt X, Y und Z durch 'w'.
            // WARUM: Das ist der mathematische Kern des 3D-Effekts! Je größer das 'w' (die Tiefe),
            // desto kleiner werden die resultierenden X- und Y-Koordinaten (Wertebereich jetzt -1.0 bis 1.0).
            let ndc_punkt = homogen_punkt.zu_ndc_space();

            // Schritt D: Viewport-Transformation. Rechnet die mathematischen NDC-Koordinaten
            // in reale, ganzzahlige Pixel-Koordinaten (z.B. X: 0-800, Y: 0-600) für das Fenster um.
            viewport.transformiere(&ndc_punkt)
        })
        .collect(); // Speichert alle 8 fertig umgerechneten 2D-Bildschirmpunkte ab.

    // =========================================================================
    // 3. RASTERISIERUNG & PRE-CALCULATED CLIPPING (LINIEN BERECHNEN)
    // =========================================================================

    // In diesem Vektor speichern wir die finalen, eindimensionalen Array-Indizes des Bildschirms.
    let mut linien_pixel = Vec::new();

    // Durchläuft alle 12 Kanten des Würfels. Jede Kante kennt die Indizes ihrer zwei Eckpunkte.
    for &(start_idx, end_idx) in &wuerfel.edges {
        // Holt die zugehörigen, vorab berechneten 2D-Pixelkoordinaten aus dem 'screen_punkte'-Vektor.
        let (x0, y0) = screen_punkte[start_idx];
        let (x1, y1) = screen_punkte[end_idx];

        // Der Bresenham-Algorithmus interpoliert eine ideale Linie schrittweise auf das Pixelraster.
        for (px, py) in bresenham_linie(x0, y0, x1, y1) {
            // SICHERHEITS-CHECK (2D-Clipping):
            // WARUM: Wenn eine Linie auch nur 1 Pixel außerhalb des Fensters (z.B. px = -1) zeichnen will,
            // würde die Array-Berechnung abstürzen (IndexOutOfBounds Panic).
            // Hier filtern wir ungültige Koordinaten radikal aus, bevor sie Schaden anrichten können.
            if px >= 0 && px < BREITE as i32 && py >= 0 && py < HOEHE as i32 {
                // Rechnet die 2D-Koordinate (X, Y) in den flachen, eindimensionalen Index des Framebuffers um.
                // Formel: (Zeile * Gesamtbreite) + Spalte
                let index = (py as usize * BREITE) + px as usize;
                linien_pixel.push(index);
            }
        }
    }

    // Erstellt den eigentlichen Pixel-Speicher für das Fenster (800 * 600 = 480.000 u32-Zahlen).
    // Jede Zahl repräsentiert die Farbe eines Pixels im Hex-Format: 0x00RRGGBB.
    let mut framebuffer = vec![0u32; BREITE * HOEHE];

    // =========================================================================
    // 4. MINIMALE HAUPTSCHLEIFE (MAIN-LOOP)
    // =========================================================================

    // Läuft so lange, wie das Fenster offen ist und nicht die ESC-Taste gedrückt wird.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Leert den gesamten Bildschirm. .fill() ist extrem schnell, da es optimierte
        // Prozessor-Befehle nutzt, um den gesamten RAM-Block auf einmal mit der Farbe zu beschreiben.
        // Farbcode: 0x1A1A1A (Ein sehr dunkles Anthrazit-Grau für den Hintergrund).
        framebuffer.fill(0x1A1A1A);

        // ZEICHNEN DES WÜRFELS:
        // Da wir alle betroffenen Pixel-Indizes bereits vorab in 'linien_pixel' berechnet haben,
        // müssen wir hier keine Mathematik, keine Matrizen und keine Schleifen mehr berechnen.
        // Wir springen direkt an die Speicheradressen und setzen die Pixelfarbe.
        // Farbcode: 0x00FF00 (Pures, leuchtendes Grün für die Linien).
        for &index in &linien_pixel {
            framebuffer[index] = 0x00FF00;
        }

        // Kopiert den fertigen Pixel-Buffer aus dem Arbeitsspeicher direkt auf die Grafikkarte,
        // damit das Betriebssystem das Fenster aktualisiert und das Bild auf dem Monitor anzeigt.
        window
            .update_with_buffer(&framebuffer, BREITE, HOEHE)
            .unwrap();
    }
}
