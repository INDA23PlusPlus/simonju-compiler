pub trait IterCloneable: Iterator + Clone {
    fn iter_clone(self) -> IterClone<Self> {
        IterClone(self)
    }
}

pub struct IterClone<T>(T);

impl<T> Iterator for IterClone<T> where T: Iterator + Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|_| self.0.clone())
    }
}

impl<T: Iterator + Clone> IterCloneable for T {}
