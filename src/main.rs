// Chapter 1 exercises

fn identity<T>(arg: T) -> T {
    arg
}

fn compose<A, B, C>(f1: fn(A) -> B, f2: fn(B) -> C, arg: A) -> C {
    f2(f1(arg))
}

fn add_two(arg: u16) -> u16 {
    arg + 2
}

fn main() {
    assert_eq!("A", identity("A"));

    assert_eq!(14, compose(add_two, add_two, 10));

    assert_eq!(12, compose(identity, add_two, 10));
    assert_eq!(12, compose(add_two, identity, 10));

}   