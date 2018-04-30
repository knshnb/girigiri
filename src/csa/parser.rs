pub fn csa_is_promoted(koma: &str) -> bool {
    match koma {
        "TO" | "NY" | "NK" | "NG" | "UM" | "RY" => true,
        _ => false,
    }
}

pub fn csa_to_kind(koma: &str) -> usize {
    match koma {
        "FU" => 0,
        "KY" => 1,
        "KE" => 2,
        "GI" => 3,
        "KI" => 4,
        "KA" => 5,
        "HI" => 6,
        "OU" => 7,
        _ => 8,
    }
}
