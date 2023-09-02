use crate::error::{code::*, Error, Result};
use crate::workqueue::{BoxedQueue, Queue};
use core::ptr;

pub(crate) struct McgWorkQueue {
    clean_wq: Option<BoxedQueue>,
}

impl McgWorkQueue {
    pub(crate) fn new() -> Self {
        Self { clean_wq: None }
    }

    pub(crate) fn init(&mut self) -> Result {
        let clean_wq_tmp = Queue::try_new(format_args!("mlx4_ib_mcg"), 655369, 1);
        self.clean_wq = match clean_wq_tmp {
            Ok(clean_wq) => Some(clean_wq),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub(crate) fn clean(&mut self) {
        if self.clean_wq.is_some() {
            drop(self.clean_wq.take().unwrap());
        }
    }
}
