# Playing with libfd and libopcodes

This directory contains examples that show howto call `binutils` bfd and
opcodes libraries from C:
1. [test_binary](examples/src/test_binary.c): this is the example from
   https://www.toothycat.net/wiki/wiki.pl?Binutils/libopcodes that was adapt to
   work with binutils 2.29.1
2. [test_buffer_x86_64](examples/src/test_buffer_x86_64.c): this example shows
   howto disassemble a buffer containing x86 instructions
3. [test_buffer_mep](examples/src/test_buffer_mep.c): similar to the second
   example with a more exotic architecture and binutils builtins

For convenience, the [libbfd](docs/ToothyWiki_\ Binutils_Bfd.html) and
[libopcodes][docs/ToothyWiki_\ Binutils_Libopcodes.html] documentation from
toothycat.net is archived in this repository.


## Building examples

The examples assume that binutils 2.29.1 is built at the root of this
repostitory. They can be compiled by typing `make` in the `src/` directory.

Here is how binutils dynamic libraries can be installed:
```
curl https://ftp.gnu.org/gnu/binutils/binutils-2.29.1.tar.gz -O
tar xzvf binutils-2.29.1.tar.gz
cd binutils-2.29.1/
./configure --prefix=../built --enable-shared --enable-targets=all
make -j8
```

Note: by default binutils does not produce shared libraries. When enabling all
targets, the resulting `objdump` binary is 61MB!
