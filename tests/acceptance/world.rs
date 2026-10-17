//! What one scenario remembers between its steps.

use std::path::PathBuf;

use cucumber::World;
use rusqlite::Connection;
use tempfile::TempDir;

#[derive(Debug, World)]
#[world(init = Self::new)]
pub struct TaxiWorld {
    dir: TempDir,
    pub built: Option<Result<(), String>>,
}

impl TaxiWorld {
    fn new() -> Self {
        let dir = TempDir::new().expect("temp dir");
        Self { dir, built: None }
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.dir.path().join(name)
    }

    pub fn db(&self) -> Connection {
        Connection::open(self.path("taxi.db")).expect("open taxi.db")
    }
}
