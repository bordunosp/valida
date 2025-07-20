use crate::core::contract::IValidate;

pub trait NestedField<V, E>
where
    V: Send + Sync + 'static,
    E: std::error::Error + Send + Sync + 'static,
{
    fn nested<TValidator>(self, validator: TValidator) -> Self
    where
        TValidator: IValidate<V, E> + 'static;
}
