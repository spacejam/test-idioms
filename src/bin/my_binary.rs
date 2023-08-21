use my_crate::MyObject;

fn main() {
    let mut my_object = MyObject::new();

    // doesn't hit the bug
    my_object.apply(0x1338);

    println!("no bugs expressed in this execution");
}
