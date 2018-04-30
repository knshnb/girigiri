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
        "KA" => 4,
        "HI" => 5,
        "KI" => 6,
        "OU" => 7,
        _ => 8,
    }
}
