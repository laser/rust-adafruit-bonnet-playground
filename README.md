# adafruit-bonnet-playground

> Drive an Adafruit 128x64 OLED Bonnet with Rust on as Raspberry Pi Zero W

## Capabilities

So far, this program hits some JSON-returning web service in a loop and displays
the time (parsed out of the HTTP response) on the screen. Not very impressive.

## Building

I could never get cross compilation working on OSX, so instead I build from
within a Docker container. Note that I'm compiling for the ARMv6Z. If you want
something different, you'll need to swap the Rust target.

```
./build-and-deploy pi@192.168.7.243
```
