pub mod reexport {
    pub use num_traits::ToPrimitive;
}

pub trait InternalTraitTest {}

pub struct TestStruct;

impl reexport::ToPrimitive for TestStruct {
    fn to_i64(&self) -> Option<i64> {
        todo!()
    }

    fn to_u64(&self) -> Option<u64> {
        todo!()
    }
}

impl InternalTraitTest for TestStruct {}
