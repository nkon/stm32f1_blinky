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

mod lock;   // event.rs のために、トップレベル(main.rs)で mod lock; を呼ばなければならない。
mod event;
mod delay;

const MASK_MAIN: u32 = 0x00010000;
const EVENT_BUTTON: u32 = 0x0001;
const EVENT_LED_ON: u32 = 0x0002;
const EVENT_LED_OFF: u32 = 0x0003;

extern {
    static mut huart2 : uart::HandleTypeDef;
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let mut mode = 1000;
    let mut send_str = "slow";

    GPIOA().WritePin(gpio::PIN_5, gpio::Level::High);
    delay::send(mode, MASK_MAIN, EVENT_LED_OFF);

    loop {
        match event::catch(MASK_MAIN) {
            Some(EVENT_BUTTON) => {
                if mode == 1000 {
                    mode = 200;
                    send_str = "fast";
                } else {
                    mode = 1000;
                    send_str = "slow";
                }
            },
            Some(EVENT_LED_ON) => {
                GPIOA().WritePin(gpio::PIN_5, gpio::Level::High);
                delay::send(mode, MASK_MAIN, EVENT_LED_OFF);
                unsafe {huart2.Transmit_IT(send_str);}
            },
            Some(EVENT_LED_OFF) => {
                GPIOA().WritePin(gpio::PIN_5, gpio::Level::Low);
                delay::send(mode, MASK_MAIN, EVENT_LED_ON);
                unsafe {huart2.Transmit_IT(send_str);}
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
