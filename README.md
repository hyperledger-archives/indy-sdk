# sovrin-client-rust

# Windows build

- Get binary dependencies (libamcl*, openssl, libsodium, libzmq, sqlite3).
- Put all *.{lib,dll} into one directory and headers into include/ subdirectory.
- open MSVS development console
- execute "C:\Program Files (x86)\Microsoft Visual Studio\2017\Community\VC\Auxiliary\Build\vcvars64.bat"
- Point path to this directory using environment variables:
  - set SOVRIN_PREBUILT_DEPS_DIR=C:\BIN\x64
  - set SODIUM_LIB_DIR=C:\BIN\x64
  - set OPENSSL_INCLUDE_DIR=C:\BIN\x64\include
  - set OPENSSL_LIB_DIR=C:\BIN\x64
  - set LIBZMQ_LIB_DIR=C:\BIN\x64
  - set LIBZMQ_INCLUDE_DIR=C:\BIN\x64\include
- set static flag for libsodium build
  - set SODIUM_STATIC=y
- change dir to sovrin-client and run cargo (you may want to add --release --target x86_64-pc-windows-msvc keys to cargo)
