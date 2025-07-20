pub trait PrimitiveRule {}

impl PrimitiveRule for String {}
impl PrimitiveRule for &str {}
impl PrimitiveRule for i32 {}
impl PrimitiveRule for i64 {}
impl PrimitiveRule for u32 {}
impl PrimitiveRule for u64 {}
impl PrimitiveRule for f32 {}
impl PrimitiveRule for f64 {}
impl PrimitiveRule for bool {}
impl PrimitiveRule for char {}
