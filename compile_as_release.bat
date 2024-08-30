cargo rustc --release -- -Clink-args="/SUBSYSTEM:WINDOWS /ENTRY:mainCRTStartup"
:: compile it to an executable that doesn't open a console

if not exist build\ ( mkdir ".\build\" )
move /y "target\release\terrain-generator.exe" ".\build\"
explorer.exe .\build\