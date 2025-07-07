struct Nyde<T> {
    origin: *mut T,
}

impl<T> Nyde<T> {
    fn new(origin: *mut T) -> Self {
        Self { origin }
    }
}
