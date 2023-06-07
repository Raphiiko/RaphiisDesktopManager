cargo build --release
taskkill /im raphiis-desktop-manager.exe /f
xcopy /y .\target\release\raphiis-desktop-manager.exe "C:\Users\Raph\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\"
start /B "" "C:\Users\Raph\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\Startup\raphiis-desktop-manager.exe"
echo "Built and started desktop manager!"