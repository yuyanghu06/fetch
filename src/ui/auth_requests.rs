//! Request ownership for independent server authentication and ordered status reads.
use std::collections::BTreeMap;

#[derive(Default)]
pub(super) struct Requests {
    active: BTreeMap<String, u64>,
    sequence: u64,
    revision: u64,
    last_read: u64,
}

impl Requests {
    pub fn begin(&mut self, server: &str) -> Option<u64> {
        if self.contains(server) { return None; }
        self.sequence += 1;
        self.revision += 1;
        self.active.insert(server.to_owned(), self.sequence);
        Some(self.sequence)
    }

    pub fn finish(&mut self, server: &str, token: u64) -> bool {
        if self.active.get(server) != Some(&token) { return false; }
        self.active.remove(server);
        self.revision += 1;
        true
    }

    pub fn contains(&self, server: &str) -> bool { self.active.contains_key(server) }
    pub fn revision(&self) -> u64 { self.revision }
    pub fn latest_sequence(&self) -> u64 { self.sequence }

    pub fn clear(&mut self) {
        self.active.clear();
        self.revision += 1;
    }

    pub fn begin_read(&mut self) -> (u64, u64) {
        self.sequence += 1;
        (self.revision, self.sequence)
    }

    pub fn accept_read(&mut self, revision: u64, sequence: u64) -> bool {
        if revision != self.revision || sequence <= self.last_read { return false; }
        self.last_read = sequence;
        true
    }
}
