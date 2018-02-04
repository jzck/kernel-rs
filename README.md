based on [Writing an OS in rust](https://os.phil-opp.com/)

# compiling

## dependencies

  - `nasm` compiles the bootcode
  - `ld` links the bootcode
  - `grub-mkrescue` builds the iso
  - `xargo` builds rust code
  - `qemu` runs the iso

on archlinux `pacman -S make grub2 xorriso mtools binutils gcc qemu`

## rust setup

```
pacman -S rustup
rustup component add rust-src
cargo install xargo
```

## docker
a standard development environment can be invoked:

```
docker run jzck/arch-kernel -it /usr/bin/zsh
```

clone the repo and `make iso`

# running

`make run` in your host operating system to launch qemu gtk window
