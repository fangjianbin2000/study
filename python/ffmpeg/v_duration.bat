@echo off
REM "Return video duration in second"
ffprobe -v error -show_entries format=duration -of default=noprint_wrappers=1:nokey=1 %1