// mod gyro_configuration {
//     // #[allow(non_camel_case_types)]
// // #[derive(Copy, Clone, Debug)]
// // /// Register 27: Gyro Config
//     pub struct GYRO_CONFIG;
//
//     impl GYRO_CONFIG {
//         pub const ADDR: u8 = 0x1b;
//         /// Gyro x axis self test bit
//         pub const XG_ST: u8 = 7;
//         /// Gyro y axis self test bit
//         pub const YG_ST: u8 = 6;
//         /// Gyro z axis self test bit
//         pub const ZG_ST: u8 = 5;
//         /// Gyro Config FS_SEL
//         pub const FS_SEL: BitBlock = BitBlock { bit: 4, length: 2 };
//     }
//     /// PI, f32
//     pub const PI: f32 = core::f32::consts::PI;
//
//     /// PI / 180, for conversion to radians
//     pub const PI_180: f32 = PI / 180.0;
//
//     /// High Byte Register Gyro x orientation
//     pub const GYRO_REGX_H: u8 = 0x43;
//     /// High Byte Register Gyro y orientation
//     pub const GYRO_REGY_H: u8 = 0x45;
//     /// High Byte Register Gyro z orientation
//     pub const GYRO_REGZ_H: u8 = 0x47;
//
//     pub const GYRO_SENS: (f32, f32, f32, f32) = (131., 65.5, 32.8, 16.4);
//     pub const ACCEL_SENS: (f32, f32, f32, f32) = (16384., 8192., 4096., 2048.);
//
//     /// High Byte Register Calc roll
//     pub const ACC_REGX_H : u8= 0x3b;
//
// }