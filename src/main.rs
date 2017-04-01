#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]
#![feature(core_intrinsics)] 
use core::intrinsics::volatile_store;

pub const PERIPH_BASE: u32 = 0x40000000;

pub const APB2PERIPH_BASE: u32 = PERIPH_BASE + 0x10000;
pub const GPIOA_BASE: u32 = APB2PERIPH_BASE + 0x0800;
pub const CRL_OFFSET: u32 = 0x00;
pub const BSRR_OFFSET: u32 = 0x10;
pub const GPIO_PIN_5: u32 = 5;

pub const AHBPERIPH_BASE: u32 = PERIPH_BASE + 0x20000;
pub const RCC_BASE: u32 = AHBPERIPH_BASE + 0x1000;
pub const CR_OFFSET: u32 = 0x00;
pub const CFGR_OFFSET: u32 = 0x04;
pub const CIR_OFFSET: u32 = 0x08;
pub const APB2ENR_OFFSET: u32 = 0x18;

pub const FLASH_BASE: u32 = 0x08000000;
pub const VECT_TAB_OFFSET: u32 = 0x0;
pub const VTOR_OFFSET: u32 = 8;

pub const SCS_BASE: u32 = 0xE000E000;
pub const SCB_BASE: u32 = SCS_BASE + 0x0D00;


#[no_mangle]
#[start]
pub extern fn main() {
    let apb2enr = (RCC_BASE + APB2ENR_OFFSET) as *mut u32;
    let crl = (GPIOA_BASE + CRL_OFFSET) as *mut u32;

    unsafe {
        volatile_store(apb2enr, *apb2enr | (1 << 2));
        volatile_store(crl, *crl & (!(6 << 20)));
        volatile_store(crl, *crl | (2 << 20));
    }

    let bsrr = (GPIOA_BASE + BSRR_OFFSET) as *mut u32;

    loop {
        unsafe {
            volatile_store(bsrr, 1 << GPIO_PIN_5); // 点灯
        }
        for _ in 1..400000 {
            unsafe {
                asm!("");
            }
        }

        unsafe {
            volatile_store(bsrr, (1 << GPIO_PIN_5) << 16); // 消灯
        }

        for _ in 1..400000 {
            unsafe {
                asm!("");
            }
        }
    }
}

#[allow(non_snake_case)] 
#[no_mangle]
pub extern fn SystemInit(){
    let rcc_cr   = (RCC_BASE+CR_OFFSET) as *mut u32;
    let rcc_cfgr = (RCC_BASE+CFGR_OFFSET) as *mut u32;
    let rcc_cir  = (RCC_BASE+CIR_OFFSET) as *mut u32;
    let scb_vtor = (SCB_BASE+VTOR_OFFSET) as *mut u32;

    unsafe {
        volatile_store(rcc_cr, *rcc_cr | 0x00000001);
        volatile_store(rcc_cfgr, *rcc_cfgr & 0xf0f0000);
        volatile_store(rcc_cr, *rcc_cr & 0xfef6ffff);
        volatile_store(rcc_cr, *rcc_cr & 0xfffbffff);
        volatile_store(rcc_cfgr, *rcc_cfgr & 0xff80ffff);
        volatile_store(rcc_cir, 0x009f0000);
        volatile_store(scb_vtor, FLASH_BASE | VECT_TAB_OFFSET);
    }
}

#[lang="panic_fmt"]
pub fn panic_fmt() -> ! {
    loop {}
}

#[lang="eh_personality"]
extern "C" fn eh_personality() {}
