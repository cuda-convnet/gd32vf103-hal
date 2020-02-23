///! Device elevtronic signature
/// 
/// Section 1.5, GD32VF103 User Manual

const UNIQUE_ID: *const [u8; 96] = 0x1FFF_F7E8 as *const _;
const MEMORY_DENSITY: *const u16 = 0x1FFF_F7E0 as *const _;

/// Factory programed unique device id.
/// 
/// According to section 1.5.2 of the Manual, this value
/// can never be altered by user.
pub fn unique_id() -> &'static [u8; 96] {
    // note(unsafe): static read-only value
    unsafe { &*UNIQUE_ID }
}

/// Flash memory density in KBytes.
/// 
/// This value indicates the flash memory density of the device in KBytes.
/// For example, 0x0020 means 32 KBytes.
pub fn flash_density() -> u16 {
    // note(unsafe): static read-only value
    unsafe { *MEMORY_DENSITY } // read bits [15:0]
}

/// On-chip SRAM density in KBytes.
/// 
/// This value indicates the on-chip SRAM density of the device in KBytes.
/// For example, 0x0008 means 8 KBytes.
pub fn sram_density() -> u16 {
    // note(unsafe): static read-only value
    unsafe { *(MEMORY_DENSITY.add(2)) } // read bits [31:16]
}
