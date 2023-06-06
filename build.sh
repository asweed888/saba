#!/bin/bash


if [ ! -e ./_test ]; then
	mkdir ./_test
fi

go build -o ./_test/saba main.go
