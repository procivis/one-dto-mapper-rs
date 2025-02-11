mod traversable_result;

use fmap::Functor;
pub use one_dto_mapper_derive::*;
pub use traversable_result::TraversableResult;

/// Converts the inner value of a container ([`Vec`], [`Option`] or other) using the [`Into::into`] function.
pub fn convert_inner<'a, T, A>(outer: T) -> T::Mapped
where
    T: Functor<'a, A>,
    T::Inner: Into<A>,
{
    outer.fmap(Into::into)
}

/// Same as [`convert_inner`], but works for deeper nested containers such as Option<Vec<A>>
pub fn convert_inner_of_inner<'a, T, K, A: 'a>(outer: T) -> T::Mapped
where
    T: Functor<'a, K>,
    T::Inner: Functor<'a, A, Mapped = K>,
    <T::Inner as Functor<'a, A>>::Inner: Into<A>,
{
    outer.fmap(convert_inner)
}

/// Converts value into an iterator, then applies [`Into::into`] to each item and collects items back to the requested type
pub fn iterable_into<T, C, R>(input: C) -> R
where
    C: IntoIterator,
    C::Item: Into<T>,
    R: FromIterator<T>,
{
    input.into_iter().map(Into::into).collect()
}

/// Same as [`convert_inner`], but will apply [`TryInto::try_into`] instead
pub fn try_convert_inner<T, A>(outer: T) -> Result<T::Mapped, <T::Inner as TryInto<A>>::Error>
where
    T: TraversableResult<A>,
    T::Inner: TryInto<A>,
{
    outer.traverse(TryInto::try_into)
}

/// Same as [`convert_inner_of_inner`], but will apply [`TryInto::try_into`] instead
#[allow(clippy::type_complexity)]
pub fn try_convert_inner_of_inner<T, K, A>(
    outer: T,
) -> Result<T::Mapped, <<T::Inner as TraversableResult<A>>::Inner as TryInto<A>>::Error>
where
    T: TraversableResult<K>,
    T::Inner: TraversableResult<A, Mapped = K>,
    <T::Inner as TraversableResult<A>>::Inner: TryInto<A>,
{
    outer.traverse(|intermediate| intermediate.traverse(TryInto::try_into))
}

/// Same as [`iterable_into`], but will apply [`TryInto::try_into`] instead
pub fn iterable_try_into<T, C, R>(input: C) -> Result<R, <C::Item as TryInto<T>>::Error>
where
    C: IntoIterator,
    C::Item: TryInto<T>,
    R: FromIterator<T>,
{
    input.into_iter().map(TryInto::try_into).collect()
}
