
struct Rectangle { // Declare a struct
    width: f32,
    height: f32
}

// this impl contains methods for Rectangle struct
impl Rectangle { 
    fn area(&self) -> f32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width > self.width && other.height > self.height
    }
}

// this impl contains associated functions for Rectangle struct
impl Rectangle { 
    fn square(size: f32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30.0,
        height: 50.5
    };

    // Print fields of struct using dot notation
    println!("width = {}, height = {}", rect.width, rect.height);

    let area = rect.area();
    println!("Area = {} pixels", area);

    let rect1 = Rectangle {
        width: 35.9,
        height: 55.5
    };

    let rect2 = Rectangle {
        width: 23.0,
        height: 70.45
    };

    println!("is rect1 can hold rect? = {}", rect.can_hold(&rect1)); // true
    println!("is rect2 can hold rect? = {}", rect.can_hold(&rect2)); // false

    let rect3 = Rectangle::square(25.50);
    println!("width = {}, height = {}", rect3.width, rect3.height);
    
}


