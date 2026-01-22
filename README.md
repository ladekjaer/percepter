# Percepter

Running on a Raspberry Pi, *Percepter* is intended to collect data from sensors and to send these to a server running the *Herodot* service.

## Testing

Tests are run by

```bash
cargo test
```

## Building

Building locally is straightforward with `cargo build`. As the intended end system, the Raspberry Pi, is rather limited, development is expected to be done on a more powerful AMD64 system. Cross building is easily done using the Cross application, which can be installed by executing `cargo install cross`. Doing so enables simple cross compilation. To build for the Raspberry Pi 3B, run

```bash
cross build --target aarch64-unknown-linux-gnu
```

Note that since Cross uses it, Docker must be installed and running.

The build process can be expanded with a single line deployment. Say, a Raspberry Pi is running on the local network with hostname `raspberrypi` and username `pi`, build and deploy with

```bash
cross build --target aarch64-unknown-linux-gnu && scp target/aarch64-unknown-linux-gnu/debug/percepters pi@raspberrypi.local:/home/pi/percepter
```

Using `&&` ensures that the deployment only happens if the build succeeds.

## Preparing a Raspberry Pi

1. Use the official [Raspberry Pi Imager](https://www.raspberrypi.com/software/) to prepare a MicroSD card with the latest version of Raspbian. Use it to set settings for WiFi and time zone. Set up SSH with your public key as authorized. Optionally also configure [Raspberry Pi Connect](https://connect.raspberrypi.com/).
2. Boot the Raspberry Pi and connect to it via SSH.
3. Update the system with `sudo apt update && sudo apt upgrade`.
4. Enable I2C.
   1. run `sudo raspi-config`.
   2. Navigate to `Interfacing Options`.
   3. Enable I2C.
5. Enable 1-Wire
   1. run `sudo raspi-config`, to reopen the configuration menu if closed.
   2. Navigate to `Interfacing Options`.
   3. Enable 1-Wire.
   4. Restart the Raspberry Pi.
6. Deploy the compiled binary to the Raspberry Pi. As an example, see the single-line compile and deploy above.

Percepter is now simply runnable by

```sh
./percepters
```

## Miscellaneous

An easy overview of the Raspberry Pi GPIO pins can be found [Raspberry Pi Pinout](https://pinout.xyz/).

```sh
# Get the Raspberry Pi serial number
cat /proc/cpuinfo | grep Serial

# Setting up Raspberry Pi Connect, if not done during the MicroSD card preparation
rpi-connect on
rpi-connect signin
sudo systemctl enable rpi-connect

# List the connected 1-Wire devices
 ls /sys/bus/w1/devices/

# Detect connected I2C devices
sudo apt-get install i2c-tools
sudo i2cdetect -y 1

# Start the interactive Raspberry Pi configuration utility
sudo raspi-config

# Shutting down or rebooting the Raspberry Pi
sudo reboot
sudo shutdown -h
sudo halt

# See the current kernel version
uname -a

# Manually add modules for 1-Wire support to the kernel
sudo modprobe w1-gpio
sudo modprobe w1-therm

# View service logs, fx for the SSH daemon
journalctl -u ssh.service

# Manage services, fx
sudo systemctl start ssh.service
sudo systemctl stop ssh.service

# Read all connected 1-Wire devices od type DS18B20
cat /sys/bus/w1/devices/28-*/w1_slave

# See the special boot configuration file
sudo vi /boot/firmware/config.txt
```