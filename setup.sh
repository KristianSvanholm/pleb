#!/bin/sh

# Install nix pkg manager (Single-user install)
#sh <(curl -L https://nixos.org/nix/install) --no-daemon

# Download
git clone https://github.com/KristianSvanholm/thesis.git energy_benchmark && cd energy_benchmark

# Install dependencies and enter temporary shell
nix develop --extra-experimental-features "nix-command flakes"

