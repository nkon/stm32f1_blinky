use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let cube_top = "../stm32cubef1/STM32Cube_FW_F1_V1.6.0";

    let out_dir = env::var("OUT_DIR").unwrap();

    let inc_dirs = [
        &format!("-I{}/Drivers/CMSIS/Device/ST/STM32F1xx/Include", cube_top),
        &format!("-I{}/Drivers/CMSIS/Include", cube_top),
        &format!("-I{}/Drivers/STM32F1xx_HAL_Driver/Inc", cube_top),
        "-Icubemx/Inc"
    ];

    let defines = [
        "-DSTM32F103xB"
    ];

    let srcs = [
        [&format!("cubemx/Src"), "stm32f1xx_hal_msp.c"],
        [&format!("cubemx/Src"), "stm32f1xx_it.c"],
        [&format!("cubemx/Src"), "system_stm32f1xx.c"],
        [&format!("cubemx/Src"), "main.c"],
        [&format!("cubemx/Src"), "gpio.c"],
        [&format!("cubemx/Src"), "usart.c"],
    ];

    let mut objs: Vec<String> = Vec::new();

    Command::new("arm-none-eabi-as")
        .args(&["-mcpu=cortex-m3", "-mthumb", "-mfloat-abi=soft"])
        .args(&["cubemx/startup/startup_stm32f103xb.s"])
        .args(&["-o"])
        .arg(&format!("{}/startup_stm32f103xb.o", out_dir))
        .status().unwrap();

    for src in &srcs {
        let obj = src[1].to_string().replace(".c", ".o");

        Command::new("arm-none-eabi-gcc")
            .arg("-c")
            .args(&["-mcpu=cortex-m3", "-mthumb", "-mfloat-abi=soft"])
            .args(&defines)
            .args(&inc_dirs)
            .arg(&format!("{}/{}",src[0], src[1]))
            .arg("-o")
            .arg(&format!("{}/{}", out_dir, obj))
            .status().unwrap();

        objs.push(obj);
    }

    Command::new("arm-none-eabi-ar")
        .args(&["crus", "libcube.a"])
        .arg(&format!("{}/startup_stm32f103xb.o", out_dir))
        .args(&objs)
        .current_dir(&Path::new(&out_dir))
        .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=cube");

    println!("cargo:rerun-if-changed=build.rs");
}

