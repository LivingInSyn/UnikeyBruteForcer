# Unikey Brute Forcer
Due to the way that the unikey dongle (https://www.esecutech.com/products/unikey/unikey-std) provisions usernames and passwords from a single 32 bit seed, it's possible to brute force the admin password relatively quicky (under 5 minutes on i7-7820HQ) brute force the admin password.

You will need to have already extracted the user passwords from the target program. Hint: look for calls to `UniKey_User_Logon`

## To build
You need rust installed: https://www.rust-lang.org/tools/install

Once it's installed run `cargo build --release`

It will be in `.\target\release\brute_unikey[.exe]`

## To Run
`brute_unikey.exe [-v] <upass1> <upass2> [threads]`

## What next?
Try and logon to the unikey dongle using the suggested admin passwords along with the user passwords you've already got. You can use the handlily provided unikey console provided by secutech in their SDK