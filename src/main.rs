use std::array::from_fn;

use rand::RngCore;

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

fn main() {
    let mut rand = rand::rng();
    let mut hash_table = HashTable::new_empty();

    // fill hash_table until fill level is at least p
    let p = 0.9;
    let fill_amount = (TABLE_SIZE as f64 * p) as usize;
    println!("Filling {} of {} slots.", fill_amount, TABLE_SIZE);

    let mut cnt = 0;
    while hash_table.get_filled_count() < fill_amount { // we probably already have some collisions so we need to check if it really filled it up
        let key = rand.next_u32();
        let value = cnt;

        hash_table.insert_key(key, value);

        cnt += 1;
    }

    println!("Table: {:?}", hash_table);
    println!("Filled Fields: {}", hash_table.get_filled_count());

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