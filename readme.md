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

`.cargo/config`の`[build]`節を書いておけば、`--target` オプションは不要だ。`xargo build`だけで良い。

```
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
rustflags = [
    "-C", "link-arg=-Tcubemx/STM32F103RBTx_FLASH.ld",
    "-C", "link-arg=-nostartfiles",
]

$ xargo build --verbose
```

## startupとリンク

startup(asm)とリンクする⇒クロスビルドが必要⇒build.rsを使う。

### Cargo.toml

`Cargo.toml` の`[package]`セクションに`build="build.rs"`とビルドスクリプトを指定する。

```
[package]
name = "stm32f1_blinky"
version = "0.1.0"
authors = ["KONDO Nobuhiro <kondou.nobuhiro@gmail.com>"]
build = "build.rs"
```

### build.rs

`build.rs`に、クロスビルドの方法を記述する。

```
use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    Command::new("arm-none-eabi-as")
        .args(&["-mcpu=cortex-m3", "-mthumb", "-mfloat-abi=soft"])
        .args(&["cubemx/startup/startup_stm32f103xb.s"])
        .args(&["-o"])
        .arg(&format!("{}/startup_stm32f103xb.o", out_dir))
        .status().unwrap();
    Command::new("arm-none-eabi-ar")
        .args(&["crus", "libcube.a", "startup_stm32f103xb.o"])
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=cube");

    println!("cargo:rerun-if-changed=build.rs");
}
```

* out_dirの場所を抽出する。そのために必要なライブラリを `use`しておく。
* `arm-none-eabi-as`を、引数を指定して実行する。
* `arm-none-eabi-ar`で、オブジェクトをライブラリにまとめる。よくわからないが、たとえ一つであっても、ライブラリにまとめる。
* `rustc-link-lib`キーワードでリンクするライブラリを指定する。
* `build.rs`が修正されたら再ビルドするように指定する。

### .cargo/config

「なぜか」`rustflags`に`-C opt-level=2`を付けないと(デフォルトの opt-level=0だと)リンクエラーになる。さすが Nightly(2017-04-01) だ。

```
[build]
target = "thumbv6m-none-eabi"

[target.thumbv6m-none-eabi]
rustflags = [
    "-Z", "no-landing-pads",
    "-C", "opt-level=2",
    "-C", "link-arg=-mcpu=cortex-m3",
    "-C", "link-arg=-mthumb",
    "-C", "link-arg=-mfloat-abi=soft",
    "-C", "link-arg=-specs=nosys.specs",
    "-C", "link-arg=-specs=nano.specs",
    "-C", "link-arg=-Tcubemx/STM32F103RBTx_FLASH.ld"
]
```

## 共通部分と個別部分

$(APP_DIR)/cubemx/ に生成するが Drivers/ 以下は共通なので、$(APP_DIR)からは削除して良い。

```
stm32f1xx/
├── Cargo.lock
├── Cargo.toml
├── cubemx/
│   └── Drivers/
│        ├── CMSIS/
│        │   ├── Device/
│        │   │   └── S/
│        │   │       └── STM32F1xx/
│        │   │           ├── Include/
│        │   │           │   ├── stm32f103xb.h
│        │   │           │   ├── stm32f1xx.h
│        │   │           │   └── system_stm32f1xx.h
│        │   │           └── Source/
│        │   │               └── Templates/
│        │   │                   └── gcc/
│        │   └── Include/
│        │       ├── arm_common_tables.h
│        │       ├── arm_const_structs.h
│        │       ├── arm_math.h
│        │       ├── cmsis_armcc.h
│        │       ├── cmsis_armcc_V6.h
│        │       ├── cmsis_gcc.h
│        │       ├── core_cm0.h
│        │       ├── core_cm0plus.h
│        │       ├── core_cm3.h
│        │       ├── core_cm4.h
│        │       ├── core_cm7.h
│        │       ├── core_cmFunc.h
│        │       ├── core_cmInstr.h
│        │       ├── core_cmSimd.h
│        │       ├── core_sc000.h
│        │       └── core_sc300.h
│        └── STM32F1xx_HAL_Driver/
│            ├── Inc/
│            │   ├── Legacy/
│            │   │   └── stm32_hal_legacy.h
│            │   ├── stm32f1xx_hal.h
│            │   ├── stm32f1xx_hal_cortex.h
│            │   ├── stm32f1xx_hal_def.h
│            │   ├── stm32f1xx_hal_dma.h
│            │   ├── stm32f1xx_hal_dma_ex.h
│            │   ├── stm32f1xx_hal_flash.h
│            │   ├── stm32f1xx_hal_flash_ex.h
│            │   ├── stm32f1xx_hal_gpio.h
│            │   ├── stm32f1xx_hal_gpio_ex.h
│            │   ├── stm32f1xx_hal_pwr.h
│            │   ├── stm32f1xx_hal_rcc.h
│            │   ├── stm32f1xx_hal_rcc_ex.h
│            │   ├── stm32f1xx_hal_tim.h
│            │   └── stm32f1xx_hal_tim_ex.h
│            └── Src/
│                ├── stm32f1xx_hal.c
│                ├── stm32f1xx_hal_cortex.c
│                ├── stm32f1xx_hal_dma.c
│                ├── stm32f1xx_hal_flash.c
│                ├── stm32f1xx_hal_flash_ex.c
│                ├── stm32f1xx_hal_gpio.c
│                ├── stm32f1xx_hal_gpio_ex.c
│                ├── stm32f1xx_hal_pwr.c
│                ├── stm32f1xx_hal_rcc.c
│                ├── stm32f1xx_hal_rcc_ex.c
│                ├── stm32f1xx_hal_tim.c
│                └── stm32f1xx_hal_tim_ex.c
├── readme.md
└── src/
     ├── hal/
     ├── lib.rs
     └── rs/
stm32f1_blinky/
├── Cargo.lock
├── Cargo.toml
├── cubemx/
│   ├── Inc/
│   │   ├── gpio.h
│   │   ├── main.h
│   │   ├── stm32f1xx_hal_conf.h
│   │   └── stm32f1xx_it.h
│   ├── STM32F103RBTx_FLASH.ld
│   ├── Src/
│   │   ├── gpio.c
│   │   ├── main.c
│   │   ├── stm32f1xx_hal_msp.c
│   │   ├── stm32f1xx_it.c
│   │   └── system_stm32f1xx.c
│   ├── cubemx.ioc
│   └── startup/
│       └── startup_stm32f103xb.s
├── readme.md
└── src/
     ├── main.rs
     └── mx/
         └── mod.rs
```

