#!/bin/bash

set -e

version=$(curl -s https://github.com/asweed888/saba/releases.atom | grep -o -E "releases/tag/v[0-9]+\.[0-9]+\.[0-9]+" | sed 's/releases\/tag\///' | head -n 1)

echo "latest version is ${version}"
