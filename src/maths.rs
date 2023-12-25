use std::collections::HashMap;

#[allow(dead_code)]
/// Returns all the divisors of `n` including `1` and `n`.
///
/// If `n == 0`, an empty Vector is returned.
///
/// ### Arguments
///
/// * `n` : `u128` - The number for which we need to find all the divisors.
///
/// ### Returns
///
/// * `Vec<u128>` - List of all divisors of `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::all_divisors as f;
///
/// assert_eq!(f(1), vec![1]);
/// assert_eq!(f(2), vec![1, 2]);
/// assert_eq!(f(23), vec![1, 23]);
/// assert_eq!(f(100), vec![1, 2, 4, 5, 10, 20, 25, 50, 100]);
/// assert_eq!(f(5040), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 16, 18,
///     20, 21, 24, 28, 30, 35, 36, 40, 42, 45, 48, 56, 60, 63, 70, 72, 80, 84,
///     90, 105, 112, 120, 126, 140, 144, 168, 180, 210, 240, 252, 280, 315,
///     336, 360, 420, 504, 560, 630, 720, 840, 1008, 1260, 1680, 2520, 5040]
/// );
///
/// ```
pub fn all_divisors(n: u128) -> Vec<u128> {
    if n == 0 {
        return vec![];
    }

    if n == 1 {
        return vec![1];
    }

    if n < 4 {
        return vec![1, n];
    }

    let sqrt: u128 = int_sqrt(n);

    let mut divisors: Vec<u128> = Vec::new();

    for i in 1..=sqrt {
        if n % i == 0 {
            divisors.push(i);
            divisors.push(n / i);
        }

        if i == sqrt && i * i == n {
            divisors.pop();
        }
    }

    divisors.sort();

    return divisors;
}

#[allow(dead_code)]
/// Returns the first `n` prime numbers.
///
/// _It does not uses_ [`sieve_of_eratosthenes`].
///
/// [`sieve_of_eratosthenes`]: fn.sieve_of_eratosthenes.html
///
/// ### Arguments
///
/// * `n` : `u128` - The number of primes that you want.
///
/// ### Returns
///
/// * `Vec<u128>` - List of first `n` prime numbers.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::first_n_primes as f;
///
/// assert_eq!(f(0), vec![]);
/// assert_eq!(f(1), vec![2]);
/// assert_eq!(f(2), vec![2, 3]);
/// assert_eq!(f(5), vec![2, 3, 5, 7, 11]);
/// ```
pub fn first_n_primes(n: u128) -> Vec<u128> {
    let mut ans: Vec<u128> = Vec::new();

    if n == 0 {
        return ans;
    }

    ans.push(2);

    let mut curr_num: u128 = 3;

    while (ans.len() as u128) < n {
        if !ans.iter().any(|prime: &u128| curr_num % prime == 0) {
            ans.push(curr_num);
        }

        curr_num += 2;
    }

    return ans;
}

#[allow(dead_code)]
/// Returns the floored square root of `n` using binary search.
///
/// ### Alternate Functions
///
/// ```
/// fn int_sqrt(n: u128) -> u128 {
///     return (n as f64).sqrt() as u128;
/// }
///
/// ```
///
/// Faster but gives wrong results when `n` is large.
///
/// ```
/// fn int_sqrt(n: u128) -> u128 {
///     return (n as f64).sqrt() as u128;
/// }
///
/// let n: u128 = std::u64::MAX as u128; // 18446744073709551615
///
/// let square: u128 = n * n; // 340282366920938463426481119284349108225
///
/// assert_ne!(int_sqrt(square), n);  // because int_sqrt(square) returns 18446744073709551616
///
/// ```
///
/// ### Arguments
///
/// * `n` : `u128` - The number whose square root we need.
///
/// ### Returns
///
/// * `u128` - Square root of `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::int_sqrt as f;
///
/// assert_eq!(f(0), 0);
/// assert_eq!(f(1), 1);
/// assert_eq!(f(8), 2);
/// assert_eq!(f(9), 3);
///
/// ```
pub fn int_sqrt(n: u128) -> u128 {
    if n < 2 {
        return n;
    }

    let mut low: u128 = 1;
    let mut high: u128 = std::u64::MAX as u128;

    while low <= high {
        let mid: u128 = low + (high - low) / 2;
        let mid_squared: u128 = mid * mid;

        if mid_squared == n {
            return mid;
        } else if mid_squared < n {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return high;
}

#[allow(dead_code)]
/// Returns whether a number is palindrome or not
///
/// ### Arguments
///
/// * `n` : `u128` - The number that we need to check.
///
/// ### Returns
///
/// * `bool` - `true` or `false` based on whether palindrome or not.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::is_palindrome as f;
///
/// assert_eq!(f(1), true);
/// assert_eq!(f(2000), false);
/// assert_eq!(f(10000001), true);
/// assert_eq!(f(12345678987654321), true);
///
/// ```
pub fn is_palindrome(n: u128) -> bool {
    let mut reversed: u128 = 0;
    let mut number: u128 = n;

    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number = number / 10;
    }

    return reversed == n;
}

#[allow(dead_code)]
/// Returns whether `n` is a perfect square or not
///
/// if `n == 0` , `true` is returned.
///
/// ### Arguments
///
/// * `n` : `u128` - The number that needs to be checked.
///
/// ### Returns
///
/// * `bool` - Whether `n` is a perfect square or not.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::is_perfect_square as f;
///
/// assert_eq!(f(0), true);
/// assert_eq!(f(1), true);
/// assert_eq!(f(2), false);
/// assert_eq!(f(4), true);
/// assert_eq!(f(100), true);
/// assert_eq!(f(999), false);
///
/// ```
pub fn is_perfect_square(n: u128) -> bool {
    if int_sqrt(n) * int_sqrt(n) == n {
        return true;
    }

    return false;
}

#[allow(dead_code)]
/// Returns whether a number is prime number or not
///
/// ### Arguments
///
/// * `n` : `u128` - The number that we need to check for primality.
///
/// ### Returns
///
/// * `bool` - `true` or `false` based on primality test.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::is_prime as f;
///
/// assert_eq!(f(1), false);
/// assert_eq!(f(2), true);
/// assert_eq!(f(5), true);
/// assert_eq!(f(8), false);
///
/// ```
pub fn is_prime(n: u128) -> bool {
    if n < 2 {
        return false;
    }

    if n % 2 == 0 {
        return n == 2;
    }

    for i in (3..=int_sqrt(n)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

#[allow(dead_code)]
/// Returns the maximum product of `n_adjacent` adjacent numbers in a grid of any size
///
/// The function checks for maximum product horizontally, vertically, and along the directions of both diagonals.
///
/// ### Arguments
///
/// * `grid` : `Vec<Vec<u128>>` - A grid in the form of a Vector of Vectors.
/// * `n_adjacent` : `u128` - The number of adjacent numbers used to find the product
///
/// ### Returns
///
/// * `u128` - The maximum product of `n_adjacent` numbers in all 4 orientations.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::max_prod_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
///     vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
///     vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
///     vec![52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
///     vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
///     vec![24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
///     vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
///     vec![67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
///     vec![24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
///     vec![21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
///     vec![78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
///     vec![16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
///     vec![86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
///     vec![19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
///     vec![04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
///     vec![88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
///     vec![04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
///     vec![20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
///     vec![20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
///     vec![01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48],
/// ];
///
/// assert_eq!(f(grid, 4), 70600674); // project euler question 11 : https://projecteuler.net/problem=11
///
/// ```
///
/// ```
/// use project_euler::maths::max_prod_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![1, 1, 1],
///     vec![1, 1, 3],
///     vec![1, 1, 3],
/// ];
///
/// assert_eq!(f(grid, 2), 9);
///
/// ```
///
/// ```
/// use project_euler::maths::max_prod_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![2, 2, 2, 2, 9, 8],
///     vec![2, 2, 2, 2, 2, 2],
/// ];
///
/// assert_eq!(f(grid, 2), 72);
///
/// ```
///
/// ```
/// use project_euler::maths::max_prod_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![1, 2, 3, 4],
///     vec![1, 2, 3, 4],
///     vec![1, 2, 3, 4],
///     vec![1, 2, 3, 4],
/// ];
///
/// assert_eq!(f(grid, 4), 256);
///
/// ```
pub fn max_prod_in_grid(grid: Vec<Vec<u128>>, n_adjacent: u128) -> u128 {
    let n_rows: usize = grid.len();
    let n_cols: usize = grid[0].len();

    let mut max_prod: u128 = 0;

    // horizontal
    for i in 0..(n_rows as usize) {
        for j in 0..(n_cols + 1 - n_adjacent as usize) {
            let mut longitudinal_prod: u128 = 1;
            for n in 0..n_adjacent as usize {
                longitudinal_prod *= grid[i][j + n];
            }
            max_prod = max_prod.max(longitudinal_prod);
        }
    }

    // vertical
    for i in 0..(n_rows + 1 - n_adjacent as usize) {
        for j in 0..(n_cols as usize) {
            let mut vertical_prod: u128 = 1;
            for n in 0..n_adjacent as usize {
                vertical_prod *= grid[i + n][j];
            }
            max_prod = max_prod.max(vertical_prod);
        }
    }

    // diagonal like this : /
    for i in 0..(n_rows + 1 - n_adjacent as usize) {
        for j in 0..(n_cols + 1 - n_adjacent as usize) {
            let mut diagonal_1: u128 = 1;
            for n in 0..n_adjacent as usize {
                diagonal_1 *= grid[i + n][j + n];
            }
            max_prod = max_prod.max(diagonal_1);
        }
    }

    // diagonal like this : \
    for i in 0..(n_rows + 1 - n_adjacent as usize) {
        for j in (n_adjacent as usize - 1)..(n_cols + 1 - n_adjacent as usize) {
            let mut diagonal_2: u128 = 1;
            for n in 0..n_adjacent as usize {
                diagonal_2 *= grid[i + n][j - n];
            }
            max_prod = max_prod.max(diagonal_2);
        }
    }
    return max_prod;
}

#[allow(dead_code)]
/// Returns the maximum sum of `n_adjacent` adjacent numbers in a grid of any size
///
/// The function checks for maximum sum horizontally, vertically, and along the directions of both diagonals.
///
/// ### Arguments
///
/// * `grid` : `Vec<Vec<u128>>` - A grid in the form of a list of lists.
/// * `n_adjacent` : `u128` - The number of adjacent numbers used to find the sum
///
/// ### Returns
///
/// * `u128` - The maximum sum of `n_adjacent` numbers in all 4 orientations.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::max_sum_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08],
///     vec![49, 49, 99, 40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00],
///     vec![81, 49, 31, 73, 55, 79, 14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65],
///     vec![52, 70, 95, 23, 04, 60, 11, 42, 69, 24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91],
///     vec![22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54, 22, 40, 40, 28, 66, 33, 13, 80],
///     vec![24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84, 20, 35, 17, 12, 50],
///     vec![32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38, 64, 70],
///     vec![67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21],
///     vec![24, 55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72],
///     vec![21, 36, 23, 09, 75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95],
///     vec![78, 17, 53, 28, 22, 75, 31, 67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92],
///     vec![16, 39, 05, 42, 96, 35, 31, 47, 55, 58, 88, 24, 00, 17, 54, 24, 36, 29, 85, 57],
///     vec![86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44, 60, 21, 58, 51, 54, 17, 58],
///     vec![19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77, 04, 89, 55, 40],
///     vec![04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98, 66],
///     vec![88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69],
///     vec![04, 42, 16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36],
///     vec![20, 69, 36, 41, 72, 30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16],
///     vec![20, 73, 35, 29, 78, 31, 90, 01, 74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54],
///     vec![01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33, 48, 61, 43, 52, 01, 89, 19, 67, 48],
/// ];
///
/// assert_eq!(f(grid, 4), 367); // project euler question 11 : https://projecteuler.net/problem=11
///
/// ```
///
/// ```
/// use project_euler::maths::max_sum_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![1, 1, 1],
///     vec![1, 1, 3],
///     vec![1, 1, 3],
/// ];
///
/// assert_eq!(f(grid, 2), 6);
///
/// ```
///
/// ```
/// use project_euler::maths::max_sum_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![2, 2, 2, 2, 9, 8],
///     vec![2, 2, 2, 2, 2, 2],
/// ];
///
/// assert_eq!(f(grid, 2), 17);
///
/// ```
///
/// ```
/// use project_euler::maths::max_sum_in_grid as f;
///
/// let grid: Vec<Vec<u128>> = vec![
///     vec![1, 2, 3, 4],
///     vec![1, 2, 3, 4],
///     vec![1, 2, 3, 4],
///     vec![1, 2, 3, 4],
/// ];
///
/// assert_eq!(f(grid, 4), 16);
///
/// ```
pub fn max_sum_in_grid(grid: Vec<Vec<u128>>, n_adjacent: u128) -> u128 {
    let n_rows: usize = grid.len();
    let n_cols: usize = grid[0].len();

    let mut max_sum: u128 = 0;

    // horizontal
    for i in 0..(n_rows as usize) {
        for j in 0..(n_cols + 1 - n_adjacent as usize) {
            let mut longitudinal_sum: u128 = 0;
            for n in 0..n_adjacent as usize {
                longitudinal_sum += grid[i][j + n];
            }
            max_sum = max_sum.max(longitudinal_sum);
        }
    }

    // vertical
    for i in 0..(n_rows + 1 - n_adjacent as usize) {
        for j in 0..(n_cols as usize) {
            let mut vertical_sum: u128 = 0;
            for n in 0..n_adjacent as usize {
                vertical_sum += grid[i + n][j];
            }
            max_sum = max_sum.max(vertical_sum);
        }
    }

    // diagonal like this : /
    for i in 0..(n_rows + 1 - n_adjacent as usize) {
        for j in 0..(n_cols + 1 - n_adjacent as usize) {
            let mut diagonal_1: u128 = 0;
            for n in 0..n_adjacent as usize {
                diagonal_1 += grid[i + n][j + n];
            }
            max_sum = max_sum.max(diagonal_1);
        }
    }

    // diagonal like this : \
    for i in 0..(n_rows + 1 - n_adjacent as usize) {
        for j in (n_adjacent as usize - 1)..(n_cols + 1 - n_adjacent as usize) {
            let mut diagonal_2: u128 = 0;
            for n in 0..n_adjacent as usize {
                diagonal_2 += grid[i + n][j - n];
            }
            max_sum = max_sum.max(diagonal_2);
        }
    }
    return max_sum;
}

#[allow(dead_code)]
/// Returns the `n`<sup>th</sup> prime number.
///
/// Uses [`first_n_primes`] to generate the first `n` primes and returns the last in the list.
///
/// [`first_n_primes`]: fn.first_n_primes.html
///
/// ### Arguments
///
/// * `n` : `u128` - The position of prime number to be returned with __1-based indexing__.
///
/// ### Returns
///
/// * `u128` - the `n`<sup>th</sup> prime number.
///
/// ### Panics
///
/// * When `n == 0`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::nth_prime as f;
///
/// assert_eq!(f(1), 2);
/// assert_eq!(f(2), 3);
/// assert_eq!(f(4), 7);
/// assert_eq!(f(888), 6907);
///
/// ```
pub fn nth_prime(n: u128) -> u128 {
    if n == 0 {
        panic!("n CANNOT be 0");
    }
    return *first_n_primes(n).last().unwrap();
}

#[allow(dead_code)]
/// Returns the total number of divisors of `n` including `1` ans `n`.
///
/// If `n == 0`, then `0` is returned.
///
/// ### Alternate Functions
///
/// ```
/// use std::collections::HashMap;
/// use project_euler::maths::prime_factors_without_sieve_as_hashmap;
///
/// fn num_divisors(n: u128) -> u128 {
///     if n == 0 {
///         return 0;
///     }
///
///     let primes_with_frequency: HashMap<u128, u128> = prime_factors_without_sieve_as_hashmap(n);
///
///     let mut num_of_divisors: u128 = 1;
///
///     for (_, value) in &primes_with_frequency {
///         num_of_divisors *= *value + 1;
///     }
///
///     return num_of_divisors;
/// }
///
/// ```
/// _It uses_ [`prime_factors_without_sieve_as_hashmap`].
///
/// [`prime_factors_without_sieve_as_hashmap`]: fn.prime_factors_without_sieve_as_hashmap.html
///
/// It is slower compared to the one actually used.
///
/// ### Arguments
///
/// * `n` : `u128` - The number for which we need to find the total number of divisors.
///
/// ### Returns
///
/// * `u128` - The total number of divisors of `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::num_divisors as f;
///
/// assert_eq!(f(1), 1);
/// assert_eq!(f(2), 2);
/// assert_eq!(f(23), 2);
/// assert_eq!(f(100), 9);
/// assert_eq!(f(5040), 60);
///
/// ```
pub fn num_divisors(n: u128) -> u128 {
    if n == 0 {
        return 0;
    }

    let mut count: u128 = 0;

    let sqrt: u128 = int_sqrt(n);

    for i in 1..=sqrt {
        if n % i == 0 {
            count += 2;
        }
    }

    if sqrt * sqrt == n {
        count -= 1;
    }

    return count;
}

#[allow(dead_code)]
/// Returns a Hashmap with keys as primes and values as the number of times the prime is present in the number
///
/// First we get a list of primes upto and including `n` with [`prime_factors_with_sieve_as_vec`].
///
/// [`prime_factors_with_sieve_as_vec`]: fn.prime_factors_with_sieve_as_vec.html
///
/// Then we check if each of the primes is a factor of `n` and if that is the case, then we find the number of times it is present in `n`.
///
/// ### Arguments
///
/// * `n` : `u128` - The number for which we are trying to find the prime factors and their frequencies.
///
/// ### Returns
///
/// * `HashMap<u128, u128>` - Hashmap with primes as keys and their frequency as values.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::prime_factors_with_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new();
/// expected.insert(2, 2);
/// expected.insert(5, 1);
/// expected.insert(3, 1);
///
/// assert_eq!(f(60), expected);
///
/// ```
///
/// ```
/// use project_euler::maths::prime_factors_with_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new(); // empty hashmap
/// assert_eq!(f(0), expected);
/// assert_eq!(f(1), expected);
///
/// ```
///
/// ```
/// use project_euler::maths::prime_factors_with_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new();
/// expected.insert(2, 1);
/// assert_eq!(f(2), expected);
///
/// ```
///
/// ```
/// use project_euler::maths::prime_factors_with_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new();
/// expected.insert(2, 1);
/// expected.insert(3, 1);
/// expected.insert(5, 1);
/// expected.insert(7, 1);
/// expected.insert(11, 1);
/// expected.insert(13, 1);
///
/// assert_eq!(f(30030), expected);
///
/// ```
pub fn prime_factors_with_sieve_as_hashmap(n: u128) -> HashMap<u128, u128> {
    let ans: Vec<u128> = sieve_of_eratosthenes(n);

    let mut n: u128 = n;

    let mut index: usize = 0;
    let mut primes_with_frequency: HashMap<u128, u128> = HashMap::new();

    while index < ans.len() {
        let curr_prime: u128 = ans[index];

        if n % curr_prime == 0 {
            primes_with_frequency.insert(
                curr_prime,
                primes_with_frequency.get(&curr_prime).unwrap_or(&0) + 1,
            );
            n /= curr_prime;
        } else {
            index += 1;
        }
    }

    return primes_with_frequency;
}

#[allow(dead_code)]
/// Returns a list of prime factors of the given number `n`.
///
/// First we find all the prime numbers less than or equal to `n` using [`sieve_of_eratosthenes`]
///
/// [`sieve_of_eratosthenes`]: fn.sieve_of_eratosthenes.html
///
/// Then we check whether each of those prime is a factor of `n`.
///
/// ### Arguments
///
/// * `n` : `u128` - The number for which we have to find prime factors.
///
/// ### Returns
///
/// * `Vec<u128>` - List of prime factors of `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::prime_factors_with_sieve_as_vec as f;
///
/// assert_eq!(f(1), vec![]);
/// assert_eq!(f(7), vec![7]);
/// assert_eq!(f(10), vec![2, 5]);
/// assert_eq!(f(60), vec![2, 3, 5]);
///
/// ```
pub fn prime_factors_with_sieve_as_vec(n: u128) -> Vec<u128> {
    let primes: Vec<u128> = sieve_of_eratosthenes(n);

    let mut index: usize = 0;
    let number: u128 = n;

    let mut ans: Vec<u128> = Vec::new();

    while number > 1 && index < primes.len() {
        let curr_prime: u128 = primes[index];

        if number % curr_prime == 0 {
            ans.push(curr_prime);
        }
        index += 1;
    }

    return ans;
}

#[allow(dead_code)]
/// Returns a Hashmap with keys as primes and values as their frequencies as present in the number
///
/// First we get a list of primes upto and including `n` with [`primes_upto_n_without_sieve`].
///
/// [`primes_upto_n_without_sieve`]: fn.primes_upto_n_without_sieve.html
///
/// Then we check if each of these primes is a factor of `n` and if that is the case, then we find the number of times it is present in `n`.
///
/// ### Arguments
///
/// * `n` : `u128` - The number for which we are trying to find the prime factos and its frequency.
///
/// ### Returns
///
/// * `HashMap<u128, u128>` - Hashmap with primes as keys and its frequency as values.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::prime_factors_without_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new();
/// expected.insert(2, 2);
/// expected.insert(5, 1);
/// expected.insert(3, 1);
///
/// assert_eq!(f(60), expected);
///
/// ```
///
/// ```
/// use project_euler::maths::prime_factors_without_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new(); // empty hashmap
/// assert_eq!(f(0), expected);
/// assert_eq!(f(1), expected);
///
/// ```
///
/// ```
/// use project_euler::maths::prime_factors_without_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new();
/// expected.insert(2, 1);
/// assert_eq!(f(2), expected);
///
/// ```
///
/// ```
/// use project_euler::maths::prime_factors_without_sieve_as_hashmap as f;
/// use std::collections::HashMap;
///
/// let mut expected: HashMap<u128, u128> = HashMap::new();
/// expected.insert(2, 1);
/// expected.insert(3, 1);
/// expected.insert(5, 1);
/// expected.insert(7, 1);
/// expected.insert(11, 1);
/// expected.insert(13, 1);
///
/// assert_eq!(f(30030), expected);
///
/// ```
pub fn prime_factors_without_sieve_as_hashmap(n: u128) -> HashMap<u128, u128> {
    let primes: Vec<u128> = primes_upto_n_without_sieve(n);

    let mut n: u128 = n;

    let mut index: usize = 0;
    let mut factors_with_frequency: HashMap<u128, u128> = HashMap::new();

    while index < primes.len() {
        let curr_prime: u128 = primes[index];

        if n % curr_prime == 0 {
            factors_with_frequency.insert(
                curr_prime,
                factors_with_frequency.get(&curr_prime).unwrap_or(&0) + 1,
            );
            n /= curr_prime;
        } else {
            index += 1;
        }
    }

    return factors_with_frequency;
}

#[allow(dead_code)]
/// Returns a list of prime factors of the given number `n` without using [`sieve_of_eratosthenes`]
///
/// [`sieve_of_eratosthenes`]: fn.sieve_of_eratosthenes.html
///
/// ### Arguments
///
/// * `n` : `u128` - The number for which we have to find prime factors.
///
/// ### Returns
///
/// * `Vec<u128>` - List of prime factors of `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::prime_factors_without_sieve_as_vec as f;
///
/// assert_eq!(f(1), vec![]);
/// assert_eq!(f(7), vec![7]);
/// assert_eq!(f(10), vec![2, 5]);
/// assert_eq!(f(60), vec![2, 3, 5]);
///
/// ```
pub fn prime_factors_without_sieve_as_vec(n: u128) -> Vec<u128> {
    let primes: Vec<u128> = primes_upto_n_without_sieve(n);

    let mut factors: Vec<u128> = Vec::new();

    for prime in primes {
        if n % prime == 0 {
            factors.push(prime);
        }
    }

    return factors;
}

#[allow(dead_code)]
/// Returns a list of primes upto and including `n` without using [`sieve_of_eratosthenes`] as it requires a lot of memory.
///
/// [`sieve_of_eratosthenes`]: fn.sieve_of_eratosthenes.html
///
/// ### Arguments
///
/// * `n` : `u128` - The inclusive number upto which we are trying to find primes.
///
/// ### Returns
///
/// * `Vec<u128>` - List of primes upto and including `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::primes_upto_n_without_sieve as f;
///
/// assert_eq!(f(0), vec![]);
/// assert_eq!(f(1), vec![]);
/// assert_eq!(f(2), vec![2]);
/// assert_eq!(f(12), vec![2, 3, 5, 7, 11]);
/// assert_eq!(f(17), vec![2, 3, 5, 7, 11, 13, 17]);
///
/// ```
pub fn primes_upto_n_without_sieve(n: u128) -> Vec<u128> {
    let mut ans: Vec<u128> = Vec::new();

    if n < 2 {
        return ans;
    }

    ans.push(2);

    for i in (3..=n).step_by(2) {
        if !ans.iter().any(|prime| i % prime == 0) {
            ans.push(i);
        }
    }

    return ans;
}

#[allow(dead_code)]
/// Returns a list of primes upto the inclusive limit `n` using __sieve of eratosthenes__. [wiki](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes).
///
/// It is done by iteratively marking the multiples of primes as composite upto the given limit. Once the algorithms stops, we are left with prime numbers
///
/// _It uses a lot of memory. Use_ [`primes_upto_n_without_sieve`] _if `n` is very large_
///
/// [`primes_upto_n_without_sieve`]: fn.primes_upto_n_without_sieve.html
///
/// ### Arguments
///
/// * `n` : `u128` - the upper limit upto which we need to find primes.
///
/// ### Returns
///
/// * `Vec<u128>` - List of primes upto and including `n`.
///
/// ### Examples
///
/// ```
/// use project_euler::maths::sieve_of_eratosthenes as f;
///
/// assert_eq!(f(0), vec![]);
/// assert_eq!(f(1), vec![]);
/// assert_eq!(f(3), vec![2, 3]);
/// assert_eq!(f(10), vec![2, 3, 5, 7]);
///
/// ```
pub fn sieve_of_eratosthenes(n: u128) -> Vec<u128> {
    if n < 2 {
        return Vec::new();
    }

    let n: usize = n as usize;

    let mut primes_true_or_false: Vec<bool> = vec![true; n + 1];

    primes_true_or_false[0] = false;
    primes_true_or_false[1] = false;

    for i in 2..=n {
        if primes_true_or_false[i] {
            for col in (i * i..=n).step_by(i) {
                primes_true_or_false[col] = false;
            }
        }
    }

    let primes: Vec<u128> = primes_true_or_false
        .iter()
        .enumerate()
        .filter(|&(_, &b)| b)
        .map(|(a, _)| a as u128)
        .collect::<Vec<u128>>();

    return primes;
}

#[cfg(test)]
mod tests {
    // for detailed testing of functions with edge cases.
    use super::*;

    #[test]
    fn test_all_divisors() {
        let test_cases: Vec<(u128, Vec<u128>)> = vec![
            (0, vec![]),
            (1, vec![1]),
            (2, vec![1, 2]),
            (3, vec![1, 3]),
            (4, vec![1, 2, 4]),
            (8, vec![1, 2, 4, 8]),

        ];

        for (input, expected_output) in test_cases {
            assert_eq!(all_divisors(input), expected_output);
        }
    }

    #[test]
    fn test_int_sqrt() {
        let test_cases: Vec<(u128, u128)> = vec![
            (0, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (4, 2),
            (30, 5),
            (std::u128::MAX, std::u64::MAX as u128),
            (std::u128::MAX - 1, std::u64::MAX as u128),
            (340282366920938463389587631136930004996, 18446744073709551614),
            (std::u64::MAX as u128 + 1, std::u32::MAX as u128 + 1),
            (std::u32::MAX as u128 + 1, std::u16::MAX as u128 + 1),

        ];

        for (input, expected_output) in test_cases {
            assert_eq!(int_sqrt(input), expected_output);
        }
    }

}
