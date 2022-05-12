fn main() {
    let a = [1, 2, 3];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(3);
    v.push(2);
    
    let third = &v[2];
    println!("The third element is {}", third);

    v.push(4);

    // we can produce run time errors with vectors
    // if we access an element that does not exist
    // let third = &v[100];

    // there is a safe way to access the values of a vector
    // by using the get method and pattern matching
    match v.get(100) {
        Some(x) => println!("The value is {}", x),
        None => println!("There is no value for that index"),
    };


    for i in &v {
        println!("Value: {}", i)
    }
    for i in &mut v {
        // we give the loop a mutable reference but we want to change the value
        // we have to get the underlying value or dereference the i to mutate it
        *i += 10;
        println!("Value: {}", i);
    }

    // dropped when the scope ends
    {
        let v2 = vec![1, 2, 3];
    }

    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String)
    }

    let row = vec![
        Spreadsheet::Int(3),
        Spreadsheet::Text(String::from("hello")),
        Spreadsheet::Float(4.9),
    ];

    // we have a vector of spreadsheet variants
    // in order to use their values we need to do pattern_matching
    match &row[1] {
        Spreadsheet::Int(i) => println!("{}", i),
        _ => ()
    }

    // Strings
    let mut s = String::from("Hello");
    s.push_str(" World");
    s.push('!');

    let s1 = String::from("Hello");
    let s2 = String::from(" World");
    let s3 = s1 + s2;
    
}