fn main() {
    // Vectors
    let arr = [1, 2, 3];
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // access element of vector
    let third = &v1[2];
    println!("{}", third);
    
    // or safer way is use get
    match v1.get(2) {
        Some(ele) => println!("{}", ele),
        None => println!("index out of bound")
    }
    
    let v2 = vec![1, 2, 3];
    for item in &v2 {
        println!("{}", item);
    }

    // Strings : They are stored as a collection of UTF-8 encoded bytes

    let s1 = String::new();
    let s2 = "string slice";
    let s3 = s2.to_string();
    let s4 = String::from("string");

    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');

    // concatenation 
    let s5 = s4 + &s; // stringfoobar!
    
    // or use format macro
    let s6 = format!("{}{}", s5, s); // stringfoobar!foobar!
    println!("{}{}", s5, s6);

    // read bytes of string
    for i in s.bytes() {
        println!("{}", i);
    }

    // read chars of string
    for i in s.chars() {
        println!("{}", i);
    }
}