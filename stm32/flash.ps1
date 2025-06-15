$ElfPath = $args[0]
# $BinPath = [System.IO.Path]::ChangeExtension($ElfPath, "bin")
# & arm-none-eabi-objcopy -O binary $ElfPath $BinPath

$UnixBinPath = $ElfPath -replace '\\', '/'

& openocd `
    -f interface/stlink.cfg `
    -f target/stm32f3x.cfg `
    -c "program $UnixBinPath verify reset exit"
