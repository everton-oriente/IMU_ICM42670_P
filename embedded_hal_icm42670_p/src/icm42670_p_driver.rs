// Import the trait or interface

use embedded_hal::i2c::I2c;
use embedded_hal::delay::DelayNs;
use super::reg;


// Implementation of the driver for the ICM42670-P sensor
pub struct Icm42670P<T> {   // This defines a generic struct for ICM42670-P, where T represents the type of bus interface (e.g., I2C, SPI),
                        // to be filled in when the struct is instantiated. This allows for flexibility in supporting different communication protocols.
    device_address: u8, // The I2C address of the sensor, whose type is fixed to u8. This is typically a fixed value defined by the sensor's datasheet.
    i2c_interface: T,   // This field stores the communication interface (e.g., I2C or SPI), and it is type is generic (hnot fixed). It will be determined when we use
                        // the struct , for example, it could be an I2C peripheral that implements the embedded-hal::i2c::I2c trait.
}

impl <T:I2c> Icm42670P<T> {
    // Methods for initializing the sensor, reading data, configuring settings, etc.
    pub fn new( i2c_interface: T, device_address: u8) -> Self {
        // Create a new instance of the driver with default settings
        Self {
            device_address,
            i2c_interface
        }
    }

    pub fn init<D: DelayNs>(&mut self, delay: &mut D) -> Result<(), T::Error> {
        let mut buf: [u8; 1] = [0u8; 1];

        self.write_byte(reg::PWR_MGMT0, 0x00)?;  // Gyro LN + Accel LN
        delay.delay_ms(50);
        
        // ✅ STEP 1: Wait for MCLK ready FIRST
        self.read_bytes(reg::MCLK_RDY, &mut buf)?;

        for _ in 0..100 {
            if (buf[0] & 0x01) == 1 {
                break;
            }
            self.read_bytes(reg::MCLK_RDY, &mut buf)?;
            delay.delay_ms(20);
        }
        
        // ✅ STEP 2: Verify WHO_AM_I
        self.read_bytes(reg::WHO_AM_I, &mut buf)?;

        // ✅ STEP 3: Reset signal paths
        self.write_byte(reg::SIGNAL_PATH_RESET, 0x10)?;

        // Wait for 50 ms after reset
        delay.delay_ms(50);
        
        // ✅ STEP 4: Wait for MCLK again
        self.read_bytes(reg::MCLK_RDY, &mut buf)?;
        for _ in 0..100 {
            if (buf[0] & 0x01) == 1 {
                break;
            }
            self.read_bytes(reg::MCLK_RDY, &mut buf)?;
            delay.delay_ms(20);
        }
        
        // Wait for 50 ms after reset
        delay.delay_ms(50);

        // ✅ STEP 6: Configure gyroscope (±2000 dps, 1600 Hz)
        self.write_byte(reg::GYRO_CONFIG0, 0x65)?;  // 011 (±2000) + 1111 (1600Hz)
        delay.delay_ms(5);
        
        // ✅ STEP 7: Configure accelerometer (±16g, 1600 Hz)
        self.write_byte(reg::ACCEL_CONFIG0, 0x65)?;  // 011 (±16g) + 1111 (1600Hz)
        delay.delay_ms(5);

         // ✅ STEP 5: Enable 6-axis sensors
        self.write_byte(reg::PWR_MGMT0, 0x1F)?;  // Gyro LN + Accel LN

        delay.delay_ms(50);
        
        Ok(())
    }

    fn write_byte(&mut self, reg_addr: u8, reg_value: u8)-> Result<(), T::Error> {
        // Write a byte to the specified register
        let buf = [reg_addr,reg_value];
        self.i2c_interface.write(self.device_address, &buf)?;
        Ok(())
    }

    fn read_bytes(&mut self, reg_addr: u8, buf: &mut [u8]) -> Result<(), T::Error> {
        // Read bytes from the specified register and store them in the buffer

        self.i2c_interface.write_read(self.device_address, &[reg_addr], buf)?;
        Ok(())
    }

    pub fn read_accelerometer(&mut self) -> Result<[i16; 3], T::Error> {
        // Read accelerometer data and return it as a tuple (x, y, z)
        // Placeholder return value
        let mut accel_data: [u8; 6] = [0u8; 6];
        self.read_bytes(reg::ACCEL_DATA_X1, &mut accel_data)?;
        let ax = i16::from_be_bytes([accel_data[0], accel_data[1]]);
        let ay = i16::from_be_bytes([accel_data[2], accel_data[3]]);
        let az = i16::from_be_bytes([accel_data[4], accel_data[5]]);
        Ok([ax, ay, az])
    }

    pub fn read_gyroscope(&mut self) -> Result<[i16; 3], T::Error> {
        // Read gyroscope data and return it as a tuple (x, y, z)
        // Placeholder return value
        let mut gyro_data: [u8; 6] = [0u8; 6];
        self.read_bytes( reg::GYRO_DATA_X1, &mut gyro_data)?;
        let gx = i16::from_be_bytes([gyro_data[0], gyro_data[1]]);
        let gy = i16::from_be_bytes([gyro_data[2], gyro_data[3]]);
        let gz = i16::from_be_bytes([gyro_data[4], gyro_data[5]]);
        Ok([gx, gy, gz])
    }

    pub fn read_who_am_i(&mut self) -> Result<u8, T::Error> {
        let mut address: [u8; 1] = [0u8; 1];
        self.i2c_interface.write_read(self.device_address, &[reg::WHO_AM_I], &mut address)?;
        Ok(address[0])
        }
    
    pub fn read_gyro_config_0(&mut self) -> Result<u8, T::Error> {
        let mut config: [u8; 1] = [0u8; 1];
        self.i2c_interface.write_read(self.device_address, &[reg::GYRO_CONFIG0], &mut config)?;
        Ok(config[0])
    }

    pub fn read_mclk_rdy(&mut self) -> Result<u8, T::Error> {
        let mut status: [u8; 1] = [0u8; 1];
        self.i2c_interface.write_read(self.device_address, &[reg::MCLK_RDY], &mut status)?;
        Ok(status[0])
    }

}
