use crate::core::field_builder::main::FieldBuilder;
use crate::core::rules::common::not_empty::{NotEmpty, RuleTarget};
use crate::core::rules::common::not_none::NotNone;
use std::error::Error;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

pub trait SupportsNotEmpty: RuleTarget {}

impl<T> SupportsNotEmpty for T where T: RuleTarget {}

impl<'a, T, V, E> FieldBuilder<'a, T, V, E>
where
    V: SupportsNotEmpty + Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn not_empty(mut self) -> Self {
        self.rules.rules.push(Box::new(NotEmpty {}));
        self
    }
}

impl<'a, T, U, E> FieldBuilder<'a, T, Option<U>, E>
where
    U: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn not_none(mut self) -> Self {
        self.rules.rules.push(Box::new(NotNone::<U> {
            _phantom: PhantomData,
        }));
        self
    }
}

impl<'a, T, E> FieldBuilder<'a, T, Rc<Option<T>>, E>
where
    T: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn not_none(mut self) -> Self {
        self.rules.rules.push(Box::new(NotNone::<T> {
            _phantom: PhantomData,
        }));
        self
    }
}

impl<'a, T, E> FieldBuilder<'a, T, Arc<Option<T>>, E>
where
    T: Send + Sync + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn not_none(mut self) -> Self {
        self.rules.rules.push(Box::new(NotNone::<T> {
            _phantom: PhantomData,
        }));
        self
    }
}
