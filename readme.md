## xargo

```
$ cargo install xargo
```

## make new project

```
$ xargo new stm32f1_blinky --bin
$ cd stm32f1_blinky
```

## install nightly

```
$ rustup install nightly
```

## use nightly toolchain

```
$ rustup override set nightly
```

## minimum main.rs

```
// src/main.rs
#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(start)]

#[no_mangle]
#[start]
pub extern fn main() {
	loop {}
}

#[lang="panic_fmt"]
pub fn panic_fmt() -> ! { loop {} }

#[lang="eh_personality"]
extern fn eh_personality () {}
```

## .cargo/config

* `.cargo/config`でリンカへのフラグを指定する。
* `./layout.ld`は、(現時点では)CubeMXが生成したものをコピー。

```
[target.thumbv6m-none-eabi]
rustflags = [
    "-C", "link-arg=-Tlayout.ld",
    "-C", "link-arg=-nostartfiles",
]
```

## first build

```
$ xargo build --target thumbv6m-none-eabi --verbose
+ "rustc" "--print" "sysroot"
+ "rustc" "--print" "target-list"
+ "cargo" "build" "--target" "thumbv6m-none-eabi" "--verbose"
   Compiling stm32f1_blinky v0.1.0 (file://$(PROJECTS)/stm32f1_blinky)
     Running `rustc --crate-name stm32f1_blinky src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=a521522334486350 -C extra-filename=-a521522334486350 --out-dir $(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps --target thumbv6m-none-eabi -L dependency=$(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps -L dependency=$(PROJECTS)/stm32f1_blinky/target/debug/deps -C link-arg=-Tlayout.ld -C link-arg=-nostartfiles --sysroot $(HOME)/.xargo`
    Finished dev [unoptimized + debuginfo] target(s) in 0.34 secs
```

うまくリンク出来た。


```
stm32f1xx/
├── Cargo.lock
├── Cargo.toml
├── cubemx
│   ├── Drivers
│   │   ├── CMSIS
│   │   │   ├── Device
│   │   │   │   └── ST
│   │   │   │       └── STM32F1xx
│   │   │   │           ├── Include
│   │   │   │           │   ├── stm32f103xb.h
│   │   │   │           │   ├── stm32f1xx.h
│   │   │   │           │   └── system_stm32f1xx.h
│   │   │   │           └── Source
│   │   │   │               └── Templates
│   │   │   │                   └── gcc
│   │   │   └── Include
│   │   │       ├── arm_common_tables.h
│   │   │       ├── arm_const_structs.h
│   │   │       ├── arm_math.h
│   │   │       ├── cmsis_armcc.h
│   │   │       ├── cmsis_armcc_V6.h
│   │   │       ├── cmsis_gcc.h
│   │   │       ├── core_cm0.h
│   │   │       ├── core_cm0plus.h
│   │   │       ├── core_cm3.h
│   │   │       ├── core_cm4.h
│   │   │       ├── core_cm7.h
│   │   │       ├── core_cmFunc.h
│   │   │       ├── core_cmInstr.h
│   │   │       ├── core_cmSimd.h
│   │   │       ├── core_sc000.h
│   │   │       └── core_sc300.h
│   │   └── STM32F1xx_HAL_Driver
│   │       ├── Inc
│   │       │   ├── Legacy
│   │       │   │   └── stm32_hal_legacy.h
│   │       │   ├── stm32f1xx_hal.h
│   │       │   ├── stm32f1xx_hal_cortex.h
│   │       │   ├── stm32f1xx_hal_def.h
│   │       │   ├── stm32f1xx_hal_dma.h
│   │       │   ├── stm32f1xx_hal_dma_ex.h
│   │       │   ├── stm32f1xx_hal_flash.h
│   │       │   ├── stm32f1xx_hal_flash_ex.h
│   │       │   ├── stm32f1xx_hal_gpio.h
│   │       │   ├── stm32f1xx_hal_gpio_ex.h
│   │       │   ├── stm32f1xx_hal_pwr.h
│   │       │   ├── stm32f1xx_hal_rcc.h
│   │       │   ├── stm32f1xx_hal_rcc_ex.h
│   │       │   ├── stm32f1xx_hal_tim.h
│   │       │   └── stm32f1xx_hal_tim_ex.h
│   │       └── Src
│   │           ├── stm32f1xx_hal.c
│   │           ├── stm32f1xx_hal_cortex.c
│   │           ├── stm32f1xx_hal_dma.c
│   │           ├── stm32f1xx_hal_flash.c
│   │           ├── stm32f1xx_hal_flash_ex.c
│   │           ├── stm32f1xx_hal_gpio.c
│   │           ├── stm32f1xx_hal_gpio_ex.c
│   │           ├── stm32f1xx_hal_pwr.c
│   │           ├── stm32f1xx_hal_rcc.c
│   │           ├── stm32f1xx_hal_rcc_ex.c
│   │           ├── stm32f1xx_hal_tim.c
│   │           └── stm32f1xx_hal_tim_ex.c
├── readme.md
└── src
     ├── hal
     ├── lib.rs
     └── rs
stm32f1_blinky/
├── Cargo.lock
├── Cargo.toml
├── cubemx
│   ├── Inc
│   │   ├── gpio.h
│   │   ├── main.h
│   │   ├── stm32f1xx_hal_conf.h
│   │   └── stm32f1xx_it.h
│   ├── STM32F103RBTx_FLASH.ld
│   ├── Src
│   │   ├── gpio.c
│   │   ├── main.c
│   │   ├── stm32f1xx_hal_msp.c
│   │   ├── stm32f1xx_it.c
│   │   └── system_stm32f1xx.c
│   ├── cubemx.ioc
│   └── startup
│       └── startup_stm32f103xb.s
├── readme.md
└── src
     ├── main.rs
     └── mx
         └── mod.rs

```