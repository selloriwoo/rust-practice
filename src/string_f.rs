pub fn string_file() {
    let mut s = String::new();

    let data = "abc";
    
    let s = data.to_string();
    let s = "abc".to_string();

    let mut s2 = String::from("abc");

    s2.push('d');
    s2.push_str("efg");
    let s3 = String::from("Hello, ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4; // s3 is moved here and can no longer be used

    // let s6 = s4 + "-" + &s5;
    let s6 = format!("{s4}-{s5}"); // format! does not take ownership of its parameters

    // let answer = &s2[0]; // error: cannot index into a `String` with an integer

    // slicing a string is possible, but the range must be on valid UTF-8 character boundaries
    // can be panic if the range is not on valid UTF-8 character boundaries
    let s7 = &s2[0..4]; 

    // return З д
    for c in "Зд".chars() {
    println!("{c}");
    }
    // return [208, 151, 208, 180]
    for b in "Зд".bytes() {
    println!("{b}");
    }

}