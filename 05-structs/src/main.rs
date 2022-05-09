// standard struct which is named and has named fields
#[derive(Debug, Clone)]
struct User {
    name: String,
    username: String,
    active: bool,
    sign_in_count: u32,
}

// tuple structs which are named but do not have named fields
// good for making a tuple a different type
// makes sense if tuples have a similar structure but mean different things
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        name: String::from("Max"),
        username: String::from("max1337"),
        active: true,
        sign_in_count: 10,
    };

    let user2 = builder_user(String::from("John"), String::from("doe1337"));
    let user3 = update_username_of_user(user2.clone(), String::from("john1337"));

    println!("{:?}", user1);
    println!("{:?}", user2);
    println!("{:?}", user3);

    let Color(r, g, b) = Color(2, 13, 45);
    let Point(x, y, z) = Point(1, 1, 5);

    println!("rgb({},{},{})", r, g, b);
    println!("coordinates(x:{},y:{},z:{})", x, y, z);
}

fn builder_user(name: String, username: String) -> User {
    User {
        name,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn update_username_of_user(user: User, username: String) -> User {
    User {
        username,
        // object update syntax
        ..user
    }
}
