use graphics_rs::view_port::ViewPort;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct AtomicViewPort {
    internal: Arc<RwLock<ViewPort>>,
}

impl AtomicViewPort {
    pub fn new() -> Self {
        Self {
            internal: Arc::new(RwLock::new(ViewPort::new())),
        }
    }

    pub fn read(&self) -> LockResult<RwLockReadGuard<ViewPort>> {
        self.internal.read()
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<ViewPort>> {
        self.internal.write()
    }
}
