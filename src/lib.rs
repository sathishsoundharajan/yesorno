const YES_STRING: [&str; 6] = ["yes", "true", "y", "proceed", "on", "yup"];
const NO_STRING: [&str; 6] = ["no", "false", "n", "nope", "off", "halt"];

const YES_MATCH_SCORE_THRESHOLD: f64 = 2.0;
const NO_MATCH_SCORE_THRESHOLD: f64 = 1.25;

pub fn is_yes(key: &str) -> bool {
    if YES_STRING.contains(&key) {
        return true;
    }
    return false;
}

pub fn is_no(key: &str) -> bool {
    if NO_STRING.contains(&key) {
        return true;
    }
    return false;
}

fn y_score(key: &str) -> f64 {
    return match key {
        "y" => 1.0,
        "t" => 0.75,
        "u" => 0.75,
        "g" => 0.25,
        "h" => 0.25,
        "j" => 0.25,
        _ => 0.0,
    };
}

fn e_score(key: &str) -> f64 {
    return match key {
        "e" => 1.0,
        "w" => 0.75,
        "r" => 0.75,
        "s" => 0.25,
        "d" => 0.25,
        "f" => 0.25,
        _ => 0.0,
    };
}

fn s_score(key: &str) -> f64 {
    return match key {
        "s" => 1.0,
        "w" => 0.25,
        "r" => 0.75,
        "q" => 0.25,
        "e" => 0.25,
        "a" => 0.75,
        "d" => 0.75,
        "z" => 0.25,
        "x" => 0.25,
        "c" => 0.25,
        _ => 0.0,
    };
}

fn n_score(key: &str) -> f64 {
    return match key {
        "n" => 1.0,
        "h" => 0.25,
        "j" => 0.25,
        "k" => 0.25,
        "b" => 0.75,
        "m" => 0.75,
        _ => 0.0,
    };
}

fn o_score(key: &str) -> f64 {
    return match key {
        "o" => 1.0,
        "i" => 0.75,
        "p" => 0.75,
        "k" => 0.25,
        "l" => 0.25,
        _ => 0.0,
    };
}

pub fn is_lenient_yes(key: &str) -> bool {
    let mut score = 0.0;
    let first: &str = &key[0..1];
    score += y_score(&first);
    if key.len() > 1 {
        let second: &str = &key[1..2];
        score += e_score(&second);
    }
    if key.len() > 2 {
        let third: &str = &key[2..3];
        score += s_score(&third);
    }
    return score > YES_MATCH_SCORE_THRESHOLD;
}

pub fn is_lenient_no(key: &str) -> bool {
    let mut score = 0.0;
    let first: &str = &key[0..1];
    score += n_score(&first);
    if key.len() > 1 {
        let second: &str = &key[1..2];
        score += o_score(&second);
    }
    return score > NO_MATCH_SCORE_THRESHOLD;
}

#[cfg(test)]
mod tests {
    use crate::is_lenient_no;
    use crate::is_lenient_yes;
    use crate::is_no;
    use crate::is_yes;

    #[test]
    fn test_yes_returns_true() {
        assert_eq!(is_yes("yes"), true);
        assert_eq!(is_yes("true"), true);
        assert_eq!(is_yes("y"), true);
        assert_eq!(is_yes("proceed"), true);
        assert_eq!(is_yes("on"), true);
        assert_eq!(is_yes("on"), true);
    }

    #[test]
    fn test_yes_returns_false() {
        assert_eq!(is_yes("e"), false);
        assert_eq!(is_yes("u"), false);
        assert_eq!(is_yes("yesh"), false);
        assert_eq!(is_yes("yp"), false);
    }

    #[test]
    fn test_no_returns_true() {
        assert_eq!(is_no("no"), true);
        assert_eq!(is_no("nope"), true);
        assert_eq!(is_no("n"), true);
        assert_eq!(is_no("false"), true);
    }

    #[test]
    fn test_no_returns_false() {
        assert_eq!(is_no("o"), false);
        assert_eq!(is_no("u"), false);
        assert_eq!(is_no("nesh"), false);
        assert_eq!(is_no("np"), false);
    }

    #[test]
    fn test_is_lenient_yes_returns_true() {
        assert_eq!(is_lenient_yes("yds"), true);
        assert_eq!(is_lenient_yes("ues"), true);
        assert_eq!(is_lenient_yes("ywa"), true);
        assert_eq!(is_lenient_yes("tes"), true);
        assert_eq!(is_lenient_yes("twa"), true);
        assert_eq!(is_lenient_yes("urd"), true);
    }

    #[test]
    fn test_is_lenient_no_returns_true() {
        assert_eq!(is_lenient_no("ni"), true);
        assert_eq!(is_lenient_no("bi"), true);
        assert_eq!(is_lenient_no("mo"), true);
    }
}