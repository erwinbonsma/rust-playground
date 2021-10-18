fn show(s: &String)    { println!("{}", s); }
fn foo(s: &mut String) { s.push_str("!"); }
fn bar(mut s: String)  { s.push_str("!"); }

pub fn main() {
    let s = String::from("Hello");
    // show(&s);

    let mut q = s;
    // show(&s);

    q.push_str("?");
    // show(&q);

    foo(&mut q);
    // show(&q);

    bar(q);
    // show(&q);
}
