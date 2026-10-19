#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Calibration temperature and checksum"]
    pub cal: CAL,
    _reserved1: [u8; 0x18],
    #[doc = "0x1c - USHFRCO calibration register"]
    pub ushfrcocal0: USHFRCOCAL0,
    _reserved2: [u8; 0x04],
    #[doc = "0x24 - AUXHFRCO calibration register 0"]
    pub auxhfrcocal0: AUXHFRCOCAL0,
    #[doc = "0x28 - AUXHFRCO calibration register 1"]
    pub auxhfrcocal1: AUXHFRCOCAL1,
    _reserved4: [u8; 0x03],
    #[doc = "0x2c - HFRCO calibration register 0"]
    pub hfrcocal0: HFRCOCAL0,
    #[doc = "0x30 - HFRCO calibration register 1"]
    pub hfrcocal1: HFRCOCAL1,
    _reserved6: [u8; 0x0f],
    #[doc = "0x40 - Low 32 bits of the device unique number"]
    pub uniquel: UNIQUEL,
    #[doc = "0x44 - High 32 bits of the device unique number"]
    pub uniqueh: UNIQUEH,
    #[doc = "0x48 - Flash and SRAM sizes in kilobytes"]
    pub msize: MSIZE,
    #[doc = "0x4c - Part description"]
    pub part: PART,
}
#[doc = "CAL (r) register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration temperature and checksum"]
pub mod cal;
#[doc = "USHFRCOCAL0 (r) register accessor: an alias for `Reg<USHFRCOCAL0_SPEC>`"]
pub type USHFRCOCAL0 = crate::Reg<ushfrcocal0::USHFRCOCAL0_SPEC>;
#[doc = "USHFRCO calibration register"]
pub mod ushfrcocal0;
#[doc = "AUXHFRCOCAL0 (r) register accessor: an alias for `Reg<AUXHFRCOCAL0_SPEC>`"]
pub type AUXHFRCOCAL0 = crate::Reg<auxhfrcocal0::AUXHFRCOCAL0_SPEC>;
#[doc = "AUXHFRCO calibration register 0"]
pub mod auxhfrcocal0;
#[doc = "AUXHFRCOCAL1 (r) register accessor: an alias for `Reg<AUXHFRCOCAL1_SPEC>`"]
pub type AUXHFRCOCAL1 = crate::Reg<auxhfrcocal1::AUXHFRCOCAL1_SPEC>;
#[doc = "AUXHFRCO calibration register 1"]
pub mod auxhfrcocal1;
#[doc = "HFRCOCAL0 (r) register accessor: an alias for `Reg<HFRCOCAL0_SPEC>`"]
pub type HFRCOCAL0 = crate::Reg<hfrcocal0::HFRCOCAL0_SPEC>;
#[doc = "HFRCO calibration register 0"]
pub mod hfrcocal0;
#[doc = "HFRCOCAL1 (r) register accessor: an alias for `Reg<HFRCOCAL1_SPEC>`"]
pub type HFRCOCAL1 = crate::Reg<hfrcocal1::HFRCOCAL1_SPEC>;
#[doc = "HFRCO calibration register 1"]
pub mod hfrcocal1;
#[doc = "UNIQUEL (r) register accessor: an alias for `Reg<UNIQUEL_SPEC>`"]
pub type UNIQUEL = crate::Reg<uniquel::UNIQUEL_SPEC>;
#[doc = "Low 32 bits of the device unique number"]
pub mod uniquel;
#[doc = "UNIQUEH (r) register accessor: an alias for `Reg<UNIQUEH_SPEC>`"]
pub type UNIQUEH = crate::Reg<uniqueh::UNIQUEH_SPEC>;
#[doc = "High 32 bits of the device unique number"]
pub mod uniqueh;
#[doc = "MSIZE (r) register accessor: an alias for `Reg<MSIZE_SPEC>`"]
pub type MSIZE = crate::Reg<msize::MSIZE_SPEC>;
#[doc = "Flash and SRAM sizes in kilobytes"]
pub mod msize;
#[doc = "PART (r) register accessor: an alias for `Reg<PART_SPEC>`"]
pub type PART = crate::Reg<part::PART_SPEC>;
#[doc = "Part description"]
pub mod part;
