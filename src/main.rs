#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]

extern crate stm32cubef1;
use stm32cubef1::*;
use stm32f1xx_hal_gpio::{GPIOA, GPIO_PIN_5};

#[no_mangle]
#[start]
pub extern fn main() {
    stm32f1xx_hal_gpio::GPIOA_CLK_ENABLE();

    let mut gpio_init_struct = stm32f1xx_hal_gpio::GPIO_InitTypeDef{Pin: 0, Mode: 0, Pull: 0, Speed: 0};
    gpio_init_struct.Pin = 0x0020;
    gpio_init_struct.Mode = 0x0001;
    gpio_init_struct.Speed = 0x0002;

    stm32f1xx_hal_gpio::Init(GPIOA(), &gpio_init_struct);

    loop {
        stm32f1xx_hal_gpio::WritePin(GPIOA(), GPIO_PIN_5, 1);
        for _ in 1..400000 {
            unsafe {
                asm!("");
            }
        }

        stm32f1xx_hal_gpio::WritePin(GPIOA(), GPIO_PIN_5, 0);
        for _ in 1..400000 {
            unsafe {
                asm!("");
            }
        }
    }
}

#[lang="panic_fmt"]
pub fn panic_fmt() -> ! {
    loop {}
}

#[lang="eh_personality"]
extern "C" fn eh_personality() {}
