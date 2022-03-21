#!/bin/bash

set -eux

wasm-pack build --release -t web
