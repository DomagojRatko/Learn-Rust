/**
 * In this module we will go through few examples and explain HashSet in rust lang.
 * Needed import. (line 14)
 * What is HashSet explained. (line 17)
 * HashSet example and syntax. (line 22)
 * HashSet insert(); example. (line 25)
 * HashSet len(); example. (line 33)
 * HashSet iter(); example. (line 36)
 * HashSet get(); example. (line 41)
 * HashSet contains(); example. (line 52)
 * HashSet remove(); example. (line 57)
 */
pub mod module {
    use std::collections::HashSet; // NOTE: Needs to be imported to use HashMap.

    pub fn hashset() {
        // HashSet is a set of unique values of type T. Adding and removing values is fast,
        // and it is fast to ask whether a given value is in the set or not.
        // The HashSet structure is defined in the std::collections module.
        // This module should be explicitly imported to access the HashSet structure.

        // HashSet example and syntax.
        // let mut hash_set_name = HashSet::new();

        // HashSet insert();
        let mut names = HashSet::new();
        names.insert("Joe");
        names.insert("Karen");
        names.insert("Jack");
        names.insert("Joe"); //duplicates not added
        println!("{:?}",names);

        // HashSet len();
        println!("size of the set is {}",names.len());

        // HashSet iter();
        for name in names.iter() {
            println!("{}",name);
        }

        // HashSet get();
        match names.get(&"Jack"){
            Some(value)=>{
                println!("found {}",value);
            }
            None =>{
                println!("not found");
            }
        }
        println!("{:?}",names);

        // HashSet contains();
        if names.contains(&"Joe") {
            println!("found name");
        }

        // HashSet remove();
        println!("length of the Hashset before remove() : {}",names.len());
        names.remove(&"Karen");
        println!("length of the Hashset after remove() : {}",names.len());
    }
}