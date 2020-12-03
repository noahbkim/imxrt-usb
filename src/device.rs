use core::marker::PhantomData;
use imxrt_iomuxc as iomuxc;
use imxrt_iomuxc::consts::Unsigned;
use imxrt_ral as ral;

use usb_device::bus::{PollResult, UsbBus};
use usb_device::endpoint::{EndpointAddress, EndpointType};
pub use usb_device::{Result as UsbResult, UsbDirection, UsbError};

pub struct Device<M>
where
    M: Unsigned,
{
    _module: PhantomData<M>,
    reg: ral::usb::Instance,
}

impl<M> Device<M>
where
    M: Unsigned
{
    pub(crate) fn new(reg: ral::usb::Instance) -> Device<M> {
        //TODO initialize the usb instance as a device controller
        Device {
            _module: Default::default(),
            reg,
        }
    }
}

impl<M> UsbBus for Device<M>
where
    M: Unsigned
{
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        interval: u8,
    ) -> UsbResult<EndpointAddress> {
        unimplemented!()
    }

    fn enable(&mut self) {
        unimplemented!()
    }

    fn reset(&self) {
        unimplemented!()
    }

    fn set_device_address(&self, addr: u8) {
        unimplemented!()
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> UsbResult<usize> {
        unimplemented!()
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> UsbResult<usize> {
        unimplemented!()
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        unimplmented!()
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        unimplemented!()
    }

    fn suspend(&self) {
        unimplemented!()
    }

    fn resume(&self) {
        unimplemented!()
    }

    fn poll(&self) -> PollResult {
        unimplmented!()
    }
}
