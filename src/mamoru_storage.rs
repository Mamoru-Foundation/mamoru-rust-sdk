use crate::component::guest::mamoru_storage;

pub struct KvStorage {
    connection_id: i32,
}

impl KvStorage {
    pub fn new(connection_id: i32) -> Self {
        Self { connection_id }
    }
    fn open() -> Self {
        KvStorage { connection_id: -1 }
    }
    fn open_external(external_agent_id: String) -> Self {
        let connection_id = mamoru_storage::open_external(&external_agent_id);
        KvStorage { connection_id }
    }

    /// Checks if the key exists.
    pub fn contains(&self, _key: String) -> bool {
        false
    }

    /// Returns keys with the given prefix
    ///
    /// Parameters:
    /// - `prefix`: Prefix to use for filtering
    ///
    /// Returns
    pub fn keys(_prefix: String) -> Vec<String> {
        Vec::new()
    }

    /// Get the value for a given `key`
    pub fn get(&self, key: String) -> Option<mamoru_storage::ValueData> {
        mamoru_storage::get(self.connection_id, key.as_str())
    }
    //TODO add default for ttl
    /// Set a value with the given `key`
    pub fn set(
        &self,
        key: String,
        value: mamoru_storage::ValueData,
        ttl: u32,
    ) -> Option<mamoru_storage::ValueData> {
        mamoru_storage::set(key.as_str(), &value, ttl)
    }
}

/// Open a KV storage for the current agent
pub fn open() -> KvStorage {
    KvStorage::open()
}

/// Opens a KV storage for the external agent id
/// The storage is read-only
///
/// Parameters:
/// -`external_agent_id`: External agent identifier
///
/// Returns:
/// KVStorage for agent
pub fn open_external(external_agent_id: String) -> KvStorage {
    KvStorage::open_external(external_agent_id)
}
