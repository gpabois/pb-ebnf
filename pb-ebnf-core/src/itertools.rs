pub trait Single: Iterator + Sized {
    // Returns the element if the iterator only contains one item.
    fn single(mut self) -> Option<Self::Item> {
        self.next()
            .and_then(move |i| if self.next().is_none() { Some(i) } else { None })
    }
}

impl<T> Single for T where Self: Iterator {}
