extern crate embedded_hal;
use embedded_hal::adc::OneShot;
extern crate linux_embedded_hal;
#[macro_use(block)]
extern crate nb;
extern crate ads1x1x;

extern crate pwm_pca9685 as pca9685;
use pca9685::{ Channel, Pca9685};

use ads1x1x::{channel, Ads1x1x};
use linux_embedded_hal::I2cdev;

use std::{thread, time};

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = pca9685::SlaveAddr::default();
    let mut pwm = Pca9685::new(dev, address);
    let mut on = [0; 16];
    let mut off = [2047; 16];
    pwm.set_all_on_off(&on, &off);

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = ads1x1x::SlaveAddr::default();
    let mut adc = Ads1x1x::new_ads1115(dev, address);
    loop {
        let values = [
            block!(adc.read(&mut channel::SingleA0)).unwrap(),
            block!(adc.read(&mut channel::SingleA1)).unwrap(),
            block!(adc.read(&mut channel::SingleA2)).unwrap(),
            block!(adc.read(&mut channel::SingleA3)).unwrap(),
        ];
        println!("---");
        for (channel, value) in values.iter().enumerate() {
            println!("Channel {}: {}", channel, value);
        }
        thread::sleep(time::Duration::from_millis(100));
    }
    // get I2C device back
    let _dev = adc.destroy_ads1115();
}
