pub trait Transaction {
    fn hash(&self) -> String;
}
