use comptime_builder::Builder;

#[derive(Builder)]
pub struct MyStruct<T1, T2> {
    pub my_field: T1,
    pub my_other_field: T2,
    pub our_field: String,
}

pub fn main() {
    
}
