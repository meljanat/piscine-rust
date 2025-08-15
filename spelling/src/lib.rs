pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }
    if n == 1_000_000 {
        return "one million".to_string();
    }

    let units = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    fn spell_under_100(n: u64, units: &[&str], tens: &[&str]) -> String {
        if n < 20 {
            units[n as usize].to_string()
        } else {
            let t = tens[(n / 10) as usize];
            let u = units[(n % 10) as usize];
            if u.is_empty() {
                t.to_string()
            } else {
                format!("{}-{}", t, u)
            }
        }
    }

    fn spell_under_1000(n: u64, units: &[&str], tens: &[&str]) -> String {
        if n < 100 {
            return spell_under_100(n, units, tens);
        }
        let hundreds = n / 100;
        let rest = n % 100;
        if rest == 0 {
            format!("{} hundred", units[hundreds as usize])
        } else {
            format!(
                "{} hundred {}",
                units[hundreds as usize],
                spell_under_100(rest, units, tens)
            )
        }
    }

    let mut parts = Vec::new();

    let thousands = n / 1000;
    let remainder = n % 1000;

    if thousands > 0 {
        parts.push(spell_under_1000(thousands, &units, &tens) + " thousand");
    }
    if remainder > 0 {
        parts.push(spell_under_1000(remainder, &units, &tens));
    }

    parts.join(" ")
}
