#![no_std]

use core::marker::PhantomData;
use imxrt_iomuxc as iomuxc;
use imxrt_iomuxc::consts::{Unsigned, U1, U2};
use imxrt_ral as ral;

pub mod device;
use device::Device;

pub struct Unclocked<M>
where
    M: Unsigned,
{
    pub(crate) _module: PhantomData<M>,
    pub(crate) reg: ral::usb::Instance,
}

impl<M> Unclocked<M>
where
    M: Unsigned
{
    /// Enable the clock for a particular USB device
    pub fn clock(self, handle: &mut ccm::Handle) -> Builder<M> {
        //TODO enable matching USB PLL here
        Builder::new(self.dev)
    }
}

pub struct Builder<M>
where
    M: Unsigned, {
    //TODO pub fn unclock(self) -> Unclocked<M> {...}

//TODO pub fn otg(self) -> OTG<M> {...}

//TODO pub fn host(self) -> Host<M> {...}

// pub fn device(self) -> Device<M>
// {
// Device::new(self.reg);
// }
}
