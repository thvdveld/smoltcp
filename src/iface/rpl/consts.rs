pub const SEQUENCE_WINDOW: u8 = 16;

pub const DEFAULT_MIN_HOP_RANK_INCREASE: u16 = 256;

pub const DEFAULT_DIO_INTERVAL_MIN: u32 = 12;
pub const DEFAULT_DIO_REDUNDANCY_CONSTANT: usize = 10;
/// This is 20 in the standard, but in Contiki they use:
pub const DEFAULT_DIO_INTERVAL_DOUBLINGS: u32 = 8;

pub const DEFAULT_RPL_INSTANCE_ID: u8 = 0x1e;

pub const DEFAULT_RPL_PARENT_SET_SIZE: usize = 16;
pub const DEFAULT_RPL_ROUTING_TABLE_SIZE: usize = 32;
