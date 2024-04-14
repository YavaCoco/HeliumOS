FROM rust:1.76.0-bookworm

ARG CPU MEMORY

RUN apt-get update &&\
    apt-get -y upgrade &&\
    apt-get -y install --no-install-recommends qemu-system ovmf gdb tmux socat

COPY --chmod=0755 ./scripts/debug.sh /vm/debug.sh
ENTRYPOINT [ "/vm/debug.sh" ]
