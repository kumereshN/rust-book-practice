use std::fmt::Display;

/*fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}*/

fn longest<'a>(x: &str, y: &str) -> String {
   String::from("really long string")
}

fn longest_with_an_announcement<'a, T> (
   x: &'a str,
   y: &'a str,
   ann: T,
) -> &'a str
where
   T: Display,
{
   println!("Announcement! {}", ann);
   if x.len() > y.len() {
      x
   } else {
      y
   }
}

fn main() {
/*    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);*/

/*    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }*/

/*    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);*/
}
