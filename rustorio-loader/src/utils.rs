use std::iter::Peekable;

pub struct HasNextIter<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator> Iterator for HasNextIter<I> {
    type Item = (I::Item, bool);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        let has_next = self.iter.peek().is_some();
        Some((item, has_next))
    }
}

pub trait HasNextExt: Iterator + Sized {
    fn has_next(self) -> HasNextIter<Self>;
}

impl<I: Iterator<Item = T>, T> HasNextExt for I {
    fn has_next(self) -> HasNextIter<Self> {
        HasNextIter {
            iter: self.peekable(),
        }
    }
}
