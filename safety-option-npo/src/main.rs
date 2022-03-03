use std::mem::transmute;

fn main() {
    let inner = String::from("hello, world!");

    // Option with Some borrowed value
    let some_val = Option::Some(&inner);
    // Transmute the pointer into an usize
    let some_val_ptr = unsafe { transmute::<_, usize>(some_val) };

    // Option with None
    let none_val = Option::<&String>::None;
    // Transmute the pointer into an usize
    // This will have a value of zero because of null-pointer optimization
    // See https://doc.rust-lang.org/std/option/index.html#representation
    let none_val_ptr = unsafe { transmute::<_, usize>(none_val) };

    dbg!(some_val, some_val_ptr);
    dbg!(none_val, none_val_ptr);
}
