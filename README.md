[![CI](https://github.com/aegoroff/aes256gen/actions/workflows/ci.yml/badge.svg)](https://github.com/aegoroff/solt/actions/workflows/ci.yml)
[![](https://tokei.rs/b1/github/aegoroff/aes256gen?category=code)](https://github.com/XAMPPRocky/tokei)

# aes256gen
AES256 encrypt codes generator for Anytone DMR radio

## Install the pre-compiled binary

**manually**:

Download the pre-compiled binaries from the [releases](https://github.com/aegoroff/aes256gen/releases) and
copy to the desired location.

## Command line syntax:
```
AES256 encrypt codes generator for Anytone DMR radio

Usage: aes256gen [OPTIONS]

Options:
  -l, --limit <NUMBER>  The maximum number of codes to generate. [default: 10]
  -c, --csv <PATH>      Path to comma separated file to store results into
  -h, --help            Print help
  -V, --version         Print version
```
**EXAMPLES**

Generate 3 aes codes into file named `aes.csv` that will be placed in the current directory.
```shell
aes256gen -c aes.csv -l 3
```
Follow content of `aes.csv` will be generated:
```
"id","num","aeskey"
1,,D5D032576AEBC9A0FBE9F7FFE92D7DA919BD36F4612F4233EC8684BDF337B1DC
2,,7FBF04C5769172DD423FB035868474A6DCBE7BC9BE1D6BFC60826A80E97A278D
3,,33B39A7FB624AE71A8669CAD0A12775627805666437D9F7BE3DD838233636BAA
```