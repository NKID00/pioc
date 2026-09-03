#!/bin/bash

fd -e ASM --strip-cwd-prefix=always -x wine WASM53B.EXE
fd -e ASM -x cargo run -- as -o '{/.}.out' '{}'
fd -e ASM -x diff '{/.}.BIN' '{/.}.out'
