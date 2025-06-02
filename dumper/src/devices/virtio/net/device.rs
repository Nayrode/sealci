// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0 OR BSD-3-Clause

use std::borrow::{Borrow, BorrowMut};
use std::net::Ipv4Addr;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};

use super::super::features::{VIRTIO_F_IN_ORDER, VIRTIO_F_RING_EVENT_IDX, VIRTIO_F_VERSION_1};
use super::super::net::features::*;
use super::super::net::{Error, NetArgs, Result, NET_DEVICE_ID, VIRTIO_NET_HDR_SIZE};
use super::super::{CommonConfig, Env, SingleFdSignalQueue, QUEUE_MAX_SIZE};
use crate::devices::virtio::net::bridge::Bridge;
use crate::devices::virtio::net::iptables::iptables_ip_masq;
use virtio_device::{VirtioConfig, VirtioDeviceActions, VirtioDeviceType, VirtioMmioDevice};
use virtio_queue::{Queue, QueueT};
use vm_device::bus::MmioAddress;
use vm_device::device_manager::MmioManager;
use vm_device::{DeviceMmio, MutDeviceMmio};
use vm_memory::{GuestAddressSpace, GuestMemoryMmap};

use super::bindings;
use super::queue_handler::QueueHandler;
use super::simple_handler::SimpleHandler;
use super::tap::Tap;

pub struct Net<M>
where
    M: GuestAddressSpace + Clone + Send + Sync + 'static,
{
    mem: Arc<GuestMemoryMmap>,
    cfg: CommonConfig<M>,
    tap_name: String,
}

impl<M> Net<M>
where
    M: GuestAddressSpace + Clone + Send + Sync + 'static,
{
    pub async fn new<B>(
        mem: Arc<GuestMemoryMmap>,
        env: &mut Env<'_, M, B>,
        args: &NetArgs,
        iface_host_addr: Ipv4Addr,
        netmask: Ipv4Addr,
        iface_guest_addr: Ipv4Addr,
    ) -> Result<Arc<Mutex<Self>>>
    where
        // We're using this (more convoluted) bound so we can pass both references and smart
        // pointers such as mutex guards here.
        B: DerefMut,
        B::Target: MmioManager<D = Arc<dyn DeviceMmio + Send + Sync>>,
    {
        let device_features = (1 << VIRTIO_F_VERSION_1)
            | (1 << VIRTIO_F_RING_EVENT_IDX)
            | (1 << VIRTIO_F_IN_ORDER)
            | (1 << VIRTIO_NET_F_CSUM)
            | (1 << VIRTIO_NET_F_GUEST_CSUM)
            | (1 << VIRTIO_NET_F_GUEST_TSO4)
            | (1 << VIRTIO_NET_F_GUEST_TSO6)
            | (1 << VIRTIO_NET_F_GUEST_UFO)
            | (1 << VIRTIO_NET_F_HOST_TSO4)
            | (1 << VIRTIO_NET_F_HOST_TSO6)
            | (1 << VIRTIO_NET_F_HOST_UFO);

        // An rx/tx queue pair.
        let queues = vec![
            Queue::new(QUEUE_MAX_SIZE).map_err(Error::VirtQueue)?,
            Queue::new(QUEUE_MAX_SIZE).map_err(Error::VirtQueue)?,
        ];

        // TODO: We'll need a minimal config space to support setting an explicit MAC addr
        // on the guest interface at least. We use an empty one for now.
        let config_space = Vec::new();
        let virtio_cfg = VirtioConfig::new(device_features, queues, config_space);

        let common_cfg = CommonConfig::new(virtio_cfg, env).map_err(Error::Virtio)?;

        let bridge_name = "br0";
        let bridge = Bridge::new(bridge_name).await.map_err(Error::BridgeError)?;

        bridge
            .set_addr(iface_host_addr, netmask)
            .await
            .map_err(Error::BridgeError)?;

        bridge
            .attach_link(args.tap_name.clone())
            .await
            .map_err(Error::BridgeError)?;
        println!(
            "attached link {:?} to bridge {}",
            args.tap_name, bridge_name
        );

        bridge.set_up().await.map_err(Error::BridgeError)?;
        println!("bridge {} set UP", bridge_name);

        // Get internet access
        iptables_ip_masq(iface_host_addr & netmask, netmask, bridge_name.into());

        let net = Arc::new(Mutex::new(Net {
            mem,
            cfg: common_cfg,
            tap_name: args.tap_name.clone(),
        }));

        let ip_pnp_param: String = format!(
            "ip={}::{}:{}::eth0:off:1.1.1.1",
            iface_guest_addr, iface_host_addr, netmask
        );

        env.kernel_cmdline
            .insert_str(ip_pnp_param.as_str())
            .expect("failed to insert ip pnp parameter");

        env.register_mmio_device(net.clone())
            .map_err(Error::Virtio)?;

        Ok(net)
    }
}

impl<M: GuestAddressSpace + Clone + Send + Sync + 'static> VirtioDeviceType for Net<M> {
    fn device_type(&self) -> u32 {
        NET_DEVICE_ID
    }
}

impl<M: GuestAddressSpace + Clone + Send + Sync + 'static> Borrow<VirtioConfig<Queue>> for Net<M> {
    fn borrow(&self) -> &VirtioConfig<Queue> {
        &self.cfg.virtio
    }
}

impl<M: GuestAddressSpace + Clone + Send + Sync + 'static> BorrowMut<VirtioConfig<Queue>>
    for Net<M>
{
    fn borrow_mut(&mut self) -> &mut VirtioConfig<Queue> {
        &mut self.cfg.virtio
    }
}

impl<M: GuestAddressSpace + Clone + Send + Sync + 'static> VirtioDeviceActions for Net<M> {
    type E = Error;

    fn activate(&mut self) -> Result<()> {
        let tap = Tap::open_named(self.tap_name.as_str()).map_err(Error::Tap)?;

        // Set offload flags to match the relevant virtio features of the device (for now,
        // statically set in the constructor.
        tap.set_offload(
            bindings::TUN_F_CSUM
                | bindings::TUN_F_UFO
                | bindings::TUN_F_TSO4
                | bindings::TUN_F_TSO6,
        )
        .map_err(Error::Tap)?;

        // The layout of the header is specified in the standard and is 12 bytes in size. We
        // should define this somewhere.
        tap.set_vnet_hdr_size(VIRTIO_NET_HDR_SIZE as i32)
            .map_err(Error::Tap)?;

        let driver_notify = SingleFdSignalQueue {
            irqfd: self.cfg.irqfd.clone(),
            interrupt_status: self.cfg.virtio.interrupt_status.clone(),
        };

        let mut ioevents = self.cfg.prepare_activate().map_err(Error::Virtio)?;

        let rxq = self.cfg.virtio.queues.remove(0);
        let txq = self.cfg.virtio.queues.remove(0);
        let inner = SimpleHandler::new(self.mem.clone(), driver_notify, rxq, txq, tap);

        let handler = Arc::new(Mutex::new(QueueHandler {
            inner,
            rx_ioevent: ioevents.remove(0),
            tx_ioevent: ioevents.remove(0),
        }));

        self.cfg.finalize_activate(handler).map_err(Error::Virtio)
    }

    fn reset(&mut self) -> std::result::Result<(), Error> {
        // Not implemented for now.
        Ok(())
    }
}

impl<M: GuestAddressSpace + Clone + Send + Sync + 'static> VirtioMmioDevice for Net<M> {}

impl<M: GuestAddressSpace + Clone + Send + Sync + 'static> MutDeviceMmio for Net<M> {
    fn mmio_read(&mut self, _base: MmioAddress, offset: u64, data: &mut [u8]) {
        self.read(offset, data);
    }

    fn mmio_write(&mut self, _base: MmioAddress, offset: u64, data: &[u8]) {
        self.write(offset, data);
    }
}
