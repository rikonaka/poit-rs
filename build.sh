#!/bin/bash

cross build --release --target x86_64-unknown-linux-musl
cross build --release --target i586-unknown-linux-musl
cross build --release --target aarch64-unknown-linux-musl

rm poit-x86_64-unknown-linux-musl.7z
rm poit-aarch64-unknown-linux-musl.7z
rm poit-i586-unknown-linux-musl.7z

7z a -mx9 poit-x86_64-unknown-linux-musl.7z ./target/x86_64-unknown-linux-musl/release/poit
7z a -mx9 poit-aarch64-unknown-linux-musl.7z ./target/aarch64-unknown-linux-musl/release/poit
7z a -mx9 poit-i586-unknown-linux-musl.7z ./target/i586-unknown-linux-musl/release/poit