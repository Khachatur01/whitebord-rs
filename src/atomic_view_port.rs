use std::ops::Deref;
use graphics_rs::view_port::ViewPort;
use std::sync::{Arc, LockResult, PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

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

    pub fn read_ref<R, F: FnOnce(RwLockReadGuard<'_, ViewPort>) -> R>(&self, predicate: F) -> Result<R, PoisonError<RwLockReadGuard<ViewPort>>> {
        let view_port: RwLockReadGuard<ViewPort> = self.internal.read()?;
        Ok(predicate(view_port))
    }

    pub fn write_ref<R, F: FnOnce(RwLockWriteGuard<'_, ViewPort>) -> R>(&self, predicate: F) -> Result<R, PoisonError<RwLockWriteGuard<ViewPort>>> {
        let view_port: RwLockWriteGuard<ViewPort> = self.internal.write()?;
        Ok(predicate(view_port))
    }
}
