# Playing with libopcodes

## Ressources

From https://www.toothycat.net/wiki/wiki.pl?Binutils/libopcodes
Conviniently archived in data/


## Building the example

```
curl https://ftp.gnu.org/gnu/binutils/binutils-2.29.1.tar.gz -O
tar xzvf binutils-2.29.1.tar.gz
cd binutils-2.29.1/
./configure
make -j8

cd ..
make test\_binary
./test\_binary
```

## Compile all possible targets

```
./configure --enable-targets=all --prefix=$HOME/built
make -j8
make install

./built/bin/objdump -mmep -bbinary -D ../mep.bin
```

The resulting binaries are 61MB each !

## Shared libraries

```
./configure --prefix=$PWD/built --enable-shared --enable-targets=mep
make -j8
make install
export LD_LIBRARY_PATH=$PWD/built/lib
```

libopcodes.so is ~ 3Mo
libbfd.so is ~ 6Mo

The resulting r2 plugin is ~ 200Ko
