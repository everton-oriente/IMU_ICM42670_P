// Import the trait or interface

use embedded_hal::i2c::I2c;
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

    pub fn init(&mut self) -> Result<(), T::Error> {
        // Recommended Initialization sequence for the ICM42670-P sensor
        // Read WHO_AM_I register to verify communication should return 0x68
        let mut buf: [u8; 1] = [0u8; 1];
        self.read_bytes(reg::WHO_AM_I, &mut buf)?;
        // Trigger soft reset, write 0x01 to DEVICE_CONFIG register
        if buf[0] == reg::WHO_AM_I {
            self.write_byte(reg::DEVICE_CONFIG, 0x01)?;
        }
        // Wait for 5ms after reset
        // Turn sensor, on writing 0x0F to PWR_MGMT0 register
        self.write_byte(reg::PWR_MGMT0, 0x9F)?;
        // Configure accelerometer and gyroscope settings as needed, GYRO_CONFIG0 and ACCEL_CONFIG0 registers
         self.write_byte(reg::GYRO_CONFIG0, 0x66)?;
        // Configure the low pass filter settings if needed, GYRO_CONFIG1 and ACCEL_CONFIG1 registers
        self.write_byte(reg::ACCEL_CONFIG0, 0x66)?; // Example: Set accelerometer to 4g range
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
        let ax = ((accel_data[0] as i16) << 8) | (accel_data[1] as i16);
        let ay = ((accel_data[2] as i16) << 8) | (accel_data[3] as i16);
        let az = ((accel_data[4] as i16) << 8) | (accel_data[5] as i16);
        Ok([ax, ay, az])
    }

    pub fn read_gyroscope(&mut self) -> Result<[i16; 3], T::Error> {
        // Read gyroscope data and return it as a tuple (x, y, z)
        // Placeholder return value
        let mut gyro_data: [u8; 6] = [0u8; 6];
        self.read_bytes( reg::GYRO_DATA_X1, &mut gyro_data)?;
        let gx = ((gyro_data[0] as i16) << 8) | (gyro_data[1] as i16);
        let gy = ((gyro_data[2] as i16) << 8) | (gyro_data[3] as i16);
        let gz = ((gyro_data[4] as i16) << 8) | (gyro_data[5] as i16);
        Ok([gx, gy, gz])
    }

    pub fn who_am_i(&mut self) -> Result<u8, T::Error> {
        let mut address: [u8; 1] = [0u8; 1];
        self.i2c_interface.write_read(self.device_address, &[reg::WHO_AM_I], &mut address)?;
        Ok(address[0])
        }
    pub fn read_gyro_config_0(&mut self) -> Result<u8, T::Error> {
        let mut config: [u8; 1] = [0u8; 1];
        self.i2c_interface.write_read(self.device_address, &[reg::GYRO_CONFIG0], &mut config)?;
        Ok(config[0])
    }
}
