<h1 align=center>Sizeme</h1>
        <h3 align=center>Shows the user how large an executable would be if they were to strip it, use upx to compress it, or both.</h3>

## Features

* Multithreaded
* Multiple Compression Levels

## Goals

- Search for executables in current directory and subdirectories

## Usage

```
sizeme [FALGS] [<path>]
```
## Installation

```
cargo install --git https://github.com/FriedPandaFries/sizeme.git
```

### Prerequisites

* strip (part of GNU Binutils)
* UPX (optional but recommended)

#### Fedora / RHEL / etc.

```
sudo dnf install upx
```

#### Ubuntu / Mint / Elementary / etc.

```
sudo apt-get update -y
sudo apt-get install -y upx-ucl
```

#### Arch Linux

```
sudo pacman -S upx
```

#### Alpine Linux

```
sudo apk add upx
```

### Building

```
git clone https://github.com/FriedPandaFries/sizeme.com
cd sizeme
cargo install --path $PWD
```

## Update

```
cargo install --force --git https://github.com/FriedPandaFries/sizeme.git
```

## Example Output
```
$ sizeme servo

Method   File Size  Ratio
------   ---------  ------
Initial  282 MB     100.0%
Both     34 MB      12.33%
Strip    92 MB      32.76%
UPX      116 MB     41.18%
```
