pub fn vector_file() {
    let v: Vec<i32> = Vec::new(); //optional. blank vector of i32 type
    let mut v = vec![1, 2, 3];
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v2 = vec![1, 2, 3, 4, 5];
    
    let third : &i32 = &v[2];
    println!("The third element is {third}");
    
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("element {third}"),
        None => println!("element "),
    }
    

    let v3 = vec![1, 2, 3, 4, 5];

    // let does_not_exist = &v3[100]; // error -> out of len.
    let does_not_exist = v3.get(100); // -> return None

    let mut v4 = vec![1, 2, 3, 4, 5];

    let first = &v4[0];

    // v4.push(6); // error -> create new address and allocate

    println!("The first element is: {first}");

    //loop
    for i in &v4 {
        println!("{i}");
    }

    //mut
    let mut v5 = vec![100, 32, 57];
    for i in &mut v5 {
        *i += 50; // dereference
    }

    //Various types
    enum  SpreadsheetCall {
        Int(i32),
        Float(f64),
        Text(String),
        
    }

    let row = vec![
        SpreadsheetCall::Int(3),
        SpreadsheetCall::Text(String::from("blue")),
        SpreadsheetCall::Float(10.12),
    ];

    {
        let v6 = vec![1, 2, 3, 4];

    } // vetor(v6) is lost 
    
}