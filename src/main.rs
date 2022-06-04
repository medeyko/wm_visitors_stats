use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // Languages according to ISO 639-1
    const PROJS: [&str; 10] = [
        "all", "wen", "wru", "wja", "wes", "wde", "wfr", "wzh", "wit", "wuk",
    ];
    // Countries accordinng to ISO 3166-1
    const REGS: [&str; 36] = [
        "ww", "su", "cw", "US", "JP", "DE", "GB", "IN", "RU", "FR", "IT", "CA", "PL", "ES", "TW",
        "BR", "AU", "UA", "NL", "TR", "CZ", "FI", "IL", "KZ", "BY", "UZ", "AZ", "LT", "GE", "EE",
        "AM", "LV", "KG", "MD", "TJ", "TM",
    ];
    let ussr = [
        "RU", "UA", "BY", "MD", "LT", "LV", "EE", "AM", "GE", "AZ", "KZ", "TM", "UZ", "KG", "TJ",
    ];
    let west = [
        "US", "CA", "JP", "KR", "AU", "GB", "GI", "GG", "JE", "PT", "ES", "AD", "FR", "BE", "NL",
        "LU", "MC", "CH", "LI", "AT", "IT", "VA", "SM", "DE", "DK", "NO", "SE", "FI", "AX", "GL",
        "IS",
    ];
    let mut table: [[u64; PROJS.len()]; REGS.len()] = [[0; PROJS.len()]; REGS.len()];

    for p in 0..PROJS.len() {
        let csv = fs::read_to_string(format!("../data/{}.csv", PROJS[p]))?;
        let mut reader = csv::Reader::from_reader(csv.as_bytes());
        for rec in reader.records() {
            let rec = rec?;
            let country = &rec[0];
            let count = rec[3].parse()?;
            table[0][p] += count;
            let i = REGS.iter().position(|&x| x == country);
            if let Some(region) = i {
                table[region][p] = count;
            }
            if ussr.iter().position(|&x| x == country).is_some() {
                table[1][p] += count;
            }
            if west.iter().position(|&x| x == country).is_some() {
                table[2][p] += count;
            }
        }
    }

    for prj in PROJS {
        print!("      {} # % %", prj);
    }
    println!();
    for r in 0..REGS.len() {
        print!("{}", REGS[r]);
        for p in 0..PROJS.len() {
            print!("{:7}", table[r][p] / 1000000);
            print!("{:4}", (table[r][0] / 2 + 100 * table[r][p]) / table[r][0]);
            print!("{:4}", (table[0][p] / 2 + 100 * table[r][p]) / table[0][p]);
        }
        println!();
    }
    Ok(())
}
