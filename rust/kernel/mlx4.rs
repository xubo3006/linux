use alloc::boxed::Box;
use cm::CmWorkQueue;
use core::pin::Pin;
use core::{cell::UnsafeCell, marker, ptr};
use macros::vtable;
use mcg::McgWorkQueue;
use qp::QpWorkQueue;

use crate::error::{code::*, Error, Result};
use crate::str::CStr;
use crate::workqueue::{BoxedQueue, Queue};
use crate::{bindings, pr_info};

mod cm;
mod mcg;
mod qp;

/// Soft RDMA transport registration.
///
pub struct Registration<T: Mlx4Operation> {
    registered: bool,
    name: &'static CStr,

    wq: Mlx4WorkQueue,
    cm_wq: CmWorkQueue,
    qp_wq: QpWorkQueue,
    mcg_wq: McgWorkQueue,
    phantom: marker::PhantomData<T>,
    //rxe_link_ops: bindings::rdma_link_ops,
    //再包一层
    //_pin: PhantomPinned,

    // /// Context initialised on construction and made available to all file instances on
    // /// [`file::Operations::open`].
    //open_data: MaybeUninit<T::OpenData>,
}

impl<T: Mlx4Operation> Registration<T> {
    pub fn new(name: &'static CStr) -> Self {
        Self {
            registered: false,
            name,
            wq: Mlx4WorkQueue::new(),
            cm_wq: CmWorkQueue::new(),
            qp_wq: QpWorkQueue::new(),
            mcg_wq: McgWorkQueue::new(),
            phantom: marker::PhantomData,
            //rxe_link_ops:bindings::rdma_link_ops::default(),
        }
    }

    pub fn new_pinned(name: &'static CStr) -> Result<Pin<Box<Self>>> {
        let mut r = Pin::from(Box::try_new(Self::new(name))?);
        r.as_mut().register()?;
        Ok(r)
    }

    pub fn register(self: Pin<&mut Self>) -> Result {
        let this = unsafe { self.get_unchecked_mut() };

        match this.wq.init() {
            Ok(()) => {}
            Err(e) => return Err(e),
        }

        match this.qp_wq.init() {
            Ok(()) => {}
            Err(e) => {
                this.wq.clean();
                return Err(e);
            }
        }

        match this.cm_wq.init() {
            Ok(()) => {}
            Err(e) => {
                this.wq.clean();
                this.qp_wq.clean();
                return Err(e);
            }
        }

        match this.mcg_wq.init() {
            Ok(()) => {}
            Err(e) => {
                this.wq.clean();
                this.cm_wq.clean();
                this.qp_wq.clean();
                return Err(e);
            }
        }

        // interface用vtable替换掉

        unsafe {
            bindings::mlx4_register_interface(Mlx4OperationTable::<T>::build());
        }

        this.registered = true;
        Ok(())
    }
}

impl<T: Mlx4Operation> Drop for Registration<T> {
    fn drop(&mut self) {
        if self.registered {
            //unsafe{bindings::mlx4_unregister_interface();}
            self.mcg_wq.clean();
            self.cm_wq.clean();
            self.qp_wq.clean();
            self.wq.clean();
        }
    }
}

pub struct Mlx4OperationTable<T>(marker::PhantomData<T>);

impl<T: Mlx4Operation> Mlx4OperationTable<T> {
    pub fn build() -> *mut bindings::mlx4_interface {
        return &mut bindings::mlx4_interface {
            add: Some(Self::add_callback),
            remove: Some(Self::remove_callback),
            event: Some(Self::event_callback),
            get_dev: None,
            activate: None,
            list: bindings::list_head {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
            // MLX4_PROT_IB_IPV6
            protocol: 0,
            // MLX4_INTFF_BONDING
            flags: 1,
        };
    }

    unsafe extern "C" fn add_callback(dev: *mut bindings::mlx4_dev) -> *mut core::ffi::c_void {
        return ptr::null_mut();
    }

    unsafe extern "C" fn remove_callback(
        dev: *mut bindings::mlx4_dev,
        context: *mut core::ffi::c_void,
    ) {
    }

    unsafe extern "C" fn event_callback(
        dev: *mut bindings::mlx4_dev,
        context: *mut core::ffi::c_void,
        event: bindings::mlx4_dev_event,
        param: core::ffi::c_ulong,
    ) {
    }

    // unsafe extern "C" fn get_dev_callback(
    //     dev: *mut mlx4_dev,
    //     context: *mut core::ffi::c_void,
    //     port: u8_,
    // ) -> *mut core::ffi::c_void {
    // }

    // unsafe extern "C" fn activate_callback(
    //     dev: *mut mlx4_dev,
    //     context: *mut core::ffi::c_void
    // ) {
    // }

    // MLX4FUNC:bindings::mlx4_interface=bindings::mlx4_interface {
    //     add:Some(Self::add_callback),
    //     remove:Some(Self::remove_callback),
    //     event:Some(Self::event_callback),
    //     get_dev:None,
    //     activate:None,
    //     list:bindings::list_head{next:ptr::null_mut(),prev:ptr::null_mut()},
    //     // MLX4_PROT_IB_IPV6
    //     protocol:0,
    //     // MLX4_INTFF_BONDING
    //     flags:1,
    // };
}

#[vtable]
pub trait Mlx4Operation {
    fn add();
    fn remove();
    fn event();
    // fn get_dev();
    // fn activate();
}

pub(crate) struct Mlx4WorkQueue {
    wq: Option<BoxedQueue>,
}

impl Mlx4WorkQueue {
    pub(crate) fn new() -> Self {
        Self { wq: None }
    }

    pub(crate) fn init(&mut self) -> Result {
        let wq_tmp = Queue::try_new(format_args!("mlx4_ib"), 655369, 1);
        self.wq = match wq_tmp {
            Ok(wq) => Some(wq),
            Err(e) => return Err(e),
        };

        Ok(())
    }

    pub(crate) fn clean(&mut self) {
        if self.wq.is_some() {
            drop(self.wq.take().unwrap());
        }
    }
}
