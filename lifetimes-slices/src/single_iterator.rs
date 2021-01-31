struct Single<T> {
    next: Option<T>,
}

fn single<T>(next: T) -> Single<T> {
    Single { next: Some(next) }
}

impl<T> Iterator for Single<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // This doesn't work since we attempt to move
        // self.next outside of a borrow
        // (meaning we have a reference to self but try to move
        // a value that is inside self)
        // self.next

        // We can swap value of self.next with a temp local variable
        // let mut res = None;
        // std::mem::swap(&mut res, &mut self.next);
        // res

        // Same thing without temporary variable
        // std::mem::replace(&mut self.next, None)

        // Or just
        self.next.take()
    }
}

pub fn run() {
    println!("This is Single Iterator example");
    let actual: Vec<u32> = single(42).collect();
    assert_eq!(vec![42], actual);
}
