use std::{
    collections::hash_map::DefaultHasher,
    fs::{self, Metadata},
    hash::{Hash, Hasher},
    marker::PhantomData,
    path::PathBuf,
};

use glob::{glob, Paths};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

use super::{AdapterError, StoreAdapter};

struct JsonFileStoreAdapter<K, V> {
    dirpath: PathBuf,
    key: PhantomData<K>,
    value: PhantomData<V>,
}

impl<K, V> JsonFileStoreAdapter<K, V>
where
    K: Hash + PartialEq,
    V: Serialize + DeserializeOwned,
{
    fn create_dir(dirpath: &PathBuf) -> Result<(), AdapterError> {
        std::fs::create_dir_all(dirpath)
            .map_err(|e| AdapterError::InitializationError(e.into()))?;

        Ok(())
    }
    pub fn new(dirpath: PathBuf) -> Result<Self, AdapterError> {
        if !dirpath.exists() {
            Self::create_dir(&dirpath)?;
        }

        Ok(Self {
            dirpath: dirpath,
            key: PhantomData,
            value: PhantomData,
        })
    }

    fn get_filename_from_key(&self, key: &K) -> PathBuf {
        let filename = {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            hasher.finish()
        };
        let filename = format!("{}.json", filename);

        self.dirpath.join(filename)
    }

    fn get_last_hash(&self, nth: usize) -> Result<Vec<String>, AdapterError> {
        let mut hashes: Vec<(String, Metadata)> = Vec::new();
        let glob_expr = format!("{}/*.key", &self.dirpath.to_str().unwrap());

        for entry in glob(&glob_expr).map_err(|e| AdapterError::OpeningStreamError(e.into()))? {
            let path = entry.map_err(|e| AdapterError::OpeningStreamError(e.into()))?;
            let metadata =
                fs::metadata(&path).map_err(|e| AdapterError::OpeningStreamError(e.into()))?;
            hashes.push((
                path.as_path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                metadata,
            ));
        }
        hashes.sort_by_key(|(_, meta)| meta.created().unwrap());
        let result = hashes
            .into_iter()
            .rev()
            .take(nth)
            .map(|(hash, _meta)| hash)
            .collect();

        Ok(result)
    }
}
impl<K, V> StoreAdapter for JsonFileStoreAdapter<K, V>
where
    K: Hash + PartialEq + Serialize + DeserializeOwned,
    V: Serialize + DeserializeOwned,
{
    type Key = K;
    type Record = V;

    /**
     * create (or update) a Value in the Store.
     * When it is created, a key file with the same Hash as the value which
     * contains the actual key the value is associated with.
     */
    fn store_record(&mut self, _key: Self::Key, _record: Self::Record) -> Result<(), AdapterError> {
        todo!()
    }

    /**
     * find and returns the expected value from its Key hash
     */
    fn get_record(&self, key: &Self::Key) -> Result<Option<Self::Record>, AdapterError> {
        if !self.record_exists(key)? {
            return Ok(None);
        }
        let filepath = self.get_filename_from_key(key);
        let value = std::fs::read_to_string(filepath)
            .map_err(|e| AdapterError::OpeningStreamError(e.into()))?;
        let record: V =
            serde_json::from_str(&value).map_err(|e| AdapterError::ParsingDataError(e.into()))?;

        Ok(Some(record))
    }

    /**
     * simple implementation
     * if the file exists, then the document exists
     */
    fn record_exists(&self, key: &Self::Key) -> Result<bool, AdapterError> {
        Ok(self.get_filename_from_key(key).is_file())
    }

    /**
     * get last N records is fairly complex, read the directory to return the
     * most recent elements only. This implies being able to sort on creation?
     * modification? date and be able to get the Key (not its hash)
     */
    fn get_last_n_records(
        &self,
        how_many: usize,
    ) -> Result<Vec<(Self::Key, Self::Record)>, AdapterError> {
        let hashes = self.get_last_hash(how_many)?;
        let mut records: Vec<(K, V)> = vec![];

        for hash in hashes {
            let filename = format!("{}.key", hash);
            let content = std::fs::read_to_string(self.dirpath.join(filename))
                .map_err(|e| AdapterError::OpeningStreamError(e.into()))?;
            let key: K = serde_json::from_str(&content)
                .map_err(|e| AdapterError::ParsingDataError(e.into()))?;
            let record = self.get_record(&key)?.unwrap();
            // panic if no value file is associated to the key
            records.push((key, record));
        }

        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Write, time::Duration};

    use super::*;

    fn get_adapter(dir: &PathBuf) -> JsonFileStoreAdapter<u64, String> {
        JsonFileStoreAdapter::new((*dir).clone()).unwrap()
    }

    fn get_pathbuf() -> PathBuf {
        std::env::temp_dir().join("mithril_test")
    }

    fn init_dir(dir: &PathBuf) {
        for (idx, hash, msg) in [
            (1, "2206609067086327257", "one"),
            (2, "11876854719037224982", "two"),
            (3, "18270091135093349626", "three"),
        ] {
            let value_filename = format!("{}.json", hash);
            let key_filename = format!("{}.key", hash);
            let mut file = fs::File::create(dir.join(value_filename)).unwrap();
            let value = json!(msg);
            file.write_fmt(format_args!("{}", value)).unwrap();
            let mut file = fs::File::create(dir.join(key_filename)).unwrap();
            let value = json!(idx);
            file.write_fmt(format_args!("{}", value)).unwrap();
            std::thread::sleep(Duration::from_millis(100));
        }
    }

    fn rmdir(dir: PathBuf) {
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn check_file_exists() {
        let dir = get_pathbuf().join("check_file_exists");
        let adapter = get_adapter(&dir);
        init_dir(&dir);
        assert!(adapter.record_exists(&1).unwrap());
        rmdir(dir);
    }

    #[test]
    fn check_file_does_not_exist() {
        let dir = get_pathbuf().join("check_file_does_not_exist");
        let adapter = get_adapter(&dir);
        init_dir(&dir);
        assert!(!adapter.record_exists(&4).unwrap());
        rmdir(dir);
    }

    #[test]
    fn check_get_record() {
        let dir = get_pathbuf().join("check_get_record");
        let adapter = get_adapter(&dir);
        init_dir(&dir);
        let content = adapter.get_record(&1).unwrap().unwrap();
        assert_eq!("one", content);
        rmdir(dir);
    }

    #[test]
    fn check_get_last_n() {
        let dir = get_pathbuf().join("check_get_last_n");
        let adapter = get_adapter(&dir);
        init_dir(&dir);
        let values = adapter.get_last_n_records(2).unwrap();
        assert!(values.len() == 2);
        assert_eq!((3, "three".to_string()), values[0]);
        assert_eq!((2, "two".to_string()), values[1]);
        rmdir(dir);
    }
}
