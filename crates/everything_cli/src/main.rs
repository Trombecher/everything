use ulid::Ulid;

fn main() {
    for _ in 0..10 {
        println!("{:?}", Ulid::new().0)
    }
}
