#!/usr/bin/env just --justfile

timestamp := `date +"%Y-%m-%d\ %H:%M"`

# Backup assets
backup:
  ouch compress assets/player assets/logo assets/attacks ~/NAS/Documents/Ambition/Assets/{{timestamp}}.tar.gz
