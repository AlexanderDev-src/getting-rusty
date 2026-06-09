struct Stacktest<T> {
    number: Vec<T>,
}

impl<T> Stacktest<T> {
    pub fn new() -> Self {
        Stacktest { number: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.number.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.number.pop()
    }
}

pub fn Stacktest_run() {
    let mut my_stack = Stacktest::new();

    my_stack.push(20);
    my_stack.push(50);
    my_stack.push(14);

    println!("{:?}", my_stack.pop());
    println!("{:?}", my_stack.pop());
    println!("{:?}", my_stack.pop());
}
