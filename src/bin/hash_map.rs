use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Kurt"), 10);
    scores.insert(String::from("Gilad"), 8);

    for (k, v) in &scores {
        println!("{}'s scores: {}", k, v);
    }
    let test_ele = scores.get(&String::from("Kurt")).copied().unwrap_or(0);
    // get() -> Optin<&V>
    println!("{}", test_ele);

    // Update value
    scores.insert(String::from("Gilad"), 11);
    let new_gilad_score = scores.get("Gilad").copied().unwrap_or(0);
    println!("Gilad is now: {}", new_gilad_score);

    // Add new k,
    scores
        .entry(String::from("Moreno"))
        .and_modify(|v| *v += 3)
        .or_insert(7);
    for (k, v) in &scores {
        println!("{}'s scores: {}", k, v);
    }
    scores
        .entry(String::from("Moreno"))
        .and_modify(|v| *v += 3)
        .or_insert(7);
    for (k, v) in &scores {
        println!("{}'s scores: {}", k, v);
    }
}
