#!/bin/bash

init_parser_crates() {
  local idl_dir="$1"

  for idl_file in "$idl_dir"/*; do
      # Skip if it's not a file
      if [ ! -f "$idl_file" ]; then
          continue
      fi
      
      # Get the filename without path
      filename=$(basename "$idl_file")    
      # Get the filename without extension
      package_name="${filename%.*}"

      echo "Creating crate yellowstone-vixen-$package_name-parser"
      
      cargo init "crates/$package_name-parser" --name "yellowstone-vixen-$package_name-parser" --lib
  done
}

# Run for both folders
init_parser_crates "idls/anchor"
init_parser_crates "idls/shank"

echo "All parser crates initialized successfully!"