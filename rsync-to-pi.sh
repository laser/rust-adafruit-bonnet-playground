#!/usr/bin/env bash

rsync -avrh \
  --progress \
  --exclude target/ \
  --delete-excluded \
  --delete \
  --exclude *.sh \
  --exclude .git \
  --exclude .idea \
  --exclude .gitignore \
  ./ \
  pi@192.168.7.254:/home/pi/Code/adafruit-bonnet-playground/
