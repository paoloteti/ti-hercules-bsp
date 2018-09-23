///
/// ARM Performance Monitor Unit (PMU)
///
/// Provides an Arm PMU driver for counting events such as cycles,
/// instructions, and cache metrics
///
/// #Reference:
///   Cortex-R4 and Cortex-R4F Technical Reference Manual, Revision: r1p4
///   "Chapter 6. Events and Performance Monitor"
///

pub enum PmuEvent {
    InstructionCacheMiss = 0x01,
    DataCacheMiss = 0x03,
    DataCacheAccess = 0x04,
    DataReadArchExecuted = 0x06,
    DataWriteArchExecuted = 0x07,
    InstructionArchExecuted = 0x08,
    ExceptionTaken = 0x09,
    ExceptionReturnArchExecuted = 0x0A,
    ChangeContextExecuted = 0x0B,
    SwChangePcArchExecuted = 0x0C,
    BranchImmArchExecuted = 0x0D,
    ProcReturnArchExecuted = 0x0E,
    UnalignedAccessArchExecuted = 0x0F,
    BranchMissPredicted = 0x10,
    CycleCount = 0x11,
    PredicatableBranches = 0x12,
    InstrBufferStall = 0x40,
    DataDependencyInstrStall = 0x41,
    DataCacheWriteBack = 0x42,
    ExtMemoryRequest = 0x43,
    LsuBusyStall = 0x44,
    ForceDrainOfStoreBuffer = 0x45,
    FiqDisabledCycleCount = 0x46,
    IrqDisabledCycleCount = 0x47,
    EtmExTimeout0 = 0x48,
    EtmExTimeout1 = 0x49,
    InstructionCacheTagEccError = 0x4A,
    InstructionCacheDataEccError = 0x4B,
    DataCacheTagEccError = 0x4C,
    DataCacheDataEccError = 0x4D,
    TcmFatalEccErrorPrefetch = 0x4E,
    TcmFatalEccErrorLoadStore = 0x4F,
    StoreBufferMerge = 0x50,
    LsuStallStoreBufferFull = 0x51,
    LsuStallStoreQueueFull = 0x52,
    IntegerDivExecuted = 0x53,
    StallIntegerDiv = 0x54,
    PldInstLineFill = 0x55,
    PldInstNoLineFill = 0x56,
    NonCacheableAccessAxiMaster = 0x57,
    InstructionCacheAccess = 0x58,
    DoubleDataCacheIssue = 0x59,
    DualIssueCaseA = 0x5A,
    DualIssueCaseB1orB2orF2orF2D = 0x5B,
    DualIssueOther = 0x5C,
    DpFloatInstExecuted = 0x5D,
    DualIssuedPairInstrArchExecuted = 0x5E,
    DataCacheDataFatalEccError = 0x60,
    DataCacheTagFatalEccError = 0x61,
    PRocessorLiveLock = 0x62,
    ATcmMultiBitEccError = 0x64,
    B0TcmMultiBitEccError = 0x65,
    B1TcmMultiBitEccError = 0x66,
    ATcmSignleBitEccError = 0x67,
    B0TcmSingleBitEccError = 0x68,
    B1TcmSingleBitEccError = 0x69,
    TcmCorEccErrorLoadStore = 0x6A,
    TcmCorEccErrorPrefetch = 0x6B,
    TcmFatailEccErrorAxiSlave = 0x6C,
    TcmCorEccErrorAxiSlave = 0x6D,
}

#[derive(Copy,Clone)]
pub enum Counter {
    Cycle,
    Event,
    Both,
}

pub unsafe fn init() {
    asm!("
        /* set control register */
        mrc p15, #0, r0, c9, c12, #0
        orr r0,  r0, #(1 << 4) + 6 + 1
        mcr p15, #0, r0, c9, c12, #0
        /* clear flags */
        mov r0,  #0
        sub r0,  r0,  #1
        mcr p15, #0, r0, c9, c12, #3

        /* select counter 0, 1, 2 event */
		mov r0,  #0
        mcr p15, #0, r0, c9, c12, #5
        mov r0,  #0x11
        mcr p15, #0, r0, c9, c13, #1

        mov r0,  #1
        mcr p15, #0, r0, c9, c12, #5
        mov r0,  #0x11
        mcr p15, #0, r0, c9, c13, #1

        mov r0,  #2
        mcr p15, #0, r0, c9, c12, #5
        mov r0,  #0x11
        mcr p15, #0, r0, c9, c13, #1
    " :::: "volatile")
}

/// Enable (and reset) cycle counter and all event counters
pub unsafe fn counters_global_enable() {
    asm!("
        mrc p15, #0, r0, c9, c12, #0
        orr r0, r0, #7
        mcr p15, #0, r0, c9, c12, #0
    " :::: "volatile")
}

/// Disable all event counters
pub unsafe fn counters_global_disable() {
    asm!("
        mrc p15, #0, r0, c9, c12, #0
        bic r0, r0, #1
        mcr p15, #0, r0, c9, c12, #0
    " :::: "volatile")
}

/// Reset cycle, event or both counters
pub unsafe fn reset_counters(counter: Counter) {
    asm!("mrc p15, #0, r0, c9, c12, #0");
    match counter {
        Counter::Event => asm!("orr r0, r0, #2"),
        Counter::Cycle => asm!("orr r0, r0, #4"),
        Counter::Both => asm!("orr r0, r0, #6"),
    }
    asm!("mcr p15, #0, r0, c9, c12, #0")
}

/// Starts selected counters
/// 'counters' - Counter mask
#[inline]
pub unsafe fn start_counters(counters: u32) {
    asm!("
        mcr p15, #0, r0, c9, c12, #1"
        :
        : "0"(counters)
        : "memory"
        : "volatile")
}

/// Stops selected counters
/// 'counters' - Counter mask
#[inline]
pub unsafe fn stop_counters(counters: u32) {
    asm!("
        mcr p15, #0, r0, c9, c12, #1"
        :
        : "0"(counters)
        : "memory"
        : "volatile")
}

/// Set event counter count event
/// 'counter' - Counter select
/// 'event'   - Count event
pub unsafe fn set_count_event(counter: u32, event: PmuEvent) {
    let event_code = event as u32;
    asm!("
        mcr p15, #0, r0, c9, c12, #5
        mcr p15, #0, r1, c9, c13, #1"
        :
        : "0"(counter) "1"(event_code)
        : "memory"
        : "volatile")
}

/// Returns current cycle counter value
#[inline]
pub unsafe fn get_cycle_count() -> u32 {
    let cycles: u32;
    asm!("
        mrc p15, #0, r0, c9, c13, #0"
        : "=r"(cycles)
        :
        : "memory"
        : "volatile");
    cycles
}

/// Returns current event counter value
/// 'counter' - Counter select
pub unsafe fn get_event_count(counter: u32) {
    asm!("
        mcr p15, #0, r0, c9, c12, #5
        mrc p15, #0, r0, c9, c13, #2"
        :
        : "0"(counter)
        : "memory"
        : "volatile")
}

/// Returns current overflow register and clear flags
pub fn overflow() -> u32 {
    0
}
