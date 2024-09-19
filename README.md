# poit-rs

Pip offline installation tool.

[![Rust](https://github.com/rikonaka/poit-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/rikonaka/poit-rs/actions/workflows/rust.yml)

## Requirements

- [x] Two servers, one can be networked called A, one **can not** be networked called B.
- [x] A server with the same `architecture` as the B server, and with a good network.
- [x] The `pip` is installed.
- [x] Both servers must have sufficient hard disk space.

## Usage

### In A server, pack all the dependencies of a pip package

We use `ipython` as example.

Create a work folder.

```bash
root@debian:~# mkdir test
root@debian:~# cp poit test/
root@debian:~# cd test
```

Start packing.

```bash
root@debian:~/test# ./poit --pack ipython
```

Or package a version of the software.

```bash
root@debian:~/test# ./poit --pack ipython --package-version 8.26.0
```

Or package with specific python version.

```bash
root@debian:~/test# ./poit --pack ipython --python-version 3.12
```

These three files will appear in the directory.

```bash
root@debian:~/test# ls
ipython.poit  ipython.poit.sha256  poit
```

Do not change any files, including `naming` and `content`, and make sure all three files are copied to a `USB` or `CD`.

### In B server, offline installation of ipython

Check for the presence of these three files.

```bash
root@debian:~/test# ls
ipython.poit  ipython.poit.sha256  poit
```

Start offline installation.

```bash
root@debian:~/test# ./poit --install ipython.poit
```

Or

```bash
root@debian:~/test# ./poit --install ipython.poit --package-version 8.26.0
```

If this process does not have any error messages, the installation is successful and you can now use the offline installed `ipython`.
