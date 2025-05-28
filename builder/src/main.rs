pub struct MyObject {
    a: i32,
    b: i32,
    c: i32,
}

impl MyObject {
    pub fn builder() -> MyObjectBuilder {
        MyObjectBuilder::new()
    }
}

struct MyObjectBuilder {
    a: i32,
    b: i32,
    c: i32,
}

impl MyObjectBuilder {
    pub fn new() -> Self {
        MyObjectBuilder { a: 0, b: 0, c: 0 }
    }

    pub fn set_a(&mut self, a: i32) -> &mut Self {
        self.a = a;
        self
    }

    pub fn set_b(&mut self, b: i32) -> &mut Self {
        self.b = b;
        self
    }

    pub fn build(&self) -> MyObject {
        MyObject {
            a: self.a,
            b: self.b,
            c: self.c,
        }
    }
}

fn main() {
    let mut builder = MyObject::builder();
    let obj = builder.set_a(1).set_b(2).build();
}
