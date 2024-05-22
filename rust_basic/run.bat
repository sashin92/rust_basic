@echo off
rustc %1 -o output.exe
output.exe
del output.exe
del output.pdb
pause