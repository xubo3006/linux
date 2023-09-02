use crate::error::{code::*, Error, Result};
use crate::workqueue::{BoxedQueue, Queue};
use core::ptr;

pub(crate) struct CmWorkQueue {
    cm_wq: Option<BoxedQueue>,
}

impl CmWorkQueue {
    pub(crate) fn new() -> Self {
        Self { cm_wq: None }
    }

    pub(crate) fn init(&mut self) -> Result {
        let cm_wq_tmp = Queue::try_new(format_args!("mlx4_ib_cm"), 0, 0);
        self.cm_wq = match cm_wq_tmp {
            Ok(cm_wq) => Some(cm_wq),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub(crate) fn clean(&mut self) {
        if self.cm_wq.is_some() {
            drop(self.cm_wq.take().unwrap());
        }
    }
}
