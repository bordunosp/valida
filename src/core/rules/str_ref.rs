use std::borrow::Cow;

pub trait StrAsRef {
    fn as_str_ref(&self) -> Option<&str>;
}

impl StrAsRef for Cow<'static, str> {
    fn as_str_ref(&self) -> Option<&str> {
        Some(self.as_ref())
    }
}

impl StrAsRef for String {
    fn as_str_ref(&self) -> Option<&str> {
        Some(self.as_str())
    }
}

impl StrAsRef for &str {
    fn as_str_ref(&self) -> Option<&str> {
        Some(*self)
    }
}

impl<T: StrAsRef> StrAsRef for Option<T> {
    fn as_str_ref(&self) -> Option<&str> {
        self.as_ref().and_then(|v| v.as_str_ref())
    }
}

impl<T: StrAsRef> StrAsRef for &T {
    fn as_str_ref(&self) -> Option<&str> {
        (**self).as_str_ref()
    }
}

impl<T: StrAsRef> StrAsRef for Box<T> {
    fn as_str_ref(&self) -> Option<&str> {
        (**self).as_str_ref()
    }
}

impl<T: StrAsRef> StrAsRef for std::sync::Arc<T> {
    fn as_str_ref(&self) -> Option<&str> {
        (**self).as_str_ref()
    }
}

impl<T: StrAsRef> StrAsRef for std::rc::Rc<T> {
    fn as_str_ref(&self) -> Option<&str> {
        (**self).as_str_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn from_str_ref() {
        let value: &str = "direct";
        assert_eq!(value.as_str_ref(), Some("direct"));
    }

    #[test]
    fn from_string() {
        let value: String = "owned".to_string();
        assert_eq!(value.as_str_ref(), Some("owned"));
    }

    #[test]
    fn from_cow_borrowed() {
        let value: Cow<'static, str> = Cow::Borrowed("borrowed");
        assert_eq!(value.as_str_ref(), Some("borrowed"));
    }

    #[test]
    fn from_cow_owned() {
        let value: Cow<'static, str> = Cow::Owned("cow".to_string());
        assert_eq!(value.as_str_ref(), Some("cow"));
    }

    #[test]
    fn from_option_some_string() {
        let value: Option<String> = Some("maybe".to_string());
        assert_eq!(value.as_str_ref(), Some("maybe"));
    }

    #[test]
    fn from_option_none() {
        let value: Option<String> = None;
        assert_eq!(value.as_str_ref(), None);
    }

    #[test]
    fn from_boxed_string() {
        let value = Box::new("boxed".to_string());
        assert_eq!(value.as_str_ref(), Some("boxed"));
    }

    #[test]
    fn from_rc_string() {
        let value = Rc::new("rc".to_string());
        assert_eq!(value.as_str_ref(), Some("rc"));
    }

    #[test]
    fn from_arc_string() {
        let value = Arc::new("arc".to_string());
        assert_eq!(value.as_str_ref(), Some("arc"));
    }

    #[test]
    fn from_reference_to_string() {
        let value = &"ref".to_string();
        assert_eq!(value.as_str_ref(), Some("ref"));
    }

    #[test]
    fn from_reference_to_cow() {
        let cow: Cow<'static, str> = Cow::Owned("ref-cow".to_string());
        let value = &cow;
        assert_eq!(value.as_str_ref(), Some("ref-cow"));
    }

    #[test]
    fn nested_option_rc() {
        let value: Option<Rc<String>> = Some(Rc::new("nested".to_string()));
        assert_eq!(value.as_str_ref(), Some("nested"));
    }
}
