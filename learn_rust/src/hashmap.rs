/**
 * In this module we will go through few examples and explain HashMap in rust lang.
 * Needed import. (line 14)
 * What is HashMap explained. (line 17)
 * HashMap example and syntax. (line 24)
 * HashMap insert(); example. (line 27)
 * HashMap len(); example. (line 33)
 * HashMap get(); example. (line 36)
 * HashMap iter(); example. (line 46)
 * HashMap contains_key(); example. (line 51)
 * HashMap remove(); example. (line 56)
 */
pub mod module {
    use std::collections::HashMap; // NOTE: Needs to be imported to use HashMap.

    pub fn hashmap() {
        // A map is a collection of key-value pairs (called entries).
        // No two entries in a map can have the same key.
        // In short, a map is a lookup table. A HashMap stores the keys and values in a hash table.
        // The entries are stored in an arbitrary order. The key is used to search for values in the HashMap.
        // The HashMap structure is defined in the std::collections module.
        // This module should be explicitly imported to access the HashMap structure.

        // HashMap example and syntax.
        // let mut instance_name = HashMap::new();

        // HashMap insert();
        let mut state_codes = HashMap::new();
        state_codes.insert("NY","New York");
        state_codes.insert("LS","Los Angeles");
        println!("{:?}",state_codes);

        // HashMap len();
        println!("{:?}",state_codes.len());

        // HashMap get();
        match state_codes.get(&"NY") {
            Some(value)=> {
                println!("Value for key NY is {}",value);
            }
            None => {
                println!("nothing found");
            }
        }

        // HashMap iter();
        for (key, val) in state_codes.iter() {
            println!("key: {} val: {}", key, val);
        }

        // HashMap contains_key();
        if state_codes.contains_key(&"LS") {
            println!("found key");
        }

        // HashMap remove();
        println!("length of the hashmap before remove() {}",state_codes.len());
        state_codes.remove(&"LS");
        println!("length of the hashmap after remove() {}",state_codes.len());
    }
}