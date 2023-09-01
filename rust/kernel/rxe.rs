

use rxe_verbs::RxeDev;
use rxe_net::RxeRecvSockets;
use core::pin::Pin;
use core::ptr;
use alloc::boxed::Box;

use crate::{bindings, pr_info};
use crate::error::{code::*,Error, Result};
use crate::str::CStr;
use crate::str::CString;


mod rxe_verbs;
mod rxe_hdr;
mod rxe_net;
mod rxe_pool;
mod rxe_recv;

/// Soft RDMA transport registration.
///
pub struct Registration {
    registered: bool,
    name:&'static CStr,
    //rxe_dev: RxeDev,
    net_socket: RxeRecvSockets,
    rxe_link_ops: bindings::rdma_link_ops,
    //再包一层
    //_pin: PhantomPinned,

    // /// Context initialised on construction and made available to all file instances on
    // /// [`file::Operations::open`].
    //open_data: MaybeUninit<T::OpenData>,
}

impl Registration{
    pub fn new(name: &'static CStr) ->Self {
        Self{
            registered:false,
            name,
            net_socket:RxeRecvSockets::new(),
            rxe_link_ops:bindings::rdma_link_ops::default(),
        }
    }
    
    pub fn new_pinned(name: &'static CStr) ->Result<Pin<Box<Self>>>{
        let mut r=Pin::from(Box::try_new(Self::new(name))?);
        r.as_mut().register()?;
        Ok(r)
    }

    pub fn register(
        self: Pin<&mut Self>,

    )-> Result{
        let this=unsafe{self.get_unchecked_mut()};
        // todo self.registered相关

        //先注册rex_net相关
        match this.net_socket.alloc(){
            Ok(())=>{},
            Err(e)=>return Err(e),
        }

        // rdma link register
        this.rxe_link_ops=RxeRdmaLinkTable::build();
        unsafe{
            bindings::rdma_link_register(&mut this.rxe_link_ops);
        }
        pr_info!("loaded");
        Ok(())
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        if self.registered{
            unsafe{bindings::rdma_link_unregister(&mut self.rxe_link_ops)};
            unsafe{bindings::ib_unregister_driver(bindings::rdma_driver_id_RDMA_DRIVER_RXE)};
        }
    }
}

pub struct RxeRdmaLinkTable;

impl RxeRdmaLinkTable {
    pub fn build()->bindings::rdma_link_ops {
        Self::RXELINKFUNC
    }

    const RXELINKFUNC:bindings::rdma_link_ops=bindings::rdma_link_ops {
        type_:"rxe".as_ptr() as *const i8,
        newlink:Some(Self::rxe_newlink),
        list:bindings::list_head{
            next:ptr::null_mut(),
            prev:ptr::null_mut(),
        },
    };

    unsafe extern "C" fn rxe_newlink(
        ibdev_name: *const core::ffi::c_char,
        ndev: *mut bindings::net_device,
    ) -> core::ffi::c_int {
        // todo
        return 0
    }
}