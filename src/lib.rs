pub enum ClonelessCow<'a, T>
where
    T: 'a,
{
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> AsRef<T> for ClonelessCow<'a, T> {
    fn as_ref(&self) -> &T {
        match self {
            ClonelessCow::Borrowed(borrowed) => borrowed,
            ClonelessCow::Owned(owned) => owned,
        }
    }
}