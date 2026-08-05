use portable_pty::MasterPty;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct PtyState {
    pub masters: Mutex<HashMap<u32, Box<dyn MasterPty + Send>>>,
    pub writers: Mutex<HashMap<u32, Box<dyn std::io::Write + Send>>>,
}

impl PtyState {
    pub fn new() -> Self {
        Self {
            masters: Mutex::new(HashMap::new()),
            writers: Mutex::new(HashMap::new()),
        }
    }
}
