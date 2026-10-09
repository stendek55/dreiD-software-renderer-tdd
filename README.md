# ***.....in bearbeitung.....***
---
# 3D Software-Renderer

Ein eigenständiges Übungsprojekt zur Implementierung einer dreidimensionalen Grafik-Engine von Grund auf in Rust. Das Projekt verzichtet konsequent auf externe Mathematik- oder Grafik-Bibliotheken (Zero-Crates-Ansatz), um die zugrundeliegende lineare Algebra, die Rendering-Pipeline und Algorithmen vollständig im Eigenbau zu durchdringen.

## Projektauftrag und Zielsetzung

Das Ziel dieses Projekts ist die Entwicklung eines funktionsfähigen Software-Renderers, der 3D-Drahtgittermodelle (Wireframes) im Raum mathematisch transformiert, rotiert und korrekt perspektivisch auf einen zweidimensionalen Bildschirm projiziert. 

Die Anzeige erfolgt über das Crate `minifb`, welches ausschließlich als primitiver Framebuffer genutzt wird. Das bedeutet, dass sämtliche Berechnungen – vom Setzen einzelner Pixel im Speicher über das Zeichnen von Linien bis hin zu Matrix-Multiplikationen – selbst programmiert werden.

## Technische Kernkomponenten

Das System wird strikt nach der Test-Driven Development (TDD) Methodik aufgebaut und gliedert sich in folgende mathematische und architektonische Bereiche:

1. Vektor- und Matrix-Arithmetik: Implementierung eigener mathematischer Strukturen für 3D-Vektoren (Vector3) und 4x4-Transformationsmatrizen (Matrix4). Dies umfasst die Vektoraddition, Skalierung, das Kreuzprodukt sowie Matrix-Matrix- und Matrix-Vektor-Multiplikationen.
2. Transformations-Pipeline: Berechnung von Rotationsmatrizen (X-, Y- und Z-Achse), Translationsmatrizen (Verschiebung im Raum) und Skalierungsmatrizen zur dynamischen Manipulation von 3D-Objekten.
3. Perspektivische Projektion: Mathematische Transformation von dreidimensionalen Weltkoordinaten in zweidimensionale Bildschirmkoordinaten unter Berücksichtigung des Sichtfeldes (Field of View) und des Seitenverhältnisses (Aspect Ratio).
4. Rasterisierung (Bresenham-Algorithmus): Implementierung eines effizienten, ganzzahlbasierten Algorithmus zur Interpolation und zum Zeichnen von Linien zwischen zwei Punkten direkt in den zweidimensionalen Pixel-Buffer.

## Entwicklungsmethodik (TDD)

Der gesamte mathematische Kern sowie die Projektionslogik werden stufenweise nach dem Prinzip Red-Green-Refactor entwickelt. Da das Projekt ohne Third-Party-Crates auskommt, stellt die lückenlose Test-Suite die absolute mathematische Korrektheit der Rotationen und Projektionen sicher, bevor die Daten an den Framebuffer übergeben werden.

## Geplante Features

- Interaktive Rotation und Verschiebung von geometrischen Primitiven (z. B. Würfel) über die Tastatur.
- Performantes Rendering durch Minimierung von Speicherallokationen innerhalb der Hauptschleife.
- Clean-Code-Architektur zur optionalen späteren Erweiterung um Face-Culling (Ausblenden von Rückseiten) oder Flat-Shading (Ausfüllen von Polygonen).

