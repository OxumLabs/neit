@echo off
setlocal

REM Define the list of items to remove
set ITEMS_TO_REMOVE=temp.asm
set ITEMS_TO_REMOVE=%ITEMS_TO_REMOVE%;target

REM Loop through the list and remove each item
for %%I in (%ITEMS_TO_REMOVE%) do (
    if exist "%%I" (
        if exist "%%I\" (
            echo Removing directory: %%I
            rmdir /s /q "%%I"
        ) else (
            echo Removing file: %%I
            del /q "%%I"
        )
    ) else (
        echo Item not found: %%I
    )
)

echo Cleanup completed.

endlocal
