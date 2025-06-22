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
```rust
use std::array::from_fn;

use rand::RngCore;


fn main() {
    let mut rand = rand::rng();
    let mut hash_table = HashTable::new_empty();

    // fill hash_table by fill_level*100 percent
    let fill_level = 0.95;
    let fill_amount = (TABLE_SIZE as f64 * fill_level) as usize;
    println!("Filling {} of {} fields.", fill_amount, TABLE_SIZE);

    let mut cnt = 0;
    while hash_table.get_filled_count() < fill_amount { // we probably already have some collisions so we need to check if it really filled it up
        let key = rand.next_u32();
        let value = cnt;

        hash_table.insert_key(key, value);

        cnt += 1;
    }

    //println!("Table: {:?}", hash_table);
    println!("Filled Fields: {}\n", hash_table.get_filled_count());

    // acces the hash table and count collisions
    let access_cnt = 1000;
    let mut collisions = 0;
    let mut matches = 0; // amount of random key guesses (key exists in table)
    let mut empty = 0;

    for _ in 0..access_cnt {
        let key = rand.next_u32();

        let result = hash_table.get(key);

        match result {
            None => empty += 1,
            Some(field) => if field.key == key { matches += 1 } else { collisions += 1 }
        }
    }

    println!("Accessed table {access_cnt} times:\nCollisions: {}\nMatches: {}\nEmpty: {}", collisions, matches, empty);
}

const TABLE_SIZE: usize = 101;

#[derive(Debug)]
struct TableField {
    key: u32,
    value: i32,
}

#[derive(Debug)]
struct HashTable {
    fields: [Option<TableField>; TABLE_SIZE],
}

impl HashTable {
    fn insert_key(&mut self, key: u32, value: i32) {
        self.fields[Self::hash(key)] = Some(TableField {
            key,
            value
        });
    }

    fn get(&self, key: u32) -> &Option<TableField> {
        &self.fields[Self::hash(key)]
    }
    
    fn hash(key: u32) -> usize {
        get_mod_hash(key) // call desired hash function
    }
    
    fn new_empty() -> Self {
        Self { fields: from_fn(|_| None ) }
    }

    // counts the fields that are empty
    fn get_filled_count(&self) -> usize {
        let mut filled_counter: usize = 0;
        self.fields.iter().for_each(|v| {
            if v.is_some() {
                filled_counter += 1;
            }
        });

        filled_counter
    }

}

// simple modulo hash function
fn get_mod_hash(key: u32) -> usize {
    let hash = key % (TABLE_SIZE as u32);
    hash as usize
}
```

