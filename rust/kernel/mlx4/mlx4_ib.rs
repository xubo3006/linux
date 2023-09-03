use core::ffi;

pub(crate) struct Mlx4IbDiagCounters {
    descs: *mut bindings::rdma_stat_desc,
    offset: *mut u32,
    num_counters: *mut u32,
}

pub(crate) struct Mlx4IbQp {}
pub(crate) struct PkeyMgt {
    virt2phys_pkey: [[[u8; 128]; 2]; 128],
    phys_pkey_cache: [[u16; 2]; 128],
    pkey_port_list: [bindings::list_head; 128],
    device_parent: [bindings::kobject; 128],
}

pub(crate) struct Mlx4IbIovSysfsAttr {
    ctx: *mut ffi::c_void,
    kobj: *mut bindings::kobject,
    data: u64,
    entry_num: u32,
    name: [char; 15],
    dentry: bindings::device_attribute,
    dev: *mut bindings::device,
}

pub(crate) struct Mlx4IbIovSysfsAttrAr {
    dentries: [Mlx4IbIovSysfsAttr; 385],
}

pub(crate) struct Mlx4IbIovPort {
    name: [char; 100],
    num: u8,
    dev: *mut Mlx4IbDev,
    list: bindings::list_head,
    dentr_ar: *mut Mlx4IbIovSysfsAttrAr,
    attr: bindings::ib_port_attr,
    cur_port: *mut bindings::kobject,
    admin_alias_parent: *mut bindings::kobject,
    gids_parent: *mut bindings::kobject,
    pkeys_parent: *mut bindings::kobject,
    mcgs_parent: *mut bindings::kobject,
    mcg_dentry: Mlx4IbIovSysfsAttr,
}

pub(crate) struct Mlx4IbCounters {
    counters_list: bindings::list_head,
    mutex: bindings::mutex,
    default_counter: u32,
}
pub(crate) enum Mlx4GuidAliasRecStatus {
    MLX4_GUID_INFO_STATUS_IDLE,
    MLX4_GUID_INFO_STATUS_SET,
}

pub(crate) struct Mlx4SriovAliasGuidInfoRecDet {
    all_recs: [u8; 64],
    guid_indexes: bindings::ib_sa_comp_mask,
    status: Mlx4GuidAliasRecStatus,
    guids_retry_schedule: [u32; 8],
    time_to_run: u64,
}

pub(crate) struct Mlx4SriovAliasGuidPortRecDet {
    all_rec_per_port: [Mlx4SriovAliasGuidInfoRecDet; 16],
    wq: *mut bindings::workqueue_struct,
    alias_guid_work: *mut bindings::delayed_work,
    port: u32,
    state_flags: u32,
    parent: *mut Mlx4SriovAliasGuid,
    cb_list: bindings::list_head,
}

pub(crate) struct Mlx4SriovAliasGuid {
    ports_guid: [Mlx4SriovAliasGuidPortRecDet; bindings::MLX4_MAX_PORTS as usize],
    ag_work_lock: bindings::spinlock_t,
    sa_client: bindings::ib_sa_client,
}

pub(crate) struct CounterIndex {
    list: bindings::list_head,
    index: u32,
    allocated: u8,
}

pub(crate) struct Mlx4IbBuf {
    addr: *mut ffi::c_void,
    mqp: bindings::dma_addr_t,
}

pub(crate) struct Mlx4IbTunTxBuf {
    buf: Mlx4IbBuf,
    ah: *mut bindings::ib_ah,
}

pub(crate) struct Mlx4IbDemuxPvQp {
    qp: *mut bindings::ib_qp,
    proxy_qgt: bindings::ib_qp_type,
    ring: *mut Mlx4IbBuf,
    tx_ring: *mut Mlx4IbTunTxBuf,
    tx_lock: bindings::spinlock_t,
    tx_ix_head: u32,
    tx_ix_tail: u32,
}

pub(crate) enum Mlx4IbDemuxPvState {
    DEMUX_PV_STATE_DOWN,
    DEMUX_PV_STATE_STARTING,
    DEMUX_PV_STATE_ACTIVE,
    DEMUX_PV_STATE_DOWNING,
}

pub(crate) struct Mlx4IbDemuxPvCtx {
    port: i32,
    slave: i32,
    state: Mlx4IbDemuxPvState,
    has_smi: i32,
    ib_dev: *mut bindings::ib_device,
    cq: *mut bindings::ib_cq,
    pd: *mut bindings::ib_pd,
    work: bindings::work_struct,
    wq: *mut bindings::workqueue_struct,
    wi_wq: *mut bindings::workqueue_struct,
    qp: [Mlx4IbDemuxPvQp; 2],
}

pub(crate) struct Mlx4IbDemuxCtx {
    ib_dev: *mut bindings::ib_device,
    port: i32,
    wq: *mut bindings::workqueue_struct,
    wi_wq: *mut bindings::workqueue_struct,
    ud_wq: *mut bindings::workqueue_struct,
    ud_lock: bindings::spinlock_t,
    subnet_prefix: bindings::atomic64_t,
    guid_cache: [bindings::__be64; 128],
    dev: *mut Mlx4IbDev,

    mcg_table_lock: bindings::mutex,
    mcg_table: bindings::rb_root,
    mcg_mgid0_list: bindings::list_head,
    mcg_wq: *mut bindings::workqueue_struct,
    tun: *mut *mut Mlx4IbDemuxPvCtx,
    tid: bindings::atomic_t,
    flushing: i32,
}

pub(crate) struct Mlx4IbSriov {
    demux: [Mlx4IbDemuxCtx; bindings::MLX4_MAX_PORTS as usize],
    sqps: [Mlx4IbDemuxPvCtx; bindings::MLX4_MAX_PORTS as usize],
    /* when using this spinlock you should use "irq" because
     * it may be called from interrupt context.*/
    going_down_lock: bindings::spinlock_t,
    is_going_down: i32,

    alias_guid: Mlx4SriovAliasGuid,

    /* CM paravirtualization fields */
    pv_id_table: bindings::xarray,
    pv_id_next: u32,
    id_map_lock: bindings::rb_root,
    cm_list: bindings::list_head,
    xa_rej_tmout: bindings::xarray,
}

pub(crate) struct GidCacheContext {
    read_index: i32,
    refcount: i32,
}
pub(crate) struct GidEntry {
    gid: bindings::ib_gid,
    gid_type: bindings::ib_gid_type,
    ctx: GidCacheContext,
    vlan_id: u16,
}

pub(crate) struct Mlx4PortGidTable {
    gids: [GidEntry; bindings::MLX4_MAX_PORT_GIDS as usize],
}

pub(crate) struct Mlx4IbIboe {
    lock: bindings::spinlock_t,
    netdevs: [bindings::net_device; bindings::MLX4_MAX_PORTS as usize],
    mac: [bindings::atomic64_t; bindings::MLX4_MAX_PORTS as usize],
    nb: bindings::notifier_block,
    gids: [Mlx4PortGidTable; bindings::MLX4_MAX_PORTS as usize],
    last_port_state: [bindings::ib_port_state; bindings::MLX4_MAX_PORTS as usize],
}

pub(crate) struct Mlx4IbDev {
    //
    pub(crate) ib_dev: Option<bindings::ib_device>,
    //
    pub(crate) dev: Option<*mut bindings::mlx4_dev>,
    //
    pub(crate) num_ports: i32,
    //
    pub(crate) uar_map: Option<*mut core::ffi::c_void>,

    //
    pub(crate) priv_uar: Option<bindings::mlx4_uar>,
    //
    pub(crate) priv_pdn: u32,
    // 64位模式不启动
    pub(crate) uar_lock: Option<bindings::spinlock_t>,

    pub(crate) send_agent: Option<[[bindings::ib_mad_agent; bindings::MLX4_MAX_PORTS as usize]; 2]>,
    pub(crate) sm_ah: Option<[bindings::ib_ah; bindings::MLX4_MAX_PORTS as usize]>,
    pub(crate) sm_lock: Option<bindings::spinlock_t>,
    pub(crate) sl2vl: Option<bindings::atomic64_t>,
    pub(crate) sriov: Option<Mlx4IbSriov>,

    pub(crate) cap_mask_mutex: Option<bindings::mutex>,
    pub(crate) ib_active: Option<bool>,
    pub(crate) iboe: Option<Mlx4IbIboe>,
    pub(crate) counters_table: Option<[Mlx4IbCounters; 2]>,
    pub(crate) eq_table: Option<i32>,
    pub(crate) iov_parent: Option<*mut bindings::kobject>,
    pub(crate) ports_parent: Option<*mut bindings::kobject>,
    pub(crate) dev_ports_parent: Option<[bindings::kobject; 128]>,
    pub(crate) iov_ports: Option<[Mlx4IbIovPort; 2]>,
    pub(crate) pkeys: Option<PkeyMgt>,
    pub(crate) ib_uc_qpns_bitmap: Option<u64>,
    pub(crate) steer_qpn_count: Option<i32>,
    pub(crate) steer_qpn_base: Option<i32>,
    pub(crate) steering_support: Option<i32>,
    // todo Mlx4IbQp
    pub(crate) qp1_proxy: Option<[Mlx4IbQp; bindings::MLX4_MAX_PORTS as usize]>,
    pub(crate) qp1_proxy_lock: Option<[bindings::mutex; bindings::MLX4_MAX_PORTS as usize]>,
    //
    pub(crate) bond_next_port: u8,
    pub(crate) reset_flow_resource_lock: Option<bindings::spinlock_t>,
    pub(crate) qp_list: Option<bindings::list_head>,
    pub(crate) diag_counters: Option<[Mlx4IbDiagCounters; 2]>,
}

impl Mlx4IbDev {
    pub(crate) fn new() -> Self {
        Self {
            ib_dev: None,
            dev: None,
            num_ports: 0,
            uar_map: None,

            priv_uar: None,
            priv_pdn: 0,
            uar_lock: None,

            send_agent: None,
            sm_ah: None,
            sm_lock: None,
            sl2vl: None,
            sriov: None,

            cap_mask_mutex: None,
            ib_active: None,
            iboe: None,
            counters_table: None,
            eq_table: None,
            iov_parent: None,
            ports_parent: None,
            dev_ports_parent: None,
            iov_ports: None,
            pkeys: None,
            ib_uc_qpns_bitmap: None,
            steer_qpn_count: None,
            steer_qpn_base: None,
            steering_support: None,
            // todo
            qp1_proxy: None,
            qp1_proxy_lock: None,
            bond_next_port: 0,
            reset_flow_resource_lock: None,
            qp_list: None,
            diag_counters: None,
        }
    }
}
