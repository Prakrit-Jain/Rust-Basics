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
}