use graphics_rs::core::entity::Identifier;
use graphics_rs::view_port::ViewPort;
use std::sync::{Arc, LockResult, RwLock, RwLockReadGuard, RwLockWriteGuard};

#[derive(Clone)]
pub struct AtomicViewPort<Id> {
    internal: Arc<RwLock<ViewPort<Id>>>,
}
impl<Id: Identifier> AtomicViewPort<Id> {
    pub fn new(id: Id) -> Self {
        Self {
            internal: Arc::new(RwLock::new(ViewPort::new(id))),
        }
    }

    pub fn read(&self) -> LockResult<RwLockReadGuard<ViewPort<Id>>> {
        self.internal.read()
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<ViewPort<Id>>> {
        self.internal.write()
    }
}
