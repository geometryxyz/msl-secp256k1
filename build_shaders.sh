#!/bin/bash
#
cd "$(dirname "$0")"

# u32_add.metal
xcrun -sdk macosx metal -c ./metal/tests/u32_add.metal -o ./metal/tests/u32_add.ir  
xcrun -sdk macosx metallib ./metal/tests/u32_add.ir -o ./metal/tests/u32_add.metallib

# bigint_add_unsafe.metal
xcrun -sdk macosx metal -c ./metal/tests/bigint_add_unsafe.metal -o ./metal/tests/bigint_add_unsafe.ir  
xcrun -sdk macosx metallib ./metal/tests/bigint_add_unsafe.ir -o ./metal/tests/bigint_add_unsafe.metallib
rm ./metal/tests/bigint_add_unsafe.ir

# bigint_add_wide.metal
xcrun -sdk macosx metal -c ./metal/tests/bigint_add_wide.metal -o ./metal/tests/bigint_add_wide.ir  
xcrun -sdk macosx metallib ./metal/tests/bigint_add_wide.ir -o ./metal/tests/bigint_add_wide.metallib
rm ./metal/tests/bigint_add_wide.ir

# bigint_sub.metal
xcrun -sdk macosx metal -c ./metal/tests/bigint_sub.metal -o ./metal/tests/bigint_sub.ir  
xcrun -sdk macosx metallib ./metal/tests/bigint_sub.ir -o ./metal/tests/bigint_sub.metallib
rm ./metal/tests/bigint_sub.ir

# ff_add.metal
xcrun -sdk macosx metal -c ./metal/tests/ff_add.metal -o ./metal/tests/ff_add.ir  
xcrun -sdk macosx metallib ./metal/tests/ff_add.ir -o ./metal/tests/ff_add.metallib
rm ./metal/tests/ff_add.ir
