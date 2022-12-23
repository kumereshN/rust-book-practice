fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

/*struct Point<T> {
    x: T,
    y: T
}*/

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    // The <X2, Y2> generics are only relevant to other parameter
    fn mixup<X2, Y2> (self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// For all types
impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Only for f32 types
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
/*    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let result = largest(&number_list);
    println!("The largest number is {}", result);*/

/*    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};

    let wont_work = Point {x: 5, y: 4.0};*/

    let p = Point {x: 5, y: 10};

    println!("p.x = {}", p.x());

}
