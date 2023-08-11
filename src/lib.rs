pub use comptime_builder_macros::Builder;

pub struct Empty;


pub trait HasField<const N: usize, T> {}
impl<const N: usize, T> HasField<N, T> for Field<N, T> {}
pub struct Field<const N: usize, T>(pub T);

pub trait WithField<const N: usize, Field, Struct>: Sized {
    type Output;
    fn with_field(self, value: Field) -> Self::Output;
}
