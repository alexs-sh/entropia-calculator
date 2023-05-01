use clap::Parser;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::path::Path;

struct Report {
    block_count: usize,
    byte_count: usize,
    values: Vec<f64>,
}

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    file: String,

    #[arg(short, long, default_value_t = 1048576)]
    block_size: usize,

    #[arg(short, long, default_value_t = 0)]
    limit: usize,

    #[arg(short, long, default_value_t = String::new())]
    ppm_report: String,
}

fn calc_entropy(data: &[u8]) -> f64 {
    let mut alpha = [0usize; 256];
    for byte in data {
        alpha[*byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut es = 0.0_f64;

    for v in alpha.iter().filter(|e| **e > 0) {
        let p = *v as f64 / len;
        let e = p * p.log2();
        es += e;
    }
    es.abs()
}

fn analyze_file(name: &Path, block_size: usize, limit: usize) -> Result<Report> {
    let file = std::fs::File::open(name)?;
    let mut buffer = vec![0_u8; block_size];
    let mut io = BufReader::new(file);
    let mut report = Report {
        block_count: 0,
        byte_count: 0,
        values: Vec::new(),
    };

    loop {
        match io.read(&mut buffer) {
            Err(_) => break,
            Ok(0) => break,
            Ok(len) => {
                let e = calc_entropy(&buffer[0..len]);
                report.block_count += 1;
                report.byte_count += len;
                report.values.push(e);
            }
        }

        if limit != 0 && report.byte_count >= limit {
            break;
        }
    }
    Ok(report)
}

fn report_stdout(report: &Report) -> Result<()> {
    println!("--------------------------------------------------------------------------------");
    println!("Block       Entropy ");
    println!("--------------------------------------------------------------------------------");
    for (n, v) in report.values.iter().enumerate() {
        println!("{:0>8}    {:.3}", n, v);
    }
    Ok(())
}

fn report_ppm(file: &Path, report: &Report) -> Result<()> {
    let width = report.block_count;
    let height = std::cmp::max(width / 10, 1);
    let max_color = 255.0;
    let max_value = 8.0;
    let file = std::fs::File::create(file)?;
    let mut io = BufWriter::new(file);

    io.write_all("P3\n".as_bytes())?;
    io.write_all(format!("{} {}\n", width, height).as_bytes())?;
    io.write_all(format!("{}\n", max_color as usize).as_bytes())?;

    for _ in 0..height {
        for e in &report.values {
            let c = (max_color / max_value) * e;
            let r = c as u8;
            let b = (max_color - c) as u8;
            io.write_all(format!("{} {} {}\n", r, 0, b).as_bytes())?;
        }
    }
    Ok(())
}

fn make_report(args: &Args, report: &Report) -> Result<()> {
    if args.ppm_report.is_empty() {
        report_stdout(report)
    } else {
        report_ppm(Path::new(&args.ppm_report), report)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let report = analyze_file(Path::new(&args.file), args.block_size, args.limit).map_err(|e| {
        println!("can't read input file:{}", args.file);
        e
    })?;

    make_report(&args, &report).map_err(|e| {
        println!("can't create report");
        e
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_entropty_test() {
        // some control values
        assert_eq!(calc_entropy(&[1]), 0.0);
        assert_eq!(calc_entropy(&[1, 0]), 1.0);
        assert!(calc_entropy(&[1, 0, 1]) < 0.92);
        assert!(calc_entropy(&[1, 0, 1]) > 0.91);
    }
}
