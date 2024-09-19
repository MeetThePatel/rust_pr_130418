pub trait TestTrait {
    fn plus_one(&mut self);
}

pub trait TestTrait2 {
    fn minus_one(&mut self);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TestStruct {
    val: i32,
}

impl std::fmt::Display for TestStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl TestTrait for TestStruct {
    fn plus_one(&mut self) {
        self.val += 1;
    }
}
impl TestTrait2 for TestStruct {
    fn minus_one(&mut self) {
        self.val -= 1;
    }
}
