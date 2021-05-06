#!/bin/bash

cargo build --target arm-unknown-linux-gnueabi
rsync -avhP ./target/arm-unknown-linux-gnueabi/debug/rpizw root@192.168.86.70:/root/
