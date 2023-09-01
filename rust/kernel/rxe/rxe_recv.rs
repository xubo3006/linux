use super::rxe_hdr::{RxeHdrLength,skb_to_pkt};
use super::rxe_verbs::RxeDev;
use crate::error::{code::*,Error, Result};
use core::ptr;

pub fn rxe_rcv(skb:*mut bindings::sk_buff) {
    let mut err:u8=0;
    let pkt:*mut RxePktInfo=unsafe{skb_to_pkt(skb)};
    let rxe:RxeDev=unsafe{(*pkt).rxe};    
    
    if unsafe{(*skb).len<RxeHdrLength::RxeBthBytes} {
        rxe_rcv_drop(skb, pkt, rxe)
    }

    if rxe_chk_dgid(){

    }

}

fn rxe_rcv_drop(skb:*mut bindings::sk_buff,pkt:*mut RxePktInfo,rxe:*const RxeDev) {
    // if (*pkt).qp 
    //     (*pkt).qp.elem
    bindings::kfree_skb(skb,2);
    unsafe{
        bindings::ib_device_put((*rxe).ib_device);
    }
}

fn rxe_chk_dgid(rxe:RxeDev,skb:*mut bindings::sk_buff) ->Result{
    let pkt:*mut RxePktInfo=unsafe{skb_to_pkt(skb)};
    let gid_attr:*mut bindings::ib_gid_attr=ptr::null_mut();
    let dgid:bindings::ib_gid=bindings::ib
    let pdgid:*mut bindings::ib_gid=ptr::null_mut();

}

#[link(name="add")]
