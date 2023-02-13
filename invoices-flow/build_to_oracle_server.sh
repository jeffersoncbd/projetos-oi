#!/bin/bash

cargo zigbuild --target x86_64-unknown-linux-gnu.2.12 --release

cp target/x86_64-unknown-linux-gnu/release/invoices-flow ./