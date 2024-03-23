# seL4-rs

## seL4 Build Dependencies

### fedora-toolbox

- clang (optional)
- arm-linux-gnueabi-{gcc, binutils}
- arm-none-eabi-gdb (optional)
- cmake
- ninja-build
- qemu-system-arm (optional)
- dtc
- cpio
- clang-libs
- python3-ply
- python3-future
- python3-pip
- python3-setuptools
- python3-jsonschema
- python3-pyyaml
- python3-lxml
- python3-jinja2
- python3-pyelftools
- python3-libarchive-c
- pyfdt

```
dnf install cmake ninja-build dtc cpio clang-libs python3-ply python3-future python3-pip python3-setuptools python3-jsonschema python3-pyyaml python3-lxml python3-jinja2 python3-pyelftools python3-libarchive-c
```

```
pip install pyfdt
```

## Compile

### GCC

```
mkdir build && cd build

cmake -GNinja -DCROSS_COMPILER_PREFIX=arm-linux-gnueabi- -DCMAKE_TOOLCHAIN_FILE=../kernel/gcc.cmake -DPLATFORM=zynq7000 -DRELEASE=FALSE -DVERIFICATION=FALSE -DKernelIsMCS=ON -DLibSel4FunctionAttributes=public -DKernelDangerousCodeInjection=ON -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..

ninja
```

### Clang

```
mkdir build && cd build

cmake -GNinja -DTRIPLE=arm-linux-gnueabi -DCMAKE_TOOLCHAIN_FILE=../kernel/llvm.cmake -DPLATFORM=zynq7000 -DRELEASE=FALSE -DVERIFICATION=FALSE -DKernelIsMCS=ON -DLibSel4FunctionAttributes=public -DKernelDangerousCodeInjection=ON -DCMAKE_EXPORT_COMPILE_COMMANDS=1 ..

ninja
```
