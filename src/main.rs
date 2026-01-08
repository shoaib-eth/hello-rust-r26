fn main() {
    let a = [1, 2, 3];

    let mut iter = a.iter(); // `iter()` does not owns the wonership of all elements, it get the reference of those elements, it does not returns the elements value while it returns the reference of those elements

    // A call to next() returns the next value.....
    assert_eq!(Some(&1), iter.next());
    assert_eq!(Some(&2), iter.next());
    assert_eq!(Some(&3), iter.next());

    // ... and then None once it's over
    assert_eq!(None, iter.next());

    // More calls may or may not returns `None`. Here, they always will.
    assert_eq!(None, iter.next());
    assert_eq!(None, iter.next());
}

// Note 📝

// Difference Between `into_iter()` and `iter()`

// `into_iter()` -> It get the ownership of elements directly and returns the elements value
// `iter()` -> It get the reference of those elemens and returns the elements reference
