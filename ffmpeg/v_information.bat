@echo off
ffprobe -v error -select_streams v:0 -show_entries stream=width,height,duration,bit_rate -of default=noprint_wrappers=1 %1
