# Aufgabe: Implementierung von Hash-Verfahren und Aufzählen der Kollisionen

## Beschreibung Implementierung
Die gegebene implementierung nutzt das einfache Divisions-Rest Verfahren

## Durchgeführte tests:
Die Tabelle in den tests wurde auf die größe 101 gesetzt (Primzahl).

Die Tabelle wurde zu 50, 90, 95 und 100 prozent mit Zufallswerten und Zufallsschlüsseln gefüllt.

(Abgerundet -> 50%: 50 Werte, 90%: 90 Werte, 95%: 95 Werte, 100%: 101 Werte)

Auf die Tabelle wurde 1000 mal mit zufälligen Schlüsseln zugegriffen.
Dabei wurden die Anzahl an Kollisionen, die Anzahl an Erfolgreichen Zugriffen und die Anzahl an Zugriffen auf leere Felder aufgezeichnet.

Dieser Vorgang wurde jeweils 5 mal pro Füllstand wiederholt.

Zusätzlich wird der Durchschnitt berechnet und durch die Gesamtanzahl der Zugriffe geteilt.


## Messung

### 50% Füllstand

| 50%         | 1.  | 2.  | 3.  | 4.  | 5.  |
|-------------|-----|-----|-----|-----|-----|
| Kollisionen | 516 | 480 | 502 | 499 | 498 |
| Matches     | 0   | 0   | 0   | 0   | 0   |
| Leer        | 484 | 520 | 498 | 501 | 502 |

Durchschnittliche Kollisionen: 499

Durchschnittliche Kollisionsrate: 49,90%


### 90% Füllstand

| 90%         | 1.  | 2.  | 3.  | 4.  | 5.  |
|-------------|-----|-----|-----|-----|-----|
| Kollisionen | 893 | 899 | 890 | 904 | 876 |
| Matches     | 0   | 0   | 0   | 0   | 0   |
| Leer        | 107 | 101 | 110 | 96  | 124 |

Durchschnittliche Kollisionen: 892,4


Durchschnittliche Kollisionsrate: 89,24%


### 95% Füllstand

| 95%         | 1.  | 2.  | 3.  | 4.  | 5.  |
|-------------|-----|-----|-----|-----|-----|
| Kollisionen | 946 | 948 | 933 | 948 | 939 |
| Matches     | 0   | 0   | 0   | 0   | 0   |
| Leer        | 54  | 52  | 67  | 52  | 61  |

Durchschnittliche Kollisionen: 942,8

Durchschnittliche Kollisionsrate: 94,28%

### 100% Füllstand

| 100%        | 1.   | 2.   | 3.   | 4.   | 5.   |
|-------------|------|------|------|------|------|
| Kollisionen | 1000 | 1000 | 1000 | 1000 | 1000 |
| Matches     | 0    | 0    | 0    | 0    | 0    |
| Leer        | 0    | 0    | 0    | 0    | 0    |

Durchschnittliche Kollisionen: 1000

## Ergebniss:

Die Kollisionsrate scheint sich durchschnittlich dem jeweiligen Füllstand anzunähern.

## Verwendetes Programm

