// Register addresses for the ICM42670-P sensor

pub const MCLK_RDY: u8 = 0x00;
pub const SIGNAL_PATH_RESET: u8 = 0x02;
pub const TEMP_DATA1: u8 = 0x09;
pub const TEMP_DATA0: u8 = 0x0A;
pub const ACCEL_DATA_X1: u8 = 0x0B;
pub const ACCEL_DATA_X0: u8 = 0x0C;
pub const ACCEL_DATA_Y1: u8 = 0x0D;
pub const ACCEL_DATA_Y0: u8 = 0x0E;
pub const ACCEL_DATA_Z1: u8 = 0x0F;
pub const ACCEL_DATA_Z0: u8 = 0x10;
pub const GYRO_DATA_X1: u8 = 0x11;
pub const GYRO_DATA_X0: u8 = 0x12;
pub const GYRO_DATA_Y1: u8 = 0x13;
pub const GYRO_DATA_Y0: u8 = 0x14;
pub const GYRO_DATA_Z1: u8 = 0x15;
pub const GYRO_DATA_Z0: u8 = 0x16;
pub const PWR_MGMT0: u8 = 0x1F;
pub const GYRO_CONFIG0: u8 = 0x20;
pub const ACCEL_CONFIG0: u8 = 0x21;
pub const WHO_AM_I: u8 = 0x75;
