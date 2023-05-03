struct User {
    name: String,
    age: isize,
}

fn builder_user(name: String,age: isize) ->User {
    User { name, age}
}

fn main() {
    let mut u = User{
        name: "henry.li".to_string(),
        age: 20,
    };

    println!("user name is {}",u.name);   

    u.age = 21;
    
    println!("user age is {}",u.age);

    let mut user = builder_user(u.name, u.age);

    user.name = "qin".to_string();
    println!("user name is {}",user.name);

    let u_user = User{
        name: "demo".to_string(),
        ..u
    };

    println!("user name {}, age {}",u_user.name,u_user.age);
}
 