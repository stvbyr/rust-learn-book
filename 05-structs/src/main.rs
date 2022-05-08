
#[derive(Debug)]
#[derive(Clone)]
struct User {
    name: String,
    username: String,
    active: bool,
    sign_in_count: u32
}

fn main() {
    let user1 = User {
        name: String::from("Max"),
        username: String::from("max1337"),
        active:true,
        sign_in_count: 10
    };

    let user2 = builder_user(String::from("John"), String::from("doe1337"));
    let user3 = update_username_of_user(user2.clone(),  String::from("john1337"));

    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{:?}", user3);
}

fn builder_user(name: String, username: String) -> User {
    User {
        name,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn update_username_of_user(user: User, username: String) -> User {
    User {
        username,
        ..user
    }
}