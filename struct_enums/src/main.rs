

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}

pub struct Point {
    x: f64,
    y: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

struct Empty; // unit struct

enum Message {
    Quit, 
    Move { x: i32, y: i32 },
    Write (String),
    ChangeColor(i32, i32, i32),

}

enum Animal {
    Dog(String),
    Cat(String),
    Bird(String),
}


fn main() {

    let chizaa = Person {
        name: String::from("Chizaa"),
        age: 25,
    };




    println!("My name is {:?} and I am {:?} years old", chizaa.name, chizaa.age);

    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 4.0, y: 5.0 };

    println!("The distance between p1 and p2 is: {}", distance(p1, p2));

  

    let empty_instance = Empty;
    empty_instance.greet();

   


    let rect1 = Rectangle { width: 30.0, height: 50.0 };

    println!("The area of the rectangle is: {}", rect1.area());
    println!("The perimeter of the rectangle is: {}", rect1.perimeter());

    let msg = Message::Write(String::from("hello"));
    process_message(msg);

    let my_pet = Animal::Cat("Fluf".to_string());

    if let Animal::Cat(name) = my_pet {
        println!("My cat's name is: {}", name);
    } else {
        println!("My pet is not a cat.");
    }


   
    let msg = Message::Write(String::from("Hello, Rust!"));
    msg.call();


}

fn distance(p1: Point, p2: Point) -> f64 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;

    (x_diff * x_diff + y_diff * y_diff).sqrt()

}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("Change color to R: {}, G: {}, B: {}", r, g, b),
        }
    }
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

 
}

impl Empty {
    fn greet (&self) {
        println!("Hello");

    }
}

fn process_message (msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        Message::Write (text) => {
            println!("Text message: {}", text);
        },
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        },
    }

}