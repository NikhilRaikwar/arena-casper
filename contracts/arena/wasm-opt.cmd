@echo off
setlocal
set "input="
set "output="

:parse
if "%~1"=="" goto done
if "%~1"=="-o" (
  set "output=%~2"
  shift
  shift
  goto parse
)
if not "%~1"=="-Oz" if not "%~1"=="-Os" if not "%~1"=="-O" if not "%~1"=="--enable-bulk-memory" (
  if exist "%~1" set "input=%~1"
)
shift
goto parse

:done
if not "%output%"=="" if not "%input%"=="" copy /Y "%input%" "%output%" >nul
exit /B 0
