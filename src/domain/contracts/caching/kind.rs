/// The cached value shape used to select an adapter's expiration policy.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CacheKind {
    Individual,
    List,
}
