/*struct Point {
    x: i32,
    y: i32,
}
*/
struct Point {
    x: i32,
    y: i32,
    z: i32
}
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn foo(_: i32, y:i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    /*let x = Some(5);
    let y= 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
     */

    /*
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    */

    /*
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    */

    /*
    let x = '1';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
    */

    /*let p = Point {x: 0, y: 7};

    match p {
        Point {x, y: 0} => println!("On the x axis at {x}"),
        Point{x:0, y} => println!("On the y axis at {y}"),
        Point {x, y} => {
            println!("On neither axis: ({x}, {y})");
        }
    }*/

    // let msg = Message::ChangeColor(0, 160, 255);

    // let msg = Message::Write(String::from("This is a test"));

    /*let msg = Message::Move {x: 5, y: 10};

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move {x ,y } => {
            println!(
                "Move in the x direction {x} and in the y direction {y}"
            )
        },
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {r}, green {g}, and blue {b}",
        ),
    }*/

/*    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h ,s ,v )) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        },
        _ => (),
    }*/

    // foo(3,4)

/*    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("Setting is {:?}", setting_value);*/

/*    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        },
        _ => { println!("Something else") }
    }*/


/*    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);
*/
/*    let origin = Point {x: 0, y: 0, z: 0};

    match origin {
        Point {x, ..} => println!("x is {}", x),
    }*/

    /*let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }*/

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x)
    }


}
