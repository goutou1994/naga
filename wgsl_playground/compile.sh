#!/bin/bash

filename=$1

../target/debug/naga $filename.wgsl $filename.spv

/Users/zhouxinyi/ThirdCode/SPIRV-Tools/build/tools/Debug/spirv-dis $filename.spv -o $filename.spvasm
