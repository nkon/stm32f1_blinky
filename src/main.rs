#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(non_snake_case)]

extern crate stm32cubef1;
use stm32cubef1::*;
use gpio;
use gpio::{GPIOA, GPIO_PIN_5};

#[no_mangle]
pub extern fn rust_main() {
    gpio::GPIOA_CLK_ENABLE();

    let mut gpio_init_struct = gpio::InitTypeDef{Pin: 0, Mode: 0, Pull: 0, Speed: 0};
    gpio_init_struct.Pin = 0x0020;
    gpio_init_struct.Mode = 0x0001;
    gpio_init_struct.Speed = 0x0002;

    gpio::Init(GPIOA(), &gpio_init_struct);
}

static mut COUNT :u32 = 0;

#[no_mangle]
pub extern fn HAL_SYSTICK_Callback() {
    unsafe {
        COUNT = COUNT + 1;
        if COUNT == 1000 {
            gpio::WritePin(GPIOA(), GPIO_PIN_5, 1);
        }
        if COUNT == 2000 {
            gpio::WritePin(GPIOA(), GPIO_PIN_5, 0);
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
