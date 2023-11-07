use math_lib::prelude::Vector3;

fn main() {
    let r = Vector3::default();
    let r2 = Vector3::new(1., 2., 3.);
    let mut r3 = r + r2;
    r3 += r2;

    println!("result: {}", r);
    println!("result: {}", r2);
    println!("result: {}", r3);
}
