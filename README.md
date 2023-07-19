# cryptshim
A little Rusty shim to decrypt the boot/EFI partition containing the kernel or other bootloaders before loading them.

This is designed to decrypt the traditional /boot partition, read from a Protobuf config file, then load whatever bootloader is specified in the config file. The goal for now is to get this working on Linux EFISTUB (vmlinux, microcode, initramfs, etc), but I'd like to get this working with other bootloaders like GRUB, rEFInd, etc.

Only LUKS1/LUKS2 is supported for now. Might do BitLocker in the future.

This uses the https://github.com/rust-osdev/uefi-rs crates.

This hobby project is under casual development, and is not ready for use.

## Goals:
### 0.1
- [ ] Actually run (duh)
- [ ] Prompt for password (unused for now)
- [ ] Read db, dbx, and MOK from EFI variables
- [ ] Load any EFI binary, preferably Linux EFISTUB
### 0.2
- [ ] Implement a very simple compile-time config
### 0.3
- [ ] Implement a Protobuf config file inside the EFI partition
### 0.5
- [ ] Decrypt a LUKS1/LUKS2 partition using the provided password
- [ ] Actually use the db, MOK, and dbx to whitelist and blacklist binaries
### 1.0
- [ ] Allow provisioning of blacklisted hashes into the compiled EFI binary
- [ ] Implement TPM2 unlocking for LUKS1/LUKS2
- [ ] Comply to Microsoft's Secure Boot signing requirements
### Later
- [ ] Implement BitLocker unlocking
- [ ] Support WIndows bootloaders
### Maybe
- [ ] Add entropy for the Linux kernel

## Out of scope (or else my ADHD will get me):
- Modify any EFI variables
- Implement a GUI
- Allow selection of multiple EFI bootloaders/kernel images, thus implementing a boot manager
- Run Crysis
