pub trait Hashable {
    type Hash;

    fn hash(&self) -> Self::Hash;
}
