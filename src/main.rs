use clap::{Parser, Subcommand};
use gauss_int::{GaussInt, BigInt};

#[derive(Parser)]
#[command(name = "gauss", about = "Gaussian integer and number theory CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add two Gaussian integers
    Add { a: String, b: String },
    /// Subtract two Gaussian integers
    Sub { a: String, b: String },
    /// Multiply two Gaussian integers
    Mul { a: String, b: String },
    /// Divide two Gaussian integers (shows quotient and remainder)
    Div { a: String, b: String },
    /// GCD of two Gaussian integers
    Gcd { a: String, b: String },
    /// Norm of a Gaussian integer
    Norm { z: String },
    /// Conjugate of a Gaussian integer
    Conj { z: String },
    /// Check if a number is prime
    #[command(name = "is-prime")]
    IsPrime { n: String },
    /// Factorize a number
    Factor { n: String },
    /// Euler's totient φ(n)
    Totient { n: String },
    /// Jacobi symbol (a/n)
    Jacobi { a: String, n: String },
    /// Chinese Remainder Theorem
    Crt { pairs: Vec<String> },
}

/// Parse a Gaussian integer string like "3+4i", "5", "-2i", "i", "-i"
fn parse_gauss(s: &str) -> Result<GaussInt, String> {
    let s = s.trim();

    if s == "0" {
        return Ok(GaussInt::from_i64(0, 0));
    }

    // Handle pure imaginary: "i", "-i", "3i", "-5i"
    if s.ends_with('i') {
        let before_i = &s[..s.len() - 1];
        if before_i.is_empty() {
            return Ok(GaussInt::from_i64(0, 1));
        }
        if before_i == "+" {
            return Ok(GaussInt::from_i64(0, 1));
        }
        if before_i == "-" {
            return Ok(GaussInt::from_i64(0, -1));
        }
        // Check if it's a single number (pure imaginary): "3i", "-5i"
        // These have no + or - separator (except the sign at position 0)
        let has_separator = before_i[1..].contains('+') || before_i[1..].contains('-');
        if !has_separator {
            let imag = before_i
                .parse::<i64>()
                .map_err(|_| format!("invalid Gaussian integer: {}", s))?;
            return Ok(GaussInt::from_i64(0, imag));
        }
        // Complex with real and imag: "3+4i", "3-4i"
        // Find the separator after the first character
        let sep_pos = before_i[1..]
            .find(|c: char| c == '+' || c == '-')
            .map(|pos| pos + 1) // +1 because we started from index 1
            .ok_or_else(|| format!("invalid Gaussian integer: {}", s))?;

        let real_str = &before_i[..sep_pos];
        let imag_str = &before_i[sep_pos..];
        let real: i64 = real_str
            .parse()
            .map_err(|_| format!("invalid real part: {}", real_str))?;
        let imag: i64 = imag_str
            .parse()
            .map_err(|_| format!("invalid imaginary part: {}", imag_str))?;
        return Ok(GaussInt::from_i64(real, imag));
    }

    // Pure real
    let real: i64 = s.parse().map_err(|_| format!("invalid number: {}", s))?;
    Ok(GaussInt::from_i64(real, 0))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { a, b } => {
            let z1 = parse_gauss(&a).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let z2 = parse_gauss(&b).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            println!("{}", &z1 + &z2);
        }
        Commands::Sub { a, b } => {
            let z1 = parse_gauss(&a).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let z2 = parse_gauss(&b).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            println!("{}", &z1 - &z2);
        }
        Commands::Mul { a, b } => {
            let z1 = parse_gauss(&a).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let z2 = parse_gauss(&b).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            println!("{}", &z1 * &z2);
        }
        Commands::Div { a, b } => {
            let z1 = parse_gauss(&a).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let z2 = parse_gauss(&b).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            match z1.div_rem(&z2) {
                Some((q, r)) => println!("quotient: {}\nremainder: {}", q, r),
                None => println!("division by zero"),
            }
        }
        Commands::Gcd { a, b } => {
            let z1 = parse_gauss(&a).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            let z2 = parse_gauss(&b).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            println!("{}", z1.gcd(&z2));
        }
        Commands::Norm { z } => {
            let z = parse_gauss(&z).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            println!("{}", z.norm());
        }
        Commands::Conj { z } => {
            let z = parse_gauss(&z).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            });
            println!("{}", z.conjugate());
        }
        Commands::IsPrime { n } => {
            let n = BigInt::from_string(&n).unwrap_or_else(|| {
                eprintln!("Error: invalid number: {}", n);
                std::process::exit(1);
            });
            println!("{}", gauss_int::number_theory::is_prime(&n));
        }
        Commands::Factor { n } => {
            let n = BigInt::from_string(&n).unwrap_or_else(|| {
                eprintln!("Error: invalid number: {}", n);
                std::process::exit(1);
            });
            let factors = gauss_int::number_theory::factorize(&n);
            for (p, e) in &factors {
                println!("{}^{}", p, e);
            }
        }
        Commands::Totient { n } => {
            let n = BigInt::from_string(&n).unwrap_or_else(|| {
                eprintln!("Error: invalid number: {}", n);
                std::process::exit(1);
            });
            println!("{}", gauss_int::number_theory::euler_totient(&n));
        }
        Commands::Jacobi { a, n } => {
            let a = BigInt::from_string(&a).unwrap_or_else(|| {
                eprintln!("Error: invalid number: {}", a);
                std::process::exit(1);
            });
            let n = BigInt::from_string(&n).unwrap_or_else(|| {
                eprintln!("Error: invalid number: {}", n);
                std::process::exit(1);
            });
            println!("{}", gauss_int::number_theory::jacobi_symbol(&a, &n));
        }
        Commands::Crt { pairs } => {
            if pairs.len() < 2 || pairs.len() % 2 != 0 {
                eprintln!("Error: CRT requires pairs of a m values (e.g., '2 3 3 5' for x≡2 mod3, x≡3 mod5)");
                std::process::exit(1);
            }
            let congruences: Vec<(BigInt, BigInt)> = pairs
                .chunks(2)
                .map(|c| {
                    let a = BigInt::from_string(&c[0]).unwrap_or_else(|| {
                        eprintln!("Error: invalid number: {}", c[0]);
                        std::process::exit(1);
                    });
                    let m = BigInt::from_string(&c[1]).unwrap_or_else(|| {
                        eprintln!("Error: invalid number: {}", c[1]);
                        std::process::exit(1);
                    });
                    (a, m)
                })
                .collect();
            match gauss_int::number_theory::crt(&congruences) {
                Some(x) => println!("{}", x),
                None => println!("no solution"),
            }
        }
    }
}
