use minifb::{Key, Window, WindowOptions};
use std::f32::consts::PI;
// Importiert die mathematischen Strukturen und Algorithmen aus Ihrer TDD-Bibliothek
use drei_d_software_renderer_tdd::{Matrix4D, Mesh, Vector3D, Viewport, bresenham_linie};

// Definition der Fensterauflösung in Pixeln
const BREITE: usize = 800;
const HOEHE: usize = 600;

fn main() {
    println!("#####-----steuerung-----#####");
    println!("RICHTUNGEN -> | w ▲ | a ◀ | s ▼ | d ▶ |");
    println!("ROTATIONEN -> | x | y | z | -> ACHSEN");
    println!("ENTFERNUNG -> | j - entfernt | k - heran |");

    let mut window = Window::new("3D-stendek55", BREITE, HOEHE, WindowOptions::default()).unwrap();

    // 42 (Frames per Second).
    window.set_target_fps(42);

    // Lädt das Würfel-Mesh
    let wuerfel = Mesh::wuerfel();

    // Der Viewport ist für das finale Mapping auf die echten Monitor-Pixel zuständig.
    let viewport = Viewport::neu(BREITE as u32, HOEHE as u32);

    // Erstellt die perspektivische Projektionsmatrix.
    // Parameter: FOV (90° als Bogenmaß), Seitenverhältnis (800/600), Nahe Clipping-Ebene, Ferne Ebene.
    let projektion = Matrix4D::perspektive(PI / 2.0, BREITE as f32 / HOEHE as f32, 0.1, 100.0);

    // Startposition: Perfekt zentriert (X=0, Y=0) und 3 Einheiten nach hinten verschoben
    let mut position = Vector3D::neu(0.0, 0.0, -3.0);

    // Schrittgeschwindigkeit für die Bewegung pro Frame
    let bewegungs_speed = 0.05;

    // Startrotationen im Bogenmaß (0.0 bedeutet unrotiert)
    let mut rot_x_winkel: f32 = 0.0;
    let mut rot_y_winkel: f32 = 0.0;
    let mut rot_z_winkel: f32 = 0.0;

    // Drehgeschwindigkeit pro Frame
    let rotations_speed = 0.05;

    // Erstellt den eigentlichen Pixel-Speicher für das Fenster (800 * 600 = 480.000 u32-Zahlen).
    // Jede Zahl repräsentiert die Farbe eines Pixels im Hex-Format: 0x00RRGGBB.
    let mut framebuffer = vec![0u32; BREITE * HOEHE];

    // Läuft so lange, wie das Fenster offen ist und nicht die ESC-Taste gedrückt wird.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // ---------------------------------------------------------------------
        // A. TASTATUR-INPUT FÜR DIE VIER RICHTUNGEN ABFRAGEN
        // ---------------------------------------------------------------------
        if window.is_key_down(Key::A) {
            position = position.sub(&Vector3D::neu(bewegungs_speed, 0.0, 0.0)); // Links
        }
        if window.is_key_down(Key::D) {
            position = position.add(&Vector3D::neu(bewegungs_speed, 0.0, 0.0)); // Rechts
        }
        if window.is_key_down(Key::W) {
            position = position.add(&Vector3D::neu(0.0, bewegungs_speed, 0.0)); // Oben
        }
        if window.is_key_down(Key::S) {
            position = position.sub(&Vector3D::neu(0.0, bewegungs_speed, 0.0)); // Unten
        }
        if window.is_key_down(Key::Y) {
            rot_y_winkel += rotations_speed; // Dreht den Würfel um die vertikale Y-Achse
        }
        if window.is_key_down(Key::X) {
            rot_x_winkel += rotations_speed; // Neigt den Würfel um die horizontale X-Achse
        }
        if window.is_key_down(Key::Z) {
            rot_z_winkel += rotations_speed;
        }
        if window.is_key_down(Key::J) {
            position = position.sub(&Vector3D::neu(0.0, 0.0, bewegungs_speed * 2.0)); // schiebe in tiefe
        }
        if window.is_key_down(Key::K) && position.z < -1.1 {
            position = position.add(&Vector3D::neu(0.0, 0.0, bewegungs_speed * 2.0)); // komm näher ran
        }

        // ---------------------------------------------------------------------
        // B. WELT-MATRIX AKTUALISIEREN
        // ---------------------------------------------------------------------
        // Erstelle die Rotationsmatrizen aus den aktuellen Winkeln
        let rot_y = Matrix4D::rotation_y(rot_y_winkel);
        let rot_x = Matrix4D::rotation_x(rot_x_winkel);
        let rot_z = Matrix4D::rotation_z(rot_z_winkel);

        // Kombiniere die Rotationen
        let gesamt_rotation = rot_x
            .multipliziere_matrix(&rot_y)
            .multipliziere_matrix(&rot_z);

        // Erstelle die Translation basierend auf der aktuellen Position
        let verschiebung = Matrix4D::translation(&position);

        // Kombiniere beides zur finalen Welt-Matrix
        // WICHTIG: Die Verschiebung steht links. Der Würfel rotiert also erst im Ursprung
        // um sein eigenes Zentrum und wird danach an seine Position im Raum verschoben.
        let welt_matrix = verschiebung.multipliziere_matrix(&gesamt_rotation);

        // ---------------------------------------------------------------------
        // C. GRAPHICS PIPELINE (3D ZU 2D PROJEKTION)
        // ---------------------------------------------------------------------
        let screen_punkte: Vec<(i32, i32)> = wuerfel
            .vertices
            .iter()
            .map(|v| {
                let welt_punkt = welt_matrix.multipliziere_punkt(v);
                let homogen_punkt = projektion.multipliziere_vektor_homogen(&welt_punkt);
                let ndc_punkt = homogen_punkt.zu_ndc_space();
                viewport.transformiere(&ndc_punkt)
            })
            .collect();

        // ---------------------------------------------------------------------
        // D. BRESENHAM-RASTERISIERUNG FÜR JEDEN FRAME BERECHNEN
        // ---------------------------------------------------------------------
        let mut linien_pixel = Vec::new();

        for &(start_idx, end_idx) in &wuerfel.edges {
            let (x0, y0) = screen_punkte[start_idx];
            let (x1, y1) = screen_punkte[end_idx];

            for (px, py) in bresenham_linie(x0, y0, x1, y1) {
                if px >= 0 && px < BREITE as i32 && py >= 0 && py < HOEHE as i32 {
                    let index = (py as usize * BREITE) + px as usize;
                    linien_pixel.push(index);
                }
            }
        }
        // Leert den gesamten Bildschirm. .fill() ist extrem schnell
        framebuffer.fill(0x1A1A1A);

        // ZEICHNEN DES WÜRFELS:
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
