// SPDX-License-Identifier: GPL-2.0

//! rxe net.
//!
//! Also called 
//!
//! C header: [`include/linux/cdev.h`](../../../../include/linux/cdev.h)
//!
//! Reference: <https://www.kernel.org/doc/html/latest/core-api/kernel-api.html#char-devices>

pub(crate) struct RxeDev {
    pub(crate) ib_device:*mut bindings::ib_device,
}

// struct rxe_dev {
// 	struct ib_device	ib_dev;
// 	struct ib_device_attr	attr;
// 	int			max_ucontext;
// 	int			max_inline_data;
// 	struct mutex	usdev_lock;

// 	struct net_device	*ndev;

// 	struct rxe_pool		uc_pool;
// 	struct rxe_pool		pd_pool;
// 	struct rxe_pool		ah_pool;
// 	struct rxe_pool		srq_pool;
// 	struct rxe_pool		qp_pool;
// 	struct rxe_pool		cq_pool;
// 	struct rxe_pool		mr_pool;
// 	struct rxe_pool		mw_pool;

// 	/* multicast support */
// 	spinlock_t		mcg_lock;
// 	struct rb_root		mcg_tree;
// 	atomic_t		mcg_num;
// 	atomic_t		mcg_attach;

// 	spinlock_t		pending_lock; /* guard pending_mmaps */
// 	struct list_head	pending_mmaps;

// 	spinlock_t		mmap_offset_lock; /* guard mmap_offset */
// 	u64			mmap_offset;

// 	atomic64_t		stats_counters[RXE_NUM_OF_COUNTERS];

// 	struct rxe_port		port;
// 	struct crypto_shash	*tfm;
// };

pub(crate) struct RxeQp{
    ibqp:bindings::ib_qp,
    elem:bindings::rxe_pool
} 