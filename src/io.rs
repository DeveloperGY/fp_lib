pub trait Split<'src, T: 'src, U: 'src> {
    fn split(&'src self) -> (T, U);
}

pub trait SplitMut<'src, T: 'src, U: 'src> {
    fn split_mut(&'src mut self) -> (T, U);
}

pub trait IntoSplit<T, U> {
    fn into_split(self) -> (T, U);
}
