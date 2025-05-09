use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};
use graphics_rs::core::entity::Id;
use graphics_rs::view_port::ViewPort;

#[derive(Clone)]
pub struct AtomicViewPort {
    internal: Arc<RwLock<ViewPort>>,
}
impl AtomicViewPort {
    pub fn new(id: impl Id + 'static) -> Self {
        Self {
            internal: Arc::new(RwLock::new(ViewPort::new(id))),
        }
    }

    pub fn read(&self) -> LockResult<RwLockReadGuard<ViewPort>> {
        self.internal.read()
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<ViewPort>> {
        self.internal.write()
    }
}
