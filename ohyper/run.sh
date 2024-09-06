#!/bin/zsh
export LOG=info
cargo bootimage
qemu-system-x86_64 -cpu host,+x2apic -accel kvm -drive file=target/ohyper/debug/bootimage-ohyper.bin,format=raw -serial mon:stdio -nographic