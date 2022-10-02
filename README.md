# QMK Programmable Buttons Control OBS and other programs

This project uses libinput to read the special key codes that are outside the normal keyboard range. It then uses the key codes to control OBS and other programs (sending post requests to my smart home server to control the lights etc.)

The qmk [programmable button feature](https://docs.qmk.fm/#/feature_programmable_button) supports up to 32 custom macros and all of these are supported by this project. You can see the guide on how to set up this feature and add the key codes to your keymap.c by reading the documentation or you can take a look at my [qmk fork for my macropad](https://github.com/ArtiomSu/qmk_firmware/blob/macropad_artiomsu/keyboards/macropad_artiomsu/keymaps/simple/keymap.c) for an example of how I did it.

The reason this feature is very good in my opinion is because these keys will not be picked up by the OS or any other programs as they are outside the normal keyboard range. This means that you can use these keys won't class with any other program. The only downside of this is you need something like this program to read the key codes and do something with them. So you can't use them to bind hotkeys in other programs directly.

This project was written in rust, I'm still learning rust so the code is not the best, this would be my first proper rust project. I'm open to any suggestions on how to improve the code, and make it more efficient.

Currently only Linux is supported due to a hard dependency on libinput. I will try to add support for windows in the future. Mostly I think I will just need to find an alternative to libinput, which shouldn't be too bad, the other libraries used in this project are cross platform I believe.

## Video Guide

[![first guide](https://img.youtube.com/vi/uFst1Hm4P9k/0.jpg)](https://www.youtube.com/watch?v=uFst1Hm4P9k)

## Customising

You can customise what you want the macros to do by editing the `src/programmable_key.rs` file. At the moment there is built in support for OBS and sending JSON post requests. You are welcome to submit a pull request to add support for other programs.

## Build

You will need to install cargo (rusts package manager) and rustc (rust compiler) to build this project. You can find instructions on how to do that [here](https://www.rust-lang.org/tools/install).

Once you have cargo and rustc installed, you can build the project by running `cargo build --release` in the root directory of the project.

To reduce the binary size further you can use upx.
`upx --best --lzma target/release/qmk_programmable_button`
