pub type HashMap<K, V> = std::collections::HashMap<K, V, ahash::RandomState>;
pub type HashSet<K> = std::collections::HashSet<K, ahash::RandomState>;
pub type DashSet<K> = dashmap::DashSet<K, ahash::RandomState>;
pub type DashMap<K, V> = dashmap::DashMap<K, V, ahash::RandomState>;

pub type BiMap<L, R> = bimap::BiHashMap<L, R, ahash::RandomState, ahash::RandomState>;
