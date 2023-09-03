use alloc::boxed::Box;
use cm::CmWorkQueue;
use core::pin::Pin;
use core::{cell::UnsafeCell, marker, mem, ptr};
use macros::vtable;
use mcg::McgWorkQueue;
use qp::QpWorkQueue;

use crate::error::{code::*, Error, Result};
use crate::str::CStr;
use crate::workqueue::{BoxedQueue, Queue};
use crate::{bindings, pr_err, pr_info};

use mlx4_ib::{CounterIndex, Mlx4IbDev, Mlx4IbIboe};

mod cm;
mod mcg;
mod mlx4_ib;
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
}

#[vtable]
pub trait Mlx4Operation {
    fn add(dev: *mut bindings::mlx4_dev) -> Result {
        let mut ibdev: Mlx4IbDev = Mlx4IbDev::new();
        let mut num_ports: i32 = 0;
        let i: i32 = 0;
        let j: i32 = 0;
        let err: i32 = 0;
        //let iboe:*mut Mlx4IbIboe=ptr::null_mut();
        let ib_num_ports: i32 = 0;
        let num_req_counters: i32 = 0;
        let allocated: i32 = 0;
        let counter_index: u32 = 0;
        let new_counter_index: *mut CounterIndex = ptr::null_mut();

        unsafe {
            for i in 1..=(*dev).caps.num_ports {
                if (*dev).caps.port_mask[i as usize] == 1 || (*dev).caps.port_mask[i as usize] == 2
                {
                    num_ports += 1;
                }
            }
        }

        if num_ports == 0 {
            return Err(EINVAL);
        }

        let ib_dev_tmp: *mut bindings::ib_device =
            unsafe { bindings::_ib_alloc_device(mem::size_of::<bindings::ib_device>()) };
        if unsafe { ib_dev_tmp == ptr::null_mut() } {
            pr_err!("Device struct alloc failed\n");
            return Err(EINVAL);
        } else {
            unsafe { ibdev.ib_dev = Some(*ib_dev_tmp) };
        }

        //iboe
        if unsafe { bindings::mlx4_pd_alloc(dev, &mut ibdev.priv_pdn) != 0 } {
            //goto err_dealloc;
        }

        let priv_uar_tmp: *mut bindings::mlx4_uar = ptr::null_mut();
        if unsafe { bindings::mlx4_uar_alloc(dev, priv_uar_tmp) != 0 } {
            //goto err_pd;
        } else {
            unsafe { ibdev.priv_uar = Some(*priv_uar_tmp) };
        }

        let uar_map_tmp =
            unsafe { bindings::ioremap(ibdev.priv_uar.as_ref().unwrap().pfn << 12, 4096) };
        if unsafe { uar_map_tmp == ptr::null_mut() } {
            //goto err_uar;
        } else {
            unsafe { ibdev.uar_map = Some(uar_map_tmp) };
        }

        // uar_lock

        ibdev.dev = Some(dev);
        ibdev.bond_next_port = 0;
        let ib_dev_tmp_ref = ibdev.ib_dev.as_mut().unwrap();
        ib_dev_tmp_ref.node_type = bindings::RDMA_NODE_IB_CA as u8;
        unsafe {
            ib_dev_tmp_ref.local_dma_lkey = (*dev).caps.reserved_lkey;
        }
        ibdev.num_ports = num_ports;
        ib_dev_tmp_ref.phys_port_cnt =
            if (unsafe { (*dev).flags & (bindings::MLX4_FLAG_BONDED as u64) }) != 0 {
                1
            } else {
                ibdev.num_ports as u32
            };
        unsafe {
            ib_dev_tmp_ref.num_comp_vectors = (*dev).caps.num_comp_vectors;
        }
        unsafe {
            ib_dev_tmp_ref.__bindgen_anon_1.dev.parent = &mut (*(*(*dev).persist).pdev).dev;
        }

        // 添加新的操作
        // bindings::ib_set_device_ops(&mut ib_dev_tmp_ref,);

        Ok(())
    }
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

pub struct Mlx4IbDevOpsTable<T>(marker::PhantomData<T>);

// impl<T: Mlx4IbDevOps> Mlx4IbDevOpsTable<T> {
//     pub fn build() -> *mut bindings::ib_device_ops {
//         return &mut bindings::ib_device_ops {
//             owner:ptr::null_mut(),
//             driver_id:2,
//             uverbs_abi_ver:4,
//             add_gid:
//             siz
//         };
//     }

//     unsafe extern "C" fn add_callback(dev: *mut bindings::mlx4_dev) -> *mut core::ffi::c_void {
//         return ptr::null_mut();
//     }
// }

#[vtable]
pub trait Mlx4IbDevOps {
    fn add_gid();
    fn alloc_mr();
    fn alloc_pd();
    fn alloc_ucontext();
    fn attach_mcast();
    fn create_ah();
    fn create_cq();
    fn create_qp();
    fn create_srq();
    fn dealloc_pd();
    fn dealloc_ucontext();
    fn del_gid();
    fn dereg_mr();
    fn destroy_ah();
    fn destroy_cq();
    fn destroy_qp();
    fn destroy_srq();
    fn detach_mcast();
    fn device_group();
    fn disassociate_ucontext();
    fn drain_rq();
    fn drain_sq();
    fn get_dev_fw_str();
    fn get_dma_mr();
    fn get_link_layer();
    fn get_netdev();
    fn get_port_immutable();
    fn map_mr_sg();
    fn mmap();
    fn modify_cq();
    fn modify_device();
    fn modify_port();
    fn modify_qp();
    fn modify_srq();
    fn poll_cq();
    fn post_recv();
    fn post_send();
    fn post_srq_recv();
    fn process_mad();
    fn query_ah();
    fn query_device();
    fn query_gid();
    fn query_pkey();
    fn query_port();
    fn query_qp();
    fn query_srq();
    fn reg_user_mr();
    fn req_notify_cq();
    fn rereg_user_mr();
    fn resize_cq();
}
