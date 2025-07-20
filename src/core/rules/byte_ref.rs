use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

pub trait ByteRef {
    fn as_bytes_ref(&self) -> Option<&[u8]>;
}

// ─────────── Базові типи ───────────

impl ByteRef for Vec<u8> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        Some(self)
    }
}

impl ByteRef for &[u8] {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        Some(*self)
    }
}

impl ByteRef for Box<[u8]> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        Some(self)
    }
}

impl ByteRef for Cow<'_, [u8]> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        Some(self.as_ref())
    }
}

// ─────────── Обгортки ───────────

impl<T: ByteRef> ByteRef for Option<T> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        self.as_ref()?.as_bytes_ref()
    }
}

impl<T: ByteRef> ByteRef for Arc<T> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        (**self).as_bytes_ref()
    }
}

impl<T: ByteRef> ByteRef for Rc<T> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        (**self).as_bytes_ref()
    }
}

impl<T: ByteRef> ByteRef for Box<T> {
    fn as_bytes_ref(&self) -> Option<&[u8]> {
        (**self).as_bytes_ref()
    }
}
