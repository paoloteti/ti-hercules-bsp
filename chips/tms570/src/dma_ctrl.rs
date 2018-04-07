
pub enum AccessWidth {
    Bits8 = 0,
    Bits16 = 1,
    Bits32 = 2,
    Bits64 = 3
}

pub enum DmaTrigger {
    /// Hardware trigger
    Hw = 0,
    /// Software trigger
    Sw = 1,
}

pub struct DmaControlPacket {
    /// Initial source address
    pub source :u32,
    /// Initial destination address
    pub destination :u32,
    /// Frame count
    pub frame_count :u32,
    /// Element count
    pub element_count :u32,
    /// Element destination offset
    pub element_dest_offset :u16,
    /// Element source offset
    pub element_source_offset :u16,
    /// Frame detination offset
    pub frame_dest_offset :u16,
    /// Frame source offset
    pub frame_source_offset :u16,
    /// Dma port
    pub port :u8,
    /// Read element size
    pub read_size :AccessWidth,
    /// Write element size
    pub write_size :AccessWidth,
    /// trigger type - frame/block
    pub trigger_type :DmaTrigger,
    /// Addresssing mode for source
    pub addressing_mode_src :AccessWidth,
    /// Addresssing mode for destination
    pub addressing_mode_dst :AccessWidth,
    /// Auto init mode
    pub auto_init :bool,
}
