use crate::error::{code::*, Error, Result};
use crate::workqueue::{BoxedQueue, Queue};
use core::ptr;

pub(crate) struct QpWorkQueue {
    mlx4_ib_qp_event_wq: Option<BoxedQueue>,
}

impl QpWorkQueue {
    pub(crate) fn new() -> Self {
        Self {
            mlx4_ib_qp_event_wq: None,
        }
    }

    pub(crate) fn init(&mut self) -> Result {
        let mlx4_ib_qp_event_wq_tmp =
            Queue::try_new(format_args!("mlx4_ib_qp_event_wq"), 655361, 1);
        self.mlx4_ib_qp_event_wq = match mlx4_ib_qp_event_wq_tmp {
            Ok(mlx4_ib_qp_event_wq) => Some(mlx4_ib_qp_event_wq),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub(crate) fn clean(&mut self) {
        if self.mlx4_ib_qp_event_wq.is_some() {
            drop(self.mlx4_ib_qp_event_wq.take().unwrap());
        }
    }
}
