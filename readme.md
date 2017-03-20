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

## first build with xargo

```
% xargo build --target thumbv6m-none-eabi --verbose
+ "rustc" "--print" "sysroot"
+ "rustc" "--print" "target-list"
+ "cargo" "build" "--target" "thumbv6m-none-eabi" "--verbose"
   Compiling stm32f1_blinky v0.1.0 (file://$(PROJECTS)/stm32f1_blinky)
     Running `rustc --crate-name stm32f1_blinky src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=a521522334486350 -C extra-filename=-a521522334486350 --out-dir $(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps --target thumbv6m-none-eabi -L dependency=$(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps -L dependency=$(PROJECTS)/stm32f1_blinky/target/debug/deps --sysroot $(HOME)/.xargo`
error: linking with `arm-none-eabi-gcc` failed: exit code: 1
  |
  = note: "arm-none-eabi-gcc" "-L" "$(HOME)/.xargo/lib/rustlib/thumbv6m-none-eabi/lib" "$(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps/stm32f1_blinky-a521522334486350.0.o" "-o" "$(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps/stm32f1_blinky-a521522334486350" "-Wl,--gc-sections" "-nodefaultlibs" "-L" "$(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps" "-L" "$(PROJECTS)/stm32f1_blinky/target/debug/deps" "-L" "$(HOME)/.xargo/lib/rustlib/thumbv6m-none-eabi/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "$(HOME)/.xargo/lib/rustlib/thumbv6m-none-eabi/lib/libcore-757c4ccf137254cc.rlib"
  = note: /usr/lib/gcc/arm-none-eabi/4.9.3/../../../arm-none-eabi/lib/crt0.o: In function `_start':
          /build/newlib-5zwpxE/newlib-2.2.0+git20150830.5a3d536/build/arm-none-eabi/libgloss/arm/../../../../libgloss/arm/crt0.S:269: undefined reference to `memset'
          /build/newlib-5zwpxE/newlib-2.2.0+git20150830.5a3d536/build/arm-none-eabi/libgloss/arm/../../../../libgloss/arm/crt0.S:419: undefined reference to `atexit'
          /build/newlib-5zwpxE/newlib-2.2.0+git20150830.5a3d536/build/arm-none-eabi/libgloss/arm/../../../../libgloss/arm/crt0.S:421: undefined reference to `__libc_init_array'
          /build/newlib-5zwpxE/newlib-2.2.0+git20150830.5a3d536/build/arm-none-eabi/libgloss/arm/../../../../libgloss/arm/crt0.S:427: undefined reference to `exit'
          /build/newlib-5zwpxE/newlib-2.2.0+git20150830.5a3d536/build/arm-none-eabi/libgloss/arm/../../../../libgloss/arm/crt0.S:427: undefined reference to `__libc_fini_array'
          collect2: error: ld returned 1 exit status
          

error: aborting due to previous error

error: Could not compile `stm32f1_blinky`.

Caused by:
  process didn't exit successfully: `rustc --crate-name stm32f1_blinky src/main.rs --crate-type bin --emit=dep-info,link -C debuginfo=2 -C metadata=a521522334486350 -C extra-filename=-a521522334486350 --out-dir $(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps --target thumbv6m-none-eabi -L dependency=$(PROJECTS)/stm32f1_blinky/target/thumbv6m-none-eabi/debug/deps -L dependency=$(PROJECTS)/stm32f1_blinky/target/debug/deps --sysroot $(HOME)/.xargo` (exit code: 101)
```

リンカがうまく動いていない。

## .cargo/config

`.cargo/config`でリンカへのフラグを指定する。

```
[target.thumbv6m-none-eabi]
rustflags = [
    "-C", "link-arg=-Tlayout.ld",
    "-C", "link-arg=-nostartfiles",
]
```

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


