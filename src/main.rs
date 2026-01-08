fn main() {
    let a = [1, 2, 3];

    let mut iter = a.into_iter(); // `into_iter() iterates directly over the elements and get the elements ownership dirctly to the iterator`

    // A call to next() returns the next value.....
    assert_eq!(Some(1), iter.next());
    assert_eq!(Some(2), iter.next());
    assert_eq!(Some(3), iter.next());

    // ... and then None once it's over
    assert_eq!(None, iter.next());

    // More calls may or may not returns `None`. Here, they always will.
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
}
