#!/usr/bin/env bash

function build-release {
  maturin build --release -o wheels -i $(which python)
}

"$@"
