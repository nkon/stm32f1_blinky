#!/bin/sh

DIR=target/thumbv6m-none-eabi/debug/deps/

ELF_FILE=${DIR}`ls -t ${DIR} | tail -1`
## ELF_FILE=target/thumbv6m-none-eabi/debug/deps/stm32f1_blinky-bbd79dc6fc972825

echo "Write $ELF_FILE"

openocd -f board/st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image $ELF_FILE" -c "reset halt" -c "reset run" -c "exit"
