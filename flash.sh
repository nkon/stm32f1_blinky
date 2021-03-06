#!/bin/sh

#DIR=target/thumbv6m-none-eabi/debug
DIR=target/thumbv7m-none-eabi/debug

PROJECT=stm32f1_blinky

ELF_FILE=${DIR}/${PROJECT}

echo "Write $ELF_FILE"

openocd -f board/st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image $ELF_FILE" -c "reset halt" -c "reset run" -c "exit"
