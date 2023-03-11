#!/bin/bash

read -p "Name of the topic -> " topic

cargo new $topic

cd $topic

echo "# 🔤 ${topic^}" >> README.md

cd ..

echo "- [${topic^}](./${topic}/README.md)" >> README.md