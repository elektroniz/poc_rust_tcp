#!/bin/zsh
cargo b --release

sudo setcap cap_net_admin=eip /home/kali/Desktop/Coding/poc_rust_tcp/target/release/poc_rust_tcp

./target/release/poc_rust_tcp &

pid=$!

sudo ip addr add 10.0.2.16/24 dev tun0
sudo ip link set up dev tun0

trap "kill $pid" INT TERM
wait $pid
