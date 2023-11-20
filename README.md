# avr_f64_testing

A repo that showcases a potential bug with `f64` on the Arduino MEGA 2560 R3 (and maybe others).

## Usage

1. Install the dependencies:
   - Debian: `sudo apt install avr-libc gcc-avr pkg-config avrdude libudev-dev build-essential`
   - Fedora: `sudo dnf install avr-libc avr-gcc pkg-config systemd-devel make automake gcc gcc-c++ kernel-devel avrdude`
   - macOS: `brew install avr-gcc avrdude`
   - Windows: haven't done this, but... `scoop install avr-gcc avrdude`
1. Ensure you have permissions to use Serial. On Linux, `sudo usermod --append --groups dialout (your username)`
1. Clone this repo
1. Plug in your Arduino board
1. Run the test: `cargo run -Z build-std=core --target avr-specs/avr-atme
ga2560.json --release`
