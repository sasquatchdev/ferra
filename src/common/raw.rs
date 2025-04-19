pub trait AsRaw<T>
{
    fn as_raw(self) -> Vec<T>;
}

/// automatically implement AsRaw for a Vector
/// of any type that implements AsRaw.
impl<I, T, U> AsRaw<U> for I
    where T: AsRaw<U>, I: IntoIterator<Item = T>
{
    fn as_raw(self) -> Vec<U>
    {
        let mut raw = Vec::new();
        for item in self.into_iter(){
            raw.extend(item.as_raw());
        }
        raw
    }
}
