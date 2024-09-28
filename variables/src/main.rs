fn main() {
    //    variable declarations
    // let a = 5;
    // println!("value of a is {a}");

    // let a = 5; //  can't be changed later

    // Correct way to mutate the value
    // let mut a = 5;
    // println!("value of a is {a}");

    // a = 10;
    // println!("value of updated a is {a}");

    // Overshadowing
    let x = 5;
    let x = x + 1; // here we can change type of value

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
