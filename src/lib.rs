pub mod error;
pub mod opts;
pub mod stats;

pub type Result<T, E = error::MCError> = std::result::Result<T, E>;

pub fn human_time(o: bool, t: &usize) -> String {
    if !o {
        return format!("{}", t);
    }
    let mut r = String::new();
    let d = t / 60 / 60 / 24;
    let h = (t - d * 60 * 60 * 24) / 60 / 60;
    let m = (t - (d * 60 * 60 * 24) - (h * 60 * 60)) / 60;
    let s = t - (d * 60 * 60 * 24) - (h * 60 * 60) - (m * 60);

    if d > 0 {
        r.push_str(format!("{}d", d).as_str());
    }
    if h > 0 {
        r.push_str(format!("{}h", h).as_str());
    }
    if m > 0 {
        r.push_str(format!("{}m", m).as_str());
    }
    if s > 0 {
        r.push_str(format!("{}s", s).as_str());
    }
    if s == 0 && r.is_empty() {
        r.push_str("0s")
    }
    r.trim().to_string()
}

pub fn human_size(o: bool, t: &usize) -> String {
    if !o {
        return format!("{}", t);
    }
    if *t >= 1024 * 1024 * 1024 {
        format!("{:.1}g", *t as f64 / 1024.0 / 1024.0 / 1024.0)
    } else if *t >= 1024 * 1024 {
        format!("{:.1}m", *t as f64 / 1024.0 / 1024.0)
    } else if *t >= 1024 {
        format!("{:.1}k", *t as f64 / 1024.0)
    } else {
        format!("{:}b", *t as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_bytes_test() {
        let bytes = [
            0,
            12,
            1024,
            1296,
            8192,
            1024 * 1024,
            1024 * 1024 * 1024,
            1024 * 1024 * 1024 + 1024 * 1024 * 512,
        ]
        .to_vec();

        let hbytes = ["0b", "12b", "1.0k", "1.3k", "8.0k", "1.0m", "1.0g", "1.5g"].to_vec();

        for (i, b) in bytes.iter().enumerate() {
            assert_eq!(hbytes[i].to_string(), human_size(true, b));
        }
    }

    #[test]
    fn simple_time_test() {
        let times = [
            0, 15, 60, 65, 119, 120, 3590, 3600, 3660, 86390, 86400, 100000,
        ]
        .to_vec();

        let htimes = [
            "0s",
            "15s",
            "1m",
            "1m5s",
            "1m59s",
            "2m",
            "59m50s",
            "1h",
            "1h1m",
            "23h59m50s",
            "1d",
            "1d3h46m40s",
        ]
        .to_vec();

        for (i, t) in times.iter().enumerate() {
            assert_eq!(htimes[i].to_string(), human_time(true, t));
        }
    }
}
