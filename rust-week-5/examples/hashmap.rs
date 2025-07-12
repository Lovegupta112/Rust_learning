use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();

    scores.insert("red".to_string(), 120);
    scores.insert("blue".to_string(), 120);

    println!("scores: {:#?}", scores);

    // get ---

    let redScore = scores.get("red");
    println!("red team scores: {:?}", redScore);
    let greenScore = scores.get("green");
    println!("green team scores: {:?}", greenScore);

    // update ----

    let score = scores.entry("black".to_string()).or_insert(100);
    *score += 200;
    println!("scores: {:#?}", scores);
}
