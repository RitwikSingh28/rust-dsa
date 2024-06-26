use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32>= HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 30);

    let team_name = String::from("Blue");

    let _score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership in Hashmaps
    // Types implementing Copy trait, are copied into the hash map
    // Whereas for owned values like String, the values will be moved and the hashmap will be the
    // owner of those values 
    
    let field_name = String::from("Team Name");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value have been moved into the map, so we can't use them from now
    // println!("{field_name}: {field_value}");

    // Check if a key exists in the hashmap
    scores.entry(String::from("Yellow")).or_insert(50); 
    scores.entry(String::from("Blue")).or_insert(40);

    println!("{scores:?}");
}
