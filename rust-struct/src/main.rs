struct User  {
    name:String,
    email:String,
    age:u8
} 


fn main() {

    let user = User{
        name:String::from("Naruto"),
        age:16,
        email:String::from("naruto@gamil.com"),
    };

    println!("{}",user.name);
    println!("{}",user.email);
    println!("{}",user.age);
}
