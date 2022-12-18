fn main() {
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly;
    let s = "initial contents".to_string();

    let s = String::from("initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);

    let mut s = String::from("lo");
    s.push('l');

    // Concatenation using '+'
    let s1 = String::from("Hello, ");
    let s2 = String::from("word!");
    // s1 becomes dropped, out of scope, it's new owner is the add method (See the documentation)
    // s2 can still be used as it's a reference
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s= s1 + "-" + &s2 + "-" + &s3;

    // println!("s is {}", s);
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    let s1 = String::from("hello");
    let h = s1[0];
}
