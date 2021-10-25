# Readme

## Overview

- `crates/` has all the binaries - one crate per binary. Binaries may depend on libs, but not on each other.
- `libs/` has all the libraries - one crate per library. Libraries can depend on each other, but be careful not to make circles.
- `tools/` has all the development tools (cargo make, cargo checks) and certain tests. One .src file per binary.