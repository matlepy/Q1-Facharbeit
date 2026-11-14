# Boid-Simulation

Simulation eines Schwarmverhaltens nach dem Boids-Modell (Reynolds, 1986) im Rahmen einer Facharbeit. Untersucht wird, wie natürliche Faktoren – insbesondere **Sichtfeld** und **Sichtradius** – die Dynamik und Struktur des Schwarms beeinflussen.

## Hintergrund

Das klassische Boids-Modell erzeugt Schwarmverhalten aus drei einfachen lokalen Regeln:

1. **Separation** – Abstand zu zu nahen Nachbarn halten
2. **Alignment** – Ausrichtung an der mittleren Bewegungsrichtung der Nachbarn
3. **Cohesion** – Bewegung zum Mittelpunkt der Nachbarn

In dieser Simulation ist die Wahrnehmung jedes Boids eingeschränkt: Ein Boid reagiert nur auf Nachbarn, die innerhalb seines Sichtradius liegen **und** sich in seinem Sichtfeld (Blickwinkel nach vorn) befinden. Durch Variation dieser Parameter lassen sich Übergänge zwischen geordnetem Schwarm, wirbelnden Gruppen und unkoordiniertem Verhalten beobachten.

## Projektstruktur

Der Code auf diesem Branch (`polished_de`) wird **parallel zur Facharbeit in Phasen** weiterentwickelt und (im Gegensatz zu `main`) in Deutsch geschrieben und kommentiert. Die Phasen spiegeln den Aufbau der Facharbeit wider (Grundmodell → Wahrnehmungsparameter → Experimente → Auswertung) und stellen **nicht** den tatsächlichen Entwicklungsprozess dar.

## Ausführung

```bash
# Repository klonen
git clone https://github.com/matlepy/Q1-Facharbeit.git
cd Q1-Facharbeit
git checkout polished_de
cd boids

# Abhängigkeiten installieren und starten
# Kompilierung (optional)
cargo build --release
# Ausführung mit Kompilierung
cargo run --release
```

## Status

- [ ] Basis-Boids-Modell
- [ ] Sichtfeld- und Sichtradius-Parameter
- [ ] Experimente und Datenaufzeichnung
- [ ] Auswertung und Visualisierung

---

*Erstellt im Rahmen der Facharbeit. Letzter Stand: November 2026.*
