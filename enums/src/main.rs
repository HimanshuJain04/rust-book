// enum Shape {
//     Rectangle(f32, f32),
//     Circle(f32),
//     Square(f32),
// }

// fn calculate_are(shape: Shape) -> f32 {
//     match shape {
//         Shape::Rectangle(a, b) => a * b,
//         Shape::Circle(r) => 3.14 * r * r,
//         Shape::Square(s) => s * s,
//     }
// }

fn find_first_n(str: String) -> Option<u8> {
    for (i, char) in str.chars().enumerate() {
        if char == 'n' {
            return Some(i as u8);
        }
    }
    return None;
}

fn main() {
    // let rect = Shape::Rectangle(5.0, 10.0);
    // let square = Shape::Square(5.0);
    // println!("Rect: {}", calculate_are(rect));
    // println!("Sqr: {}", calculate_are(square));

    let str = String::from("Himashu");
    // let str = String::from("Himanshu");
    let index = find_first_n(str);

    match index {
        Option::Some(val) => println!("{}", val),
        Option::None => println!("Not Found"),
    }
}
