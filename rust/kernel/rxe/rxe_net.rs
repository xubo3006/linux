// SPDX-License-Identifier: GPL-2.0

//! rxe net.
//!
//! Also called 
//!
//! C header: [`include/linux/cdev.h`](../../../../include/linux/cdev.h)
//!
//! Reference: <https://www.kernel.org/doc/html/latest/core-api/kernel-api.html#char-devices>
use core::ptr;

use bindings::udp_tunnel_sock_cfg;

use crate::{bindings, container_of, pr_err, pr_debug};
use crate::rxe::rxe_verbs::RxeDev;
use crate::rxe::rxe_hdr::{RxePktInfo,skb_to_pkt};
use crate::error::{code::*,Error, Result};
use core::{mem};

pub(crate) struct RxeRecvSockets {
    sk4: Option<*mut bindings::socket>,
    sk6: Option<*mut bindings::socket>,
    rxe_net_notifier: Option<bindings::notifier_block>,
    
}

// 返回值后续改为result的形式
impl RxeRecvSockets{
    pub(crate) fn new()->Self {
        Self{
            sk4: None,
            sk6: None,
            rxe_net_notifier: None,
        }
    }

    pub(crate) fn alloc(&mut self) -> Result<()>{
        //let rxe_socket= Self::new();

        // 初始化网络部分
        match self.ipv4_init() {
            Ok(_tmp)=>{},
            Err(e)=>{return Err(e)},
        }
       
       match self.ipv6_init() {
            Ok(_tmp)=>{},
            Err(e)=>{
                self.rxe_net_release();
                return Err(e)
            },
       }

       match self.net_notifier_register() {
            Ok(_tmp)=>{},
            Err(e)=>{
                self.rxe_net_release();
                return Err(e)
            },
       }
       Ok(())
    }

    fn ipv4_init(&mut self) ->Result<()>{
        let mut udp_cfg=bindings::udp_port_cfg::default();
        let mut tnl_cfg=bindings::udp_tunnel_sock_cfg::default();
        let mut sock:*mut bindings::socket=ptr::null_mut();
        
        udp_cfg.family=bindings::AF_INET as u8;
        udp_cfg.local_udp_port=46866;
        let err=unsafe{
            bindings::udp_sock_create4(&mut bindings::init_net,&mut udp_cfg,&mut sock)
        };

        if err<0 {
            pr_err!("Failed to create IPv4 UDP tunnel\n");
            return Err(Error::from_kernel_errno(err));
        }

        tnl_cfg.encap_type=1;
        tnl_cfg.encap_rcv=Some(rxe_udp_encap_recv);

        unsafe{
            bindings::setup_udp_tunnel_sock(&mut bindings::init_net, sock, &mut tnl_cfg)
        }
        self.sk4=Some(sock);
        Ok(())
    }

    fn ipv6_init(&mut self) ->Result<()>{
        #[cfg(CONFIG_IPV6)]
        {
            let mut udp_cfg=bindings::udp_port_cfg::default();
            let mut tnl_cfg=bindings::udp_tunnel_sock_cfg::default();
            let mut sock:*mut bindings::socket=ptr::null_mut();
            
            udp_cfg.family=bindings::AF_INET6 as u8;
            udp_cfg.set_ipv6_v6only(1);
            udp_cfg.local_udp_port=46866;
            let err=unsafe{
                bindings::udp_sock_create4(&mut bindings::init_net,&mut udp_cfg,&mut sock)
            };
    
            if err<0 {
                // EAFNOSUPPORT
                if err==-97 {
                    pr_err!("IPv6 is not supported, can not create a UDPv6 socket\n");
                    return Ok(());
                } else {
                    pr_err!("Failed to create IPv6 UDP tunnel\n");
                    return Err(Error::from_kernel_errno(err));
                }
            }
    
            tnl_cfg.encap_type=1;
            tnl_cfg.encap_rcv=Some(rxe_udp_encap_recv);
    
            unsafe{
                bindings::setup_udp_tunnel_sock(&mut bindings::init_net, sock, &mut tnl_cfg)
            }
            self.sk4=Some(sock);
        }
        Ok(())
    }

    fn net_notifier_register(&mut self) ->Result<()>{
        let err:i32;
        self.rxe_net_notifier=Some(RxeNotifyFuncTable::build());
        unsafe {
            err=bindings::register_netdevice_notifier(self.rxe_net_notifier.as_mut().unwrap());
        }
        if err!=0 {
            pr_err!("Failed to register netdev notifier\n");
            if self.rxe_net_notifier.is_some() {
                unsafe{bindings::unregister_netdevice_notifier(&mut self.rxe_net_notifier.take().unwrap())};
            }
            return Err(Error::from_kernel_errno(err));
        }

        Ok(())
    }
    
    fn rxe_net_release(&mut self) {
        if self.sk4.is_some(){
            unsafe{
                bindings::udp_tunnel_sock_release(self.sk4.take().unwrap());
            }
        }
        if self.sk6.is_some(){
            unsafe{
                bindings::udp_tunnel_sock_release(self.sk6.take().unwrap());
            }
        }
    }
    
}

impl Drop for RxeRecvSockets{
    fn drop(&mut self) {
        self.rxe_net_release();
        if self.rxe_net_notifier.is_some() {
            unsafe{bindings::unregister_netdevice_notifier(&mut self.rxe_net_notifier.take().unwrap())};
        }
    }
}


unsafe extern "C" fn rxe_udp_encap_recv(sk:*mut bindings::sock,skb:*mut bindings::sk_buff) -> i32 {
    let udph:bindings::udphdr;
    let mut rxe:*const RxeDev;
    let mut ndev:*mut bindings::net_device;
    let mut pkt:*mut RxePktInfo;
    
    unsafe{
        ndev=(*skb).__bindgen_anon_1.__bindgen_anon_1.__bindgen_anon_1.dev;
        pkt=skb_to_pkt(skb);
    }

    rxe=rxe_get_dev_from_net(ndev);
    if rxe==ptr::null_mut() && is_vlan_dev(ndev) {
        unsafe{
            bindings::BUG();
        }
        // rxe=rxe_get_dev_from_net(ptr::null_mut());
    }

    if rxe==ptr::null_mut() {
        unsafe{
            bindings::__kfree_skb(skb);
        }
        return 0;
    }

    let res=if unsafe{(*skb).data_len !=0} {
        if unsafe{bindings::__pskb_pull_tail(skb, (*skb).data_len as i32).is_null()} {
            -12
        } else {
            0
        }
    } else {
        0
    };
    
    if res!=0{
        unsafe{
            bindings::ib_device_put((*rxe).ib_device);
            bindings::__kfree_skb(skb);
        }
        return 0;
    }

    // todo先省略一部分麻烦的
    // 编译时检查被省略了
    if !((*skb).__bindgen_anon_5.__bindgen_anon_1.as_ref().transport_header != -1){
        udph=unsafe{(*skb).head+(*skb).__bindgen_anon_5.__bindgen_anon_1.as_ref().transport_header};
    }
    (*pkt).rxe=rxe;
    (*pkt).port_num=1;
    (*pkt).hdr=udph.wrapping_offset(1).cast::<u8>();
    // RXE_GRH_MASK
    (*pkt).mask=2;
    (*pkt).paylen=be16_to_cpu((*udph).len)-mem::size_of::<bindings::udphdr>();


    
    return 0
}

fn rxe_get_dev_from_net(ndev:*mut bindings::net_device) ->*const RxeDev{
    let ibdev:*mut bindings::ib_device;
    unsafe{
        ibdev=bindings::ib_device_get_by_netdev(ndev, bindings::rdma_driver_id_RDMA_DRIVER_RXE);
    }
    
    if ibdev==ptr::null_mut() {
        return ptr::null_mut();
    }

    return container_of!(ibdev,RxeDev,ib_device);
}

fn is_vlan_dev(dev:*mut bindings::net_device)->bool{
    let res:u64;
    unsafe{
        res=(*dev).priv_flags & bindings::netdev_priv_flags_IFF_802_1Q_VLAN;
    }

    if res==0 {
        return false;
    }

    true
}

fn be16_to_cpu(value: u16) -> u16 {
    u16::from_be_bytes(value.to_be_bytes())
}

struct RxeNotifyFuncTable;

impl RxeNotifyFuncTable {
    pub(crate) fn build()->bindings::notifier_block {
        Self::NOTIFYFUNC
    }

    const NOTIFYFUNC:bindings::notifier_block=bindings::notifier_block {
        notifier_call:Some(Self::rxe_notify),
        next:ptr::null_mut(),
        priority:0,
    };

    unsafe extern "C" fn rxe_notify(
        not_blk:*mut bindings::notifier_block,
        event: core::ffi::c_ulong,
        arg: *mut core::ffi::c_void,
    ) -> core::ffi::c_int {
        let ndev_info:*mut bindings::netdev_notifier_info=unsafe{mem::transmute(arg)};
        let ndev:*mut bindings::net_device=unsafe{(*ndev_info).dev};

        let ibdev:*mut bindings::ib_device=unsafe{bindings::ib_device_get_by_netdev(ndev, bindings::rdma_driver_id_RDMA_DRIVER_RXE)};
        let rxe:*mut RxeDev=if ibdev==ptr::null_mut() {
            ptr::null_mut()
        } else{
            let aaa=container_of!(ibdev,RxeDev,ib_device);
            aaa.cast_mut()
        };

        if rxe==ptr::null_mut() {
            return bindings::NOTIFY_OK as i32;
        }

        match event as u32 {
            bindings::netdev_cmd_NETDEV_UNREGISTER=>{},
            bindings::netdev_cmd_NETDEV_UP=>{},
            bindings::netdev_cmd_NETDEV_DOWN=>{},
            bindings::netdev_cmd_NETDEV_CHANGEMTU=>{
                pr_debug!("{:?} changed mut to {}\n",(*ndev).name,(*ndev).mtu);
                //rxe_set_mtu(rxe, ndev->mtu);
            },
            bindings::netdev_cmd_NETDEV_CHANGE=>{
                //rxe_set_port_state(rxe);
            },
            bindings::netdev_cmd_NETDEV_REBOOT=>{},
            bindings::netdev_cmd_NETDEV_GOING_DOWN=>{},
            bindings::netdev_cmd_NETDEV_CHANGEADDR=>{},
            bindings::netdev_cmd_NETDEV_CHANGENAME=>{},
            bindings::netdev_cmd_NETDEV_FEAT_CHANGE=>{},
            _=>{
                pr_debug!("ignoring netdev event = {} for {:?}\n",event,(*ndev).name);
            },
        }
        unsafe{bindings::ib_device_put((*rxe).ib_device);}
        return bindings::NOTIFY_OK as i32;
    }
}

