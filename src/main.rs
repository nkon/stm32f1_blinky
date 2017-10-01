#![no_std]
#![no_main]
#![feature(lang_items)]
#![allow(non_snake_case)]

extern crate stm32cubef1;
use stm32cubef1::*;
use gpio;
use gpio::GPIOA;
use pwr;
use hal;
use uart;
use lock;

mod event;
mod delay;

const MASK_MAIN: u32 = 0x00010000;
const EVENT_BUTTON: u32 = 0x0001;
const EVENT_LED_ON: u32 = 0x0002;
const EVENT_LED_OFF: u32 = 0x0003;
const EVENT_TX_OKOK: u32 = 0x0004;

// macro を使えば???
extern {
    static mut huart2 : uart::Handle;
}

pub fn HUART2() -> &'static mut uart::Handle {
    unsafe { &mut huart2 }
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut mode = 1000;

    GPIOA().WritePin(gpio::PIN_5, gpio::Level::High);
    delay::send(mode, MASK_MAIN, EVENT_LED_OFF);
    HUART2().Transmit_IT("ok1");

    loop {
        match event::catch(MASK_MAIN) {
            Some(EVENT_BUTTON) => {
                if mode == 1000 {
                    mode = 200;
//                    HUART2().SetBuffer();
//                    HUART2().Transmit_Q("OK2".as_bytes());
                } else {
                    mode = 1000;
                    HUART2().Transmit_IT("mode = 1000");
                    delay::send(100, MASK_MAIN, EVENT_TX_OKOK);
                    delay::send(200, MASK_MAIN, EVENT_TX_OKOK);
                    delay::send(1100, MASK_MAIN, EVENT_TX_OKOK);
                    delay::send(1200, MASK_MAIN, EVENT_TX_OKOK);                    
                }
            },
            Some(EVENT_LED_ON) => {
                GPIOA().WritePin(gpio::PIN_5, gpio::Level::High);
                delay::send(mode, MASK_MAIN, EVENT_LED_OFF);
                HUART2().Transmit_IT("ok4");
            },
            Some(EVENT_LED_OFF) => {
                GPIOA().WritePin(gpio::PIN_5, gpio::Level::Low);
                delay::send(mode, MASK_MAIN, EVENT_LED_ON);
                HUART2().Transmit_IT("ok5");
              },
            Some(EVENT_TX_OKOK) => {
                HUART2().Transmit_IT("OKOK");
              },
            _ => {},
        }

        pwr::EnterSLEEPMode(pwr::SLEEPENTRY_WFI);
    }

}

#[no_mangle]
pub extern "C" fn HAL_SYSTICK_Callback() {
    if let Some(ev) = delay::check_event(hal::GetTick()) {
        event::send(ev,ev);
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

#[no_mangle]
pub fn abort() {}
