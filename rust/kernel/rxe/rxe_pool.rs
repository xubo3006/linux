use rxe_dev::RxeDev;

pub(crate) enum RxeElemType {
    RxeTypeUc=0,
    RxeTypePd=1,
    RxeTypeAh=2,
    RxeTypeSrq=3,
    RxeTypeQp=4,
    RxeTypeCq=5,
    RxeTypeMr=6,
    RxeTypeMw=7,
    RxeNumTypes=8,
}

pub(crate) struct RxePoolElem{
    pool:RxePool,
    obj:*mut u8,
    ref_cnt:bindings::kref,
    list:bindings::list_head,
    complete:bindings::completion,
    index:u32,
}

pub(crate) struct RxePool{
    rxe:RxeDev,
    name:bindings::char,
    cleanup:Option<fn(*mut elem)>,
    types:RxeElemType,

    max_elem:bindings::u_int,
    num_elem:bindings::atomic_t,
    elem_size:usize,
    elem_offset:usize,
    
    xa:bindings::xarray,
    xa:bindings::xa_limit,
    nest:u32,
}
