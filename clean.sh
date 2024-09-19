#!/bin/bash

# Define the list of items to remove
ITEMS_TO_REMOVE=(
    "temp.asm"        # File
    "./output"         # Directory
    "target/"         # Directory
)

# Loop through the list and remove each item
for item in "${ITEMS_TO_REMOVE[@]}"; do
    if [ -d "$item" ]; then
        echo "Removing directory: $item"
        rm -rf "$item"
    elif [ -f "$item" ]; then
        echo "Removing file: $item"
        rm -f "$item"
    else
        echo "Item not found: $item"
    fi
done

echo "Cleanup completed."
