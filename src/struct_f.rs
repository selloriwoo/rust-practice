pub fn stfile() {
    let w =50;
    let h =50;

    println!("result: {}",area(w, h));

    let mut rect = (50,50);
    println!("result2: {}", area2(rect));
    rect.0 =30;
    println!("result2: {}", area2(rect));

    let rect2 = Rectangle { length : 30, width:50 };
    println!("result3: {}",area3(&rect2));
    println!("result4: {:?}", rect2);
}

fn area(w: i32, h : i32) -> i32 {
    w*h
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}