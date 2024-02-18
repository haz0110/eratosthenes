# Eratosthenes

Eratosthenes is a mathematics library written in Rust. It provides various mathematical operations, series and sequences commonly used in solving mathematical problems, particularly those found on [Project Euler](https://projecteuler.net/). This library is designed to be simple, efficient, and easy to use.

Please note that this library is in its early stages and missing important features. It is evolving with time.

## Features

- TO DO

### Functions

- eratosthenes::is_valid_triangle()
- eratosthenes::reduce_decimals()
- eratosthenes::primes()
- eratosthenes::nth_prime()

## Usage

To add this to your Rust project as a Git dependency, include in dependencies:
```
eratosthenes = { git = "https://github.com/haz0110/eratosthenes", branch = "master"  }
```

To add this to your Rust project as a local package, include in dependencies:
```
eratosthenes = { path = "../path/to/eratosthenes" }
```

# 0.3.0 Breaking Update

After months of discontinuity for the project, the main structure of the project is reorganized and many things are changed.

The code is organized in a way that a project that uses eratosthenes 0.1.1 or 0.2.0 version would not work with version 0.3.0 and above.