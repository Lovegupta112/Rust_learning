use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, Vec<u32>> = HashMap::new();

    scores.insert("red".to_string(), vec![100]);
    scores.insert("blue".to_string(), vec![120]);

    println!("scores: {:#?}", scores);

    // get ---

    let redScore = scores.get("red");
    println!("red team scores: {:?}", redScore);
    let greenScore = scores.get("green");
    println!("green team scores: {:?}", greenScore);

    // update ----

    let score = scores.entry("red".to_string()).or_insert(vec![500]);
    // *score += 200;
    println!("scores: {:#?}", scores);
}
