# eratosthenes

v0.2.0 is in alpha stage. Please report any issues you encounter via <mailto:hazar@a0110.com>

eratosthenes is a mathematics library written in pure Rust. It provides various mathematical operations and sequences commonly used in solving mathematical problems, particularly those found on [Project Euler](https://projecteuler.net/). This library is designed to be simple, efficient, and easy to use.

Please note that this library is in its early stages and missing important features. It is evolving with time.

## Features

- `array_clean`: Cleans an input array by removing duplicate values and sorting the elements.
- `array_merge`: Merges two arrays into a single array without modifying the original arrays.
- `factors`: Calculates the factors of a given number.
- `mean_usize`: Calculates the mean value of a vector of unsigned integers.
- `mean_f64`: Calculates the mean value of a vector of floating-point numbers.
- `factors_prime`: Calculates the prime factors of a given number.
- `is_palindrome`: Checks if a given number is a palindrome.
- `sum_even`: Calculates the sum of even numbers in an array.
- `sum_odd`: Calculates the sum of odd numbers in an array.
- `arithmetic`: Generates an arithmetic sequence of numbers.
- `fibonacci`: Generates a Fibonacci sequence up to a specified limit.
- `nth_fibonacci`: Returns the nth Fibonacci number.
- `primes`: Generates prime numbers up to a specified limit.
- `nth_prime`: Finds the nth prime number.
- `is_prime`: Checks if a number is prime.
- `square_numbers`: Generates a vector of square numbers.
- `triangular_numbers`: Generates a vector of triangular numbers.
- `nth_triangular`: Returns the nth triangular number.

## Usage

To add this to your Rust project as a Git dependency, include in dependencies:
```
eratosthenes = { git = "https://github.com/haz0110/eratosthenes", branch = "master"  }
```
