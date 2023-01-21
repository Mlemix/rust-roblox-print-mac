# rust-roblox-print-mac
call ROBLOX's print function using the power of rust on MacOS!!!

how to build:
```
cargo build --release --target x86_64-apple-darwin
```
the dylib should be in /target/x86_64-apple-darwin/release/

how to use:
```
1. remove roblox's signature
codesign --remove-signature /path/to/RobloxPlayer
2. inject (if you chose to do it this way, add a sleep so you have enough time to get into a game)
sudo DYLD_INSERT_LIBRARIES="/path/to/librust_roblox_print_mac.dylib" /path/to/RobloxPlayer
```

this was written and tested on a 2022 macbook air running MacOS Ventura 13.0.1

c++ version: https://github.com/Mlemix/roblox-print-mac<br/>
windows version: https://github.com/Mlemix/rust-roblox-print<br/>
windows c++ version: https://github.com/Mlemix/roblox-print
