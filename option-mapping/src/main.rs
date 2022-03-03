struct Val<T>(Option<T>);

fn main() {
    let my_val = Val(Some(Val(Some(Val(Some(3))))));
    let inner_val = (move || { my_val.0?.0?.0 })();
    println!("{:?}", inner_val);
}
