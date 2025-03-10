use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    };

    // ownerships in hashmap

    let team_color = String::from("Red");
    let team_score = 30;

    scores.insert(team_color, team_score);
    // now that we have moved the team_color and team_score in the hashmap, we can no longer use
    // them in the code, they are gone :(


    // editing a hashmap

    // overwriting
    scores.insert(String::from("Blue"), 50);
    println!("{scores:?}");

    // adding key value pair only if key isnt present

    scores.entry(String::from("Yellow")).or_insert(5);
    scores.entry(String::from("Brown")).or_insert(80);

    println!("{scores:?}");

    // updating value using old value

    for (team, score) in scores.iter_mut() {
        *score += 5;
    }

    println!("After updating: {:?}", scores);

}
