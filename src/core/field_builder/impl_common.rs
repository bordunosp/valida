use crate::core::field_builder::main::FieldBuilder;
use crate::core::rules::common::not_empty::{NotEmpty, RuleTarget};
use crate::core::rules::common::not_none::NotNone;
use crate::core::rules::value_ref::ValueRef;
use std::error::Error;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

impl<'a, T, V, U, E> FieldBuilder<'a, T, V, E>
where
    V: ValueRef<Target = U> + RuleTarget,
    U: Send + Sync + std::fmt::Debug + 'static,
    E: Error + Send + Sync + 'static,
{
    pub fn not_empty(mut self) -> Self {
        self.rules.rules.push(Box::new(NotEmpty {}));
        self
    }
}

impl<'a, T, E> FieldBuilder<'a, Option<T>, Option<T>, E>
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

impl<'a, T, E> FieldBuilder<'a, T, Box<Option<T>>, E>
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
