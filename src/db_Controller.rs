use sled;

struct Database {
    db: sled::Db,
}

impl Database {
    fn new(db: sled::Db) -> Self {
        Database { db }
    }

    fn store(&self, id: &str, data: &str) -> Result<(), sled::Error> {
        self.db.insert(id, data)?;
        Ok(())
    }
}