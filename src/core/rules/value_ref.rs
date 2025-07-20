use std::rc::Rc;
use std::sync::Arc;

// ┌────────────────────────────────────────────┐
// │           Основний trait ValueRef          │
// └────────────────────────────────────────────┘
pub trait ValueRef {
    type Target;
    fn value(&self) -> Option<&Self::Target>;
}

// ┌────────────────────────────────────────────┐
// │        Імплементації для числових типів    │
// └────────────────────────────────────────────┘
macro_rules! impl_value_ref_for_numbers {
    ($($t:ty),*) => {
        $(
            impl ValueRef for $t {
                type Target = $t;
                fn value(&self) -> Option<&Self::Target> {
                    Some(self)
                }
            }
        )*
    };
}

impl_value_ref_for_numbers!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);

// ┌────────────────────────────────────────────┐
// │          Делегуючі обгортки                │
// └────────────────────────────────────────────┘
impl<T: ValueRef> ValueRef for &T {
    type Target = T::Target;
    fn value(&self) -> Option<&Self::Target> {
        (**self).value()
    }
}

impl<T: ValueRef> ValueRef for Option<T> {
    type Target = T::Target;
    fn value(&self) -> Option<&Self::Target> {
        self.as_ref().and_then(|v| v.value())
    }
}

impl<T: ValueRef> ValueRef for Box<T> {
    type Target = T::Target;
    fn value(&self) -> Option<&Self::Target> {
        (**self).value()
    }
}

impl<T: ValueRef> ValueRef for Rc<T> {
    type Target = T::Target;
    fn value(&self) -> Option<&Self::Target> {
        (**self).value()
    }
}

impl<T: ValueRef> ValueRef for Arc<T> {
    type Target = T::Target;
    fn value(&self) -> Option<&Self::Target> {
        (**self).value()
    }
}

// ┌────────────────────────────────────────────┐
// │                 Тести 🧪                    │
// └────────────────────────────────────────────┘
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_for_primitives() {
        let v: i32 = 42;
        assert_eq!(v.value(), Some(&42));

        let f: f64 = 3.14;
        assert_eq!(f.value(), Some(&3.14));
    }

    #[test]
    fn value_for_reference() {
        let v = &100u16;
        assert_eq!(v.value(), Some(&100));
    }

    #[test]
    fn value_for_option_some() {
        let v = Some(88i64);
        assert_eq!(v.value(), Some(&88));
    }

    #[test]
    fn value_for_option_none() {
        let v: Option<i64> = None;
        assert_eq!(v.value(), None);
    }

    #[test]
    fn value_for_boxed() {
        let v = Box::new(256u32);
        assert_eq!(v.value(), Some(&256));
    }

    #[test]
    fn value_for_rc() {
        let v = Rc::new(77u8);
        assert_eq!(v.value(), Some(&77));
    }

    #[test]
    fn value_for_arc() {
        let v = Arc::new(std::f64::consts::PI);
        assert_eq!(v.value(), Some(&std::f64::consts::PI));
    }
}
