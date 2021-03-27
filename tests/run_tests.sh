#!/bin/bash

# Clear previous results if any
rm -rf /tmp/monke/*

# Create directories
for s in stage*/; do mkdir -p -- "/tmp/monke/$s/valid";mkdir -p -- "/tmp/monke/$s/invalid"; done

# Compile all valid tests and report errors
for f in stage*/valid/*.gg; do cargo run -- "$f" -o "/tmp/monke/${f%.gg}" 2>>/tmp/monke/valid.txt; echo -e "\n" >> /tmp/monke/valid.txt; done

# Compile all invalid tests and report errors
for f in stage*/invalid/*.gg; do cargo run -- "$f" -o "/tmp/monke/${f%.gg}" 2>>/tmp/monke/invalid.txt; echo -e "\n" >> /tmp/monke/invalid.txt; done
