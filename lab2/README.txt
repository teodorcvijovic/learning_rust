- binary code instrumentation with Valgrind
    # valgrind --tool=memcheck ./uppercase 'Hello, World!'
    - with this we can detect memory leaks

- LLVM sanitazer
!   - instrumentation is done at compile time,
      but verification is done in RUNTIME
    - it can detect stack overflow
    # clang -g -O0 -Wall -Wextra -o uppercase uppercase.c -fsanitize=address,leak,undefined

- clang-tidy
    # clang-tidy linkedlist.c --checks=* -- -fblocks