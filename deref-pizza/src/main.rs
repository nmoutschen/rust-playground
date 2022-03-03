use std::ops::Deref;

struct Pizza;

impl Deref for Pizza {
    type Target = [char; 8];

    fn deref(&self) -> &Self::Target {
        &['🍕', '🍕', '🍕', '🍕', '🍕', '🍕', '🍕', '🍕']
    }
}

fn main() {
    println!("{:?}", *Pizza);
}
