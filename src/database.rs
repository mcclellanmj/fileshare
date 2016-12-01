use rusqlite::Connection;
use rusqlite::types::ToSql;

use std::sync::{Arc, Mutex};

use time;
use time::Duration;

pub struct ShareDatabase {
    connection: Arc<Mutex<Connection>>
}

pub enum ShareResult<T> {
    Missing,
    Expired,
    Valid(T)
}

impl ShareDatabase {
    pub fn new(database_location: &str) -> ShareDatabase {
        let connection = Connection::open(database_location).unwrap();
        {
            let mut stmt = connection.prepare("SELECT name FROM sqlite_master WHERE type='table' and name='shared_files'").unwrap();
            let table_exists = stmt.exists(&[]).unwrap();

            if !table_exists {
                connection.execute("CREATE TABLE shared_files(\
                    id INTEGER PRIMARY KEY AUTOINCREMENT,\
                    hash CHAR(36) NOT NULL UNIQUE,\
                    expiration TIMESTAMP NOT NULL,\
                    path VARCHAR(32768))", &[]).unwrap();

                connection.execute("CREATE UNIQUE INDEX shared_files_hash_index on shared_files (hash)", &[]).unwrap();
            }
        }

        ShareDatabase {
            connection: Arc::new(Mutex::new(connection))
        }
    }

    pub fn get_shared_by_hash<T: ToSql>(&self, hash: &T) -> ShareResult<String> {
        let connection = self.connection.lock().unwrap();

        let mut stmt = connection.prepare("SELECT path, expiration FROM shared_files WHERE hash=:hash").unwrap();
        let mut rows = stmt.query_named(&[(":hash", hash)]).unwrap();

        if let Some(r) = rows.next() {
            let result = r.unwrap();
            let expiration : time::Timespec = result.get(1);

            if expiration >= time::get_time() {
                ShareResult::Valid(result.get(0))
            } else {
                ShareResult::Expired
            }
        } else {
            ShareResult::Missing
        }
    }

    pub fn add_shared_file<T: ToSql>(&self, hash: &T, filepath: &T) -> i32 {
        let connection = self.connection.lock().unwrap();
        let expiration = time::get_time() + Duration::days(7);

        connection.execute_named("INSERT INTO shared_files(hash, path, expiration) VALUES (:hash, :path, :expiration)", &[(":hash", hash), (":path", filepath), (":expiration", &expiration)]).unwrap()
    }
}