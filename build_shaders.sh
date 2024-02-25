#!/bin/bash
#
cd "$(dirname "$0")"

# u32_add.metal
xcrun -sdk macosx metal -c ./metal/u32_add.metal -o ./metal/u32_add.ir  
xcrun -sdk macosx metallib ./metal/u32_add.ir -o ./metal/u32_add.metallib

# bigint_add.metal
xcrun -sdk macosx metal -c ./metal/bigint_add.metal -o ./metal/bigint_add.ir  
xcrun -sdk macosx metallib ./metal/bigint_add.ir -o ./metal/bigint_add.metallib
