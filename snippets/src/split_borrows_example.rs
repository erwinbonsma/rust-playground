pub fn main() {
    let mut v = vec!(1, 2, 3);
    let mut iter = v.iter_mut();

    let v1 = iter.next().unwrap();
    let v2 = iter.next().unwrap();

    *v1 *= 3;
    *v2 *= 3;
    let v3 = iter.next().unwrap();
    // println!("{:?}", v);

    *v3 *= 3;
    println!("{:?}", v);
}