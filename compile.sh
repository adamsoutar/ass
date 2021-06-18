# Run ass
RUST_BACKTRACE=1 cargo run input.c > output.s && \

# Assemble x86_64 assembly into a binary
# (ass is _not_ an assembler or linker)
gcc output.s -o output && \

# Run output
./output

# Print exit code
# (not as relevant now that we can call putchar())
# echo $?
