use rxe_verbs::{RxeDev,RxeQp};

pub(crate) struct RxePktInfo {
    pub(crate) rxe:RxeDev,
    qp:RxeQp
}

pub(crate) struct RxeBth {
    opcode:u8,
    flags:u8,
    pkey:bindings::__be16,
    qpn:bindings::__be32,
    apsn:bindings::__be32,
}

pub(crate) fn skb_to_pkt(skb: *mut bindings::sk_buff) -> *mut RxePktInfo {
    unsafe {
        let pkt_info: *mut RxePktInfo = core::mem::transmute((*skb).cb);
        pkt_info
    }
}

pub(crate) enum RxeHdrLength {
    RxeBthBytes= std::mem::size_of::<RxeBth>(),
    //RXE_DETH_BYTES = std::mem::size_of::<RxeDeth>(),
    //RXE_IMMDT_BYTES = std::mem::size_of::<RxeImmdt>(),
    //RXE_RETH_BYTES = std::mem::size_of::<RxeReth>(),
    //RXE_AETH_BYTES = std::mem::size_of::<RxeAeth>(),
    //RXE_ATMACK_BYTES = std::mem::size_of::<RxeAtmack>(),
    //RXE_ATMETH_BYTES = std::mem::size_of::<RxeAtmeth>(),
    //RXE_IETH_BYTES = std::mem::size_of::<RxeIeth>(),
    //RXE_RDETH_BYTES = std::mem::size_of::<RxeRdeth>(),
    //RXE_FETH_BYTES = std::mem::size_of::<RxeFeth>(),
}