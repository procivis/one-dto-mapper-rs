use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::hash::Hash;

/// Simplified Traversable typeclass that only supports [`Result`] as the outermost effect.
pub trait TraversableResult<B> {
    type Inner;
    type Mapped: TraversableResult<B, Inner = B, Mapped = Self::Mapped>
        + TraversableResult<Self::Inner, Inner = B, Mapped = Self>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>;
}

impl<A, B, K> TraversableResult<B> for BTreeMap<K, A>
where
    K: Ord,
{
    type Inner = A;
    type Mapped = BTreeMap<K, B>;

    fn traverse<F, Error>(self, mut f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<A, B> TraversableResult<B> for BTreeSet<A>
where
    A: Ord,
    B: Ord,
{
    type Inner = A;
    type Mapped = BTreeSet<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> TraversableResult<B> for BinaryHeap<A>
where
    A: Ord,
    B: Ord,
{
    type Inner = A;
    type Mapped = BinaryHeap<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B, K> TraversableResult<B> for HashMap<K, A>
where
    K: Eq + Hash,
{
    type Inner = A;
    type Mapped = HashMap<K, B>;

    fn traverse<F, Error>(self, mut f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter()
            .map(|(k, v)| f(v).map(|v| (k, v)))
            .collect()
    }
}

impl<A, B> TraversableResult<B> for HashSet<A>
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    type Inner = A;
    type Mapped = HashSet<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> TraversableResult<B> for LinkedList<A> {
    type Inner = A;
    type Mapped = LinkedList<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> TraversableResult<B> for Option<A> {
    type Inner = A;
    type Mapped = Option<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.map(f).transpose()
    }
}

impl<A, B, E> TraversableResult<B> for Result<A, E> {
    type Inner = A;
    type Mapped = Result<B, E>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        match self.map(f) {
            Ok(Ok(v)) => Ok(Ok(v)),
            Ok(Err(err)) => Err(err),
            Err(err) => Ok(Err(err)),
        }
    }
}

impl<A, B> TraversableResult<B> for Vec<A> {
    type Inner = A;
    type Mapped = Vec<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter().map(f).collect()
    }
}

impl<A, B> TraversableResult<B> for VecDeque<A> {
    type Inner = A;
    type Mapped = VecDeque<B>;

    fn traverse<F, Error>(self, f: F) -> Result<Self::Mapped, Error>
    where
        F: FnMut(Self::Inner) -> Result<B, Error>,
    {
        self.into_iter().map(f).collect()
    }
}
