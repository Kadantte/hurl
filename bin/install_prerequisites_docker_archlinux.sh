#!/bin/bash
set -eu

# Install packages
pacman -Syy --noconfirm
pacman -Sy --noconfirm bash python3 python-pip
python3 -m pip install --upgrade pip --quiet
