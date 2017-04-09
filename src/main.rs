#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(non_snake_case)]

extern crate stm32cubef1;
use stm32cubef1::*;
use gpio;
use gpio::{GPIOA};

#[no_mangle]
pub extern fn rust_main() {
    let mut gpio_init_struct = gpio::InitTypeDef{Pin: 0, Mode: 0, Pull: 0, Speed: 0};
    gpio_init_struct.Pin = gpio::PIN_5 as u32;
    gpio_init_struct.Mode = gpio::MODE_OUTPUT_PP;
    gpio_init_struct.Speed = gpio::SPEED_FREQ_LOW;

    gpio::Init(GPIOA(), &gpio_init_struct);
}

static mut COUNT :u32 = 0;

#[no_mangle]
pub extern fn HAL_SYSTICK_Callback() {
    unsafe {
        COUNT = COUNT + 1;
        if COUNT == 1000 {
            gpio::WritePin(GPIOA(), gpio::PIN_5, 1);
        }
        if COUNT == 2000 {
            gpio::WritePin(GPIOA(), gpio::PIN_5, 0);
            COUNT = 0;
        }
    }
}

#[lang="panic_fmt"]
pub fn panic_fmt() -> ! {
    loop {}
}

#[lang="eh_personality"]
extern "C" fn eh_personality() {}
