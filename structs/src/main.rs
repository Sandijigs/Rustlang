// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!("The area of the rectangle is {} square pixels.", area(width1, height1));
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
// fn main(){
//     let recti= (30,59);
//     println!("The area of the rectangle is {} square pixels.", area(recti));
// }
//  fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
//  }

// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main(){
//     let rect = Rectangle { width: 30, height: 50 };
//     println!("The area of the rectangle is {} square pixels.", area(&rect));
// }
// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }



// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(&rect1)
//     );
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }


// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         rect1.area()
//     );
// }
// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let scale = 2;
//     let rect1 = Rectangle {
//         width: dbg!(30 * scale),
//         height: 50,
//     };

//     dbg!(&rect1);
// }

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn is_tall(&self) -> bool { //You defined a method inside the impl block for Rectangle.You used &self, which means you're borrowing the instance to read its data — no ownership taken, no mutation.
        self.height > 100 //you checked whether the height field is more than 100 and returned a boolean./
    }
}
impl Rectangle {
    fn scale_up(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
    fn shrink_by_half(&mut self) {
        self.width /= 2;
        self.height /= 2;
    }
      fn resized(&self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
    
    fn double(&mut self) -> &mut Self {
        self.width *= 2;
        self.height *= 2;
        self
    
}
}


fn main() {
    let mut rect = Rectangle {
        width: 10,
        height: 20,
    };
     let bigger = rect.resized(3);

 println!("Original: {:?}", rect);  // Original: Rectangle { width: 10, height: 20 }
println!("Resized: {:?}", bigger);
rect.double().shrink_by_half();
     println!("Result: {:?}", rect); // Rectangle { width: 5, height: 10 }
}

