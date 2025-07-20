pub trait SliceRef {
    type Item;

    fn slice(&self) -> Option<&[Self::Item]>;
}

impl<T> SliceRef for Vec<T> {
    type Item = T;
    fn slice(&self) -> Option<&[T]> {
        Some(self.as_slice())
    }
}

impl<T> SliceRef for Option<Vec<T>> {
    type Item = T;
    fn slice(&self) -> Option<&[T]> {
        self.as_ref().map(|v| v.as_slice())
    }
}

impl<T> SliceRef for &Vec<T> {
    type Item = T;
    fn slice(&self) -> Option<&[T]> {
        Some(self.as_slice())
    }
}

impl<T> SliceRef for Box<Vec<T>> {
    type Item = T;
    fn slice(&self) -> Option<&[T]> {
        Some(self.as_slice())
    }
}

impl<T> SliceRef for std::rc::Rc<Vec<T>> {
    type Item = T;
    fn slice(&self) -> Option<&[T]> {
        Some(self.as_slice())
    }
}

impl<T> SliceRef for std::sync::Arc<Vec<T>> {
    type Item = T;
    fn slice(&self) -> Option<&[T]> {
        Some(self.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::SliceRef;
    use std::rc::Rc;
    use std::sync::Arc;

    #[test]
    fn slice_from_vec() {
        let value = vec![1, 2, 3];
        let slice = value.slice();
        assert_eq!(slice, Some(&[1, 2, 3][..]));
    }

    #[test]
    fn slice_from_option_vec_some() {
        let value = Some(vec![4, 5]);
        let slice = value.slice();
        assert_eq!(slice, Some(&[4, 5][..]));
    }

    #[test]
    fn slice_from_option_vec_none() {
        let value: Option<Vec<u32>> = None;
        let slice = value.slice();
        assert_eq!(slice, None);
    }

    #[test]
    fn slice_from_ref_vec() {
        let vec = vec![6, 7, 8];
        let value = &vec;
        let slice = value.slice();
        assert_eq!(slice, Some(&[6, 7, 8][..]));
    }

    #[test]
    fn slice_from_box_vec() {
        let value = Box::new(vec![9, 10]);
        let slice = value.slice();
        assert_eq!(slice, Some(&[9, 10][..]));
    }

    #[test]
    fn slice_from_rc_vec() {
        let value = Rc::new(vec![11]);
        let slice = value.slice();
        assert_eq!(slice, Some(&[11][..]));
    }

    #[test]
    fn slice_from_arc_vec() {
        let value = Arc::new(vec![12, 13]);
        let slice = value.slice();
        assert_eq!(slice, Some(&[12, 13][..]));
    }

    #[test]
    fn slice_from_empty_vec() {
        let value: Vec<i32> = vec![];
        let slice = value.slice();
        assert_eq!(slice, Some(&[][..]));
    }

    #[test]
    fn slice_from_option_vec_empty() {
        let value: Option<Vec<u8>> = Some(vec![]); // або Vec<i32>, якщо потрібно
        let slice = value.slice();
        assert_eq!(slice, Some(&[][..]));
    }
}
