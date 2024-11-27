use std::io;

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");
        println!("Booth's Algorithm Multiplier");
        println!("Enter first number (multiplicand) or 'q' to quit:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if input.trim().eq_ignore_ascii_case("q") {
            println!("Exiting...");
            break;
        }

        let multiplicand: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        println!("Enter second number (multiplier):");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let multiplier: i64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        println!("Enter number of bits:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        let n_bits: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                continue;
            }
        };

        seq_circuit_booths(multiplicand, multiplier, n_bits);
        println!("\nPress Enter to continue...");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");
    }
}

fn print_hline(n_bits: usize) {
    for _ in 0..n_bits + 1 {
        print!("-");
    }

    println!("-");
}

fn seq_circuit_booths(multiplicand: i64, multiplier: i64, n_bits: usize) -> i64 {
    let mask = (1 << n_bits) - 1;
    let mut a = 0;
    let m = multiplicand;
    let mut q = multiplier;
    let mut q_neg1 = 0;

    let m_masked = m & mask;
    let q_masked = q & mask;

    let print_len = n_bits + 2;
    println!(
        "\nPerforming Booth's algorithm multiplication on {m} ({m_masked:#0n$b}) and {q} ({q_masked:#0n$b})\n",
        n = print_len
    );

    q &= mask;

    for i in 0..n_bits {
        let q0_qneg1 = ((q & 1) << 1) | q_neg1 & 1;

        println!("Iteration {itr}:\n", itr = i + 1);
        println!("Q_0 Q_-1 = {q0_qneg1:02b}");

        match q0_qneg1 {
            0b01 => {
                let mut a_masked = a & mask;
                let m_masked = m & mask;
                println!("A <- A + M:");
                println!("  {a_masked:0n$b}", n = n_bits);
                println!("+ {m_masked:0n$b}", n = n_bits);
                print_hline(n_bits);
                a += m;
                a_masked = a & mask;
                println!("  {a_masked:0n$b}", n = n_bits);
            }
            0b10 => {
                let mut a_masked = a & mask;
                let neg_m = (!m + 1) & mask;
                println!("A <- A - M:");
                println!("  {a_masked:0n$b}", n = n_bits);
                println!("+ {neg_m:0n$b}", n = n_bits);
                print_hline(n_bits);
                a -= m;
                a_masked = a & mask;
                println!("  {a_masked:0n$b}", n = n_bits);
            }
            _ => (),
        }

        let mut a_masked = a & mask;

        println!("Shift right A Q Q_-1:");
        println!(
            "A: {:0n$b} Q: {:0n$b} Q_neg1: {}",
            a_masked,
            q,
            q_neg1,
            n = n_bits
        );
        q_neg1 = q & 1;
        q = (q >> 1) | ((a & 1) << (n_bits - 1));
        a >>= 1;

        a_masked = a & mask;
        q &= mask;

        println!(
            "A: {:0n$b} Q: {:0n$b} Q_neg1: {}\n",
            a_masked,
            q,
            q_neg1,
            n = n_bits
        );
    }

    let aq = (a << n_bits) | q;
    println!("Result: {aq}");
    aq
}
