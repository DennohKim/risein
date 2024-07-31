

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


fn main() {

    let chizaa = Person {
        name: String::from("Chizaa"),
        age: 25,
    };


    println!("My name is {:?} and I am {:?} years old", chizaa.name, chizaa.age);

    let p1 = Point { x: 1.0, y: 1.0 };
    let p2 = Point { x: 4.0, y: 5.0 };

    println!("The distance between p1 and p2 is: {}", distance(p1, p2));

    impl Empty {
        fn greet (&self) {
            println!("Hello");

        }
    }

    let empty_instance = Empty;
    empty_instance.greet();

    impl Rectangle {
        fn area(&self) -> f64 {
            self.width * self.height
        }

        fn perimeter(&self) -> f64 {
            2.0 * (self.width + self.height)
        }

     
    }


    let rect1 = Rectangle { width: 30.0, height: 50.0 };

    println!("The area of the rectangle is: {}", rect1.area());
    println!("The perimeter of the rectangle is: {}", rect1.perimeter());


}

fn distance(p1: Point, p2: Point) -> f64 {
    let x_diff = p1.x - p2.x;
    let y_diff = p1.y - p2.y;

    (x_diff * x_diff + y_diff * y_diff).sqrt()

}