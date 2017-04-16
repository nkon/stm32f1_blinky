#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(non_snake_case)]

extern crate stm32cubef1;
use stm32cubef1::*;
use gpio;
use gpio::GPIOA;
use pwr;

mod lock;   // event.rs のために、トップレベル(main.rs)で mod lock; を呼ばなければならない。
mod event;

static mut COUNT: u32 = 0;
static mut MODE: u32 = 1000;

const MASK_MAIN: u32 = 0x00010000;
const EVENT_BUTTON: u32 = 0x0001;

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut mode = 1000;
    loop {
        // if let 構文を使う。
        if let Some(EVENT_BUTTON) = event::catch(MASK_MAIN) {
            if mode == 1000 {
                mode = 500;
            } else {
                mode = 1000;
            }
        }

        unsafe {MODE = mode;}

        pwr::EnterSLEEPMode(pwr::SLEEPENTRY_WFI);
    }

}

#[no_mangle]
pub extern "C" fn HAL_SYSTICK_Callback() {
    unsafe {
        COUNT += 1;
        if COUNT == MODE {
            GPIOA().WritePin(gpio::PIN_5, gpio::Level::High);
        }
        if COUNT > (2 * MODE) {
            GPIOA().WritePin(gpio::PIN_5, gpio::Level::Low);
            COUNT = 0;
        }
    }
}

#[no_mangle]
pub extern "C" fn HAL_GPIO_EXTI_Callback(gpio_pin: u16) {
    if gpio_pin == gpio::PIN_13 {
        event::send(MASK_MAIN, EVENT_BUTTON);
    }
}


#[lang="panic_fmt"]
pub fn panic_fmt() -> ! {
    loop {}
}

#[lang="eh_personality"]
extern "C" fn eh_personality() {}
