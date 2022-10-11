# QMK Programmable Buttons Control OBS and other programs

This project uses libinput to read the special key codes that are outside the normal keyboard range. It then uses the key codes to control OBS and other programs (sending post requests to my smart home server to control the lights etc.)

The qmk [programmable button feature](https://docs.qmk.fm/#/feature_programmable_button) supports up to 32 custom macros and all of these are supported by this project. You can see the guide on how to set up this feature and add the key codes to your keymap.c by reading the documentation or you can take a look at my [qmk fork for my macropad](https://github.com/ArtiomSu/qmk_firmware/blob/macropad_artiomsu/keyboards/macropad_artiomsu/keymaps/simple/keymap.c) for an example of how I did it.

The reason this feature is very good in my opinion is because these keys will not be picked up by the OS or any other programs as they are outside the normal keyboard range. This means that you can use these keys won't class with any other program. The only downside of this is you need something like this program to read the key codes and do something with them. So you can't use them to bind hotkeys in other programs directly.

This project was written in rust, I'm still learning rust so the code is not the best, this would be my first proper rust project. I'm open to any suggestions on how to improve the code, and make it more efficient.

Currently Linux and Windows is supported. Linux support is better, I still need to improve windows version more but it is fully functional, just might be a bit more difficult to setup as the Linux one works right out of the box.

## Video Guide

[![first guide](https://img.youtube.com/vi/uFst1Hm4P9k/0.jpg)](https://www.youtube.com/watch?v=uFst1Hm4P9k)
[![windows guide](https://img.youtube.com/vi/0pucPQXm3IE/0.jpg)](https://www.youtube.com/watch?v=0pucPQXm3IE)

## Customising

You can customise what you want the macros to do by editing the `src/programmable_key.rs` file. At the moment there is built in support for OBS and sending JSON post requests. You are welcome to submit a pull request to add support for other programs.

## Build

You will need to install cargo (rusts package manager) and rustc (rust compiler) to build this project. You can find instructions on how to do that [here](https://www.rust-lang.org/tools/install).

Once you have cargo and rustc installed, you can build the project by running `cargo build --release` in the root directory of the project.

To reduce the binary size further you can use upx.
`upx --best --lzma target/release/qmk_programmable_button`

## Running on Windows

Windows is supported but still needs some work. The program will auto compile on windows but it might not detect the keypresses until you hardcode the correct device ID.

### Configuring it for windows

1. Download [hidapitester](https://github.com/todbot/hidapitester) and run it with `.\hidapitester.exe --list-detail`. You should your keyboard listed more than once.

```cmd
FEED/6060: ArtiomSu - macropad_artiomsu
  vendorId:      0xFEED
  productId:     0x6060
  usagePage:     0x0001
  usage:         0x0006
  serial_number: (null)
  interface:     0
  path: long random string

FEED/6060: ArtiomSu - macropad_artiomsu
  vendorId:      0xFEED
  productId:     0x6060
  usagePage:     0x000C
  usage:         0x0001
  serial_number: (null)
  interface:     1
  path: long random string
```

2. Pay attention to the usagePage and usage. The usagePage with 0x001 and usage 0x006 can be ignore as this is simply the keyboard. The usagePage with 0x00C and usage 0x001 is the programmable button. Take a note of that if it is different to what I have as you will need to change it.

3. If its different then go into `src/window_listner.rs` to go the `attach` function at the end of the page and update the `usagePage` and `usage` variables to match your keyboard.

4. I only mapped the macro keys I'm using for windows so you will need to add the rest in `programmable_key.rs` if you want to use them. I will add them in the future once I figure out a better way to handle windows.
