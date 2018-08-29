
extern "C" {
    fn _init_core_registers();
    fn _init_stack_pointers();
    fn _flash_ecc_enable();
    fn _flash_ecc_disable();
    fn _event_bus_export_enable();
    fn _event_bus_export_disable();
    fn _ram_ecc_enable();
    fn _ram_ecc_disable();
    fn _irq_vic_enable();
    fn _vfp_enable();
}

/// Initialize Core registers, including floating point, in all
/// CPU working modes.
/// This function is andatory to avoid any lock-step compare
/// failure at startup or at first mode switch.
#[naked]
pub unsafe fn init_core_registers() {
    _init_core_registers()
}

#[naked]
pub unsafe fn init_stack_pointers() {
    _init_stack_pointers()
}

/// Enable or disable ECC on integrated flash memory
pub unsafe fn flash_ecc(enable: bool) {
    if enable {
        _flash_ecc_enable()
    } else {
        _flash_ecc_disable()
    }
}

/// Enable or disable Event Bus Export
pub unsafe fn event_bus_export(export: bool) {
    if export {
        _event_bus_export_enable()
    } else {
        _event_bus_export_disable()
    }
}

pub unsafe fn ram_ecc(enable: bool) {
    if enable {
        _ram_ecc_enable()
    } else {
        _ram_ecc_disable()
    }
}

/// Enable Offset via Vic controller
pub unsafe fn irq_vic_enable() {
    _irq_vic_enable()
}

/// Enable Vector Floating Point unit
#[cfg(vfp)]
pub unsafe fn vfp_enable() {
    _vfp_enable()
}
