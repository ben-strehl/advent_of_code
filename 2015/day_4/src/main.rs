use md5;

fn main() {
    let key = "iwrupvqb";
    let mut input = 1;
    let mut done = false;

    while !done {
        let input_str = format!("{}{}", key, input.to_string());
        let digest = md5::compute(input_str.as_bytes());
        if has_zeroes(&digest) {
            println!("{input}");
            println!("{digest:?}");
            done = true;
        }

        input += 1;
    }
}

fn has_zeroes(digest: &md5::Digest) -> bool {
    let mut count = 0;
    for byte in digest.into_iter() {
        if byte.leading_zeros() >= 4 {
            count += 1;
        } else {
            return false;
        }
        if byte.trailing_zeros() >= 4 {
            count += 1;
        } else {
            return false;
        }
        if count >= 6 {
            return true;
        }
    }

    false
}

