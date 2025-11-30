use std::collections::BTreeMap;
use std::sync::RwLock;

// basic k, v types. raw bytes for db
pub type Key = Vec<u8>
pub type Value = Vec<u8>

// versioning for later, currently early stage wrapper
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sequence(pub u64)

// errors and results
#[derive(Debug)]
pub enum KvError {
    NotFound,
    Io(std::io::Error),
    Corruption(String),
    // TODO: extend with more contexts
}

pub type Result<T> = std::result::Result<T, KvError>;

// Allow `?` on std::io::Error in the future.
impl From<std::io::Error> for KvError {
    fn from(e: std::io::Error) -> Self {
        KvError::Io(e)
    }
}

// ENGINE CORE
// can be shared across threads and survive whole program
pub trait StorageEngine: Send + Sync + 'static {
    /// - Ok(Some(value)) if present
    /// - Ok(None) if key is missing
    /// - Err(_) for real errors (I/O, corruption, etc.)
    fn get(&self, key: &[u8]) -> Result<Option<Value>>;

    /// insert/overwrite
    fn put(&self, key: Key, value: Value) -> Result<()>;

    /// deleting non-existent key = no-op
    fn delete(&self, key: &[u8]) -> Result<()>;

    /// range scan over [start, end) in lexicographic byte order
    ///
    /// in mem: turn into Vec and return iterator
    /// on disk: stream from SSTables/b+tree
    fn scan(
        &self,
        start: &[u8],
        end: &[u8],
    ) -> Result<Box<dyn Iterator<Item = (Key, Value)> + Send>>;
}
