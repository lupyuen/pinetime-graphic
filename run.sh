#!/usr/bin/env bash

set -e  #  Exit when any command fails
set -x  #  Echo commands

export EINK_PATH=$PWD/../../pinecone/bl_iot_sdk/customer_app/sdk_app_uart_eink/sdk_app_uart_eink 

cargo run -- --min 0  --max 85  uart-cartoon2.png >image_black.inc
cargo run -- --min 86 --max 215 uart-cartoon2.png >image_red.inc

cp image_black.inc $EINK_PATH
cp image_red.inc $EINK_PATH
