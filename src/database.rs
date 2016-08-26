use rusqlite::Connection;
use rusqlite::types::ToSql;
use std::path::{Path, PathBuf};

use std::sync::{Arc, Mutex};

pub struct ShareDatabase {
    connection: Arc<Mutex<Connection>>
}

impl ShareDatabase {
    pub fn new(database_location: &str) -> ShareDatabase {
        let connection = Connection::open(database_location).unwrap();
        {
            let mut stmt = connection.prepare("SELECT name FROM sqlite_master WHERE type='table' and name='shared_files'").unwrap();
            let table_exists = stmt.exists(&[]).unwrap();

            if !table_exists {
                connection.execute("CREATE TABLE shared_files(
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    hash CHAR(36) UNIQUE,
                    path VARCHAR(32768))", &[]).unwrap();

                connection.execute("CREATE UNIQUE INDEX shared_files_hash_index on shared_files (hash)", &[]).unwrap();
            }
        }

        ShareDatabase {
            connection: Arc::new(Mutex::new(connection))
        }
    }

    pub fn get_shared_by_hash<T: ToSql>(&self, hash: &T) -> Option<PathBuf> {
        let connection = self.connection.lock().unwrap();

        let mut stmt = connection.prepare("SELECT path FROM shared_files WHERE hash=:hash").unwrap();
        let mut rows = stmt.query_named(&[(":hash", hash)]).unwrap();

        if let Some(r) = rows.next() {
            let result = r.unwrap();
            let path: String = result.get(0);
            Some(Path::new(&path).to_owned())
        } else {
            None
        }
    }

    pub fn add_shared_file<T: ToSql>(&self, hash: &T, filepath: &T) -> i32 {
        let connection = self.connection.lock().unwrap();
        connection.execute_named("INSERT INTO shared_files(hash, path) VALUES (:hash, :path)", &[(":hash", hash), (":path", filepath)]).unwrap()
    }
}