pub use dto_mapper_derive::*;
use fmap::Functor;

pub fn convert_inner<'a, T, A>(outer: T) -> T::Mapped
where
    T: Functor<'a, A>,
    T::Inner: Into<A>,
{
    outer.fmap(Into::into)
}

pub fn convert_inner_of_inner<'a, T, K, A: 'a>(outer: T) -> T::Mapped
where
    T: Functor<'a, K>,
    T::Inner: Functor<'a, A, Mapped = K>,
    <T::Inner as Functor<'a, A>>::Inner: Into<A>,
{
    outer.fmap(convert_inner)
}

pub fn iterable_try_into<T, C, R>(input: C) -> Result<R, <C::Item as TryInto<T>>::Error>
where
    C: IntoIterator,
    C::Item: TryInto<T>,
    R: FromIterator<T>,
{
    input.into_iter().map(|item| item.try_into()).collect()
}
