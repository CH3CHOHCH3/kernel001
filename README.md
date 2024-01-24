# Introduction

A rust kernel for learning OS, based on RISC-V.

This kernel runs on qemu and refers to the implementation of [rCore](https://wyfcyx.gitee.io/rcore-tutorial-book-v3/).

# Install Dependencies

## Rust

```
curl https://sh.rustup.rs -sSf | sh             # install rust
rustup component add llvm-tools-preview         # install rust-readobj,rust-objdump etc.
rustup target add riscv64gc-unknown-none-elf    # install core crates for riscv64
```

## Clang/LLVM

```
sudo apt update
sudo apt install clang llvm -y
```

## Qemu

```
# install dependencies
sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev libsdl2-dev \
              git tmux python3 python3-pip ninja-build

# download and unzip the source code
wget https://download.qemu.org/qemu-7.0.0.tar.xz
tar -xf qemu-7.0.0.tar.xz

# configure and compile
cd qemu-7.0.0
./configure --target-list=riscv64-softmmu,riscv64-linux-user
make -j$(nproc)
```

## GDB

```
sudo apt update
sudo apt install build-essential gdb-multiarch gcc-riscv64-linux-gnu binutils-riscv64-linux-gnu -y
```

# Getting Started

Build and run the kernel:

- Just build the opensbi: `make bios`;
- Just build the kernel: `make kernel`;
- Build the opensbi and kernel, run the kernel on qemu: `make qemu`;

Debug the kernel with gdb:
- Build a gdb server with qemu: `make gdb-server`;
- Build a gdb client in another terminal: `make gdb-client`;

Clean the workspace: `make clean`.
