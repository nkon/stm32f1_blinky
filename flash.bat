set DIR=target/thumbv7m-none-eabi/debug
set PROJECT=stm32f1_blinky
set ELF_FILE=%DIR%/%PROJECT%
set OPENOCD_DIR="C:\Users\kondo\bin\ocd"
set OPENOCD=%OPENOCD_DIR%\openocd.exe

%OPENOCD% -s %OPENOCD_DIR%\tcl -f board\st_nucleo_f103rb.cfg -c "init" -c "reset init" -c "stm32f1x mass_erase 0" -c "flash write_image %ELF_FILE%" -c "reset halt" -c "reset run" -c "exit"

