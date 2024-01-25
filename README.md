# Introduction

A rust kernel for learning OS, based on RISC-V.

This kernel runs on qemu and refers to the implementation of [rCore](https://wyfcyx.gitee.io/rcore-tutorial-book-v3/).

# Install Dependencies

## Rust

```
curl https://sh.rustup.rs -sSf | sh
cargo install cargo-binutils
rustup component add llvm-tools-preview
rustup target add riscv64gc-unknown-none-elf
```

## Clang/LLVM

```
sudo apt update
sudo apt install clang llvm lld -y
```

## Qemu

```
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
    gawk build-essential bison flex texinfo gperf libtool patchutils bc \
    zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev libsdl2-dev \
    git tmux python3 python3-pip ninja-build

wget https://download.qemu.org/qemu-7.0.0.tar.xz
tar -xf qemu-7.0.0.tar.xz && cd qemu-7.0.0

./configure --target-list=riscv64-softmmu,riscv64-linux-user
make -j$(nproc) && sudo make install
```

## GDB

```
wget https://ftp.gnu.org/gnu/gdb/gdb-14.1.tar.xz
tar -xf gdb-14.1.tar.xz && cd gdb-14.1

./configure --target=riscv64-unknown-elf
make -j$(nproc) && sudo make install
```

# Getting Started

Init submodule: `git submodule update --init`;

Build and run the kernel:

- Build the opensbi only: `make bios`;
- Build the kernel only: `make kernel`;
- Build the opensbi and kernel, run the kernel on qemu: `make qemu`;

Debug the kernel with gdb:

- Set up a gdb server with qemu: `make gdb-server`;
- Create a gdb client in another terminal: `make gdb-client`;

Clean up the compilation products: `make clean`.
