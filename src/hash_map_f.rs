use std::collections::HashMap;

pub fn hash_map_file() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 30);
    
    let team = String::from("Blue");

    //Option<&i32>
    // let team_score = scores.get(&team);
    //&i32
    let team_score= scores.get(&team).unwrap();

    //i32
    let team_score = scores.get(&team).copied().unwrap_or(0);
    
    // borrow -> &scores, move -> scores
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // error. because: move to map
    // print!("test{}",field_name);

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    //25
    println!("{:?}", scores);

    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);

    //entry feature: scores have "Blue" key. so not insert 50 to Blue
    scores2.entry(String::from("Yellow")).or_insert(50);
    scores2.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores2);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        //key's address return
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}