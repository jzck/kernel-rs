based on [Writing an OS in rust](https://os.phil-opp.com/)

# compiling

## dependencies

`pacman -S grub2 xorriso mtools binutils`

## docker
a standard development environment can be invoked:

```
docker run jzck/arch-kernel -it /usr/bin/zsh
```

clone the repo and `make iso`

# running

`make run` in your host operating system to launch qemu gtk window
