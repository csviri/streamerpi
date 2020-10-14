# streampi

A tiny pet project to learn rust and stream videos from my raspberry pi :)

## Run

`./streampi ~/dir_with_my_movies`

## What does it do
- Exposes the configured dir and sub-dirs through a minimalistic web page
- Streams (don't transcode - so its kinda limited) videos through html5 video   

Tested on Linux + Chrome.

## Cross Compile to Pi
this worked with some minor tweaks:

https://hackernoon.com/compiling-rust-for-the-raspberry-pi-49fdcd7df658

Build for ARM: `cargo build --target=armv7-unknown-linux-gnueabihf --release` 
