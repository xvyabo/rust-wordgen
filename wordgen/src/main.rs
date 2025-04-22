use rand::prelude::*;
//use std::time::SystemTime;

fn main() {
    gen_words();
    gen_words();
    gen_words();
    gen_words();
    gen_words();
    println!();
}

fn gen_words() {
    //let start = SystemTime::now();

    let base_vows: Vec<&str> = vec!["a", "e", "i", "o", "u", "y"];
    let base_cons: Vec<&str> = vec![
        "m", "p", "b", "n", "t", "d", "s", "z", "r", "l", "š", "ž", "j", "k", "g", "h", "w",
    ];
    let fin_cons: Vec<&str> = vec![
        "m", "p", "b", "n", "t", "d", "s", "z", "r", "l", "š", "ž", "k", "g",
    ];

    let poss_word: Vec<Vec<&str>> = vec![
        vec!["I", "V", "F", "E", "S", "C"],
        vec!["I", "V", "F", "E", "S"],
        vec!["I", "V", "F", "S", "C"],
        vec!["I", "V", "F", "S"],
        vec!["I", "V", "F"],
        vec!["I", "V", "E", "S"],
        vec!["I", "V", "E", "S", "C"],
        vec!["I", "V"],
        vec!["V", "F", "E", "S", "C"],
        vec!["V", "F", "E", "S"],
        vec!["V", "F", "S", "C"],
        vec!["V", "F", "S"],
        vec!["V", "F"],
        vec!["V", "I", "S", "C"],
        vec!["V", "I", "S"],
        vec!["V"],
    ];

    let mut rng = rand::rng();

    let rnd_base_vow_1: &str = base_vows[rng.random_range(0..=base_vows.len() - 1)];
    let rnd_base_vow_2: &str = base_vows[rng.random_range(0..=base_vows.len() - 1)];
    let rnd_base_cons_1: &str = base_cons[rng.random_range(0..=base_cons.len() - 1)];
    let rnd_base_cons_2: &str = base_cons[rng.random_range(0..=base_cons.len() - 1)];
    let rnd_fin_cons_1: String = fin_cons[rng.random_range(0..=fin_cons.len() - 1)].to_string();
    let rnd_fin_cons_2: String = fin_cons[rng.random_range(0..=fin_cons.len() - 1)].to_string();

    let rnd_word: Vec<&str> = poss_word[rng.random_range(0..=poss_word.len() - 1)].clone();

    println!();
    print!("« ");

    syll_converter(
        rnd_word,
        &choose_init_cons(rnd_base_cons_1),
        &choose_vowel(rnd_base_vow_1),
        &rnd_fin_cons_1,
        &choose_init_cons(rnd_base_cons_2),
        &choose_vowel(rnd_base_vow_2),
        &rnd_fin_cons_2,
    );

    print!(" »");
    //println!();

    //let duration = start.elapsed();

    //println!("{duration:?}");
}

fn syll_converter<'a>(
    rnd_word: Vec<&'a str>,
    init_cons_1: &'a String,
    vow_1: &'a String,
    fin_cons_1: &'a String,
    init_cons_2: &'a String,
    vow_2: &'a String,
    fin_cons_2: &'a String,
) {
    let mut rnd_word_defs = vec![];

    for x in rnd_word {
        if x == "I" {
            let i_def = vec!["I", init_cons_1];
            rnd_word_defs.push(i_def);
        } else if x == "V" {
            let v_def = vec!["V", vow_1];
            rnd_word_defs.push(v_def);
        } else if x == "F" {
            let f_def = vec!["F", fin_cons_1];
            rnd_word_defs.push(f_def);
        } else if x == "E" {
            let e_def = vec!["E", init_cons_2];
            rnd_word_defs.push(e_def);
        } else if x == "S" {
            let s_def = vec!["S", vow_2];
            rnd_word_defs.push(s_def);
        } else if x == "C" {
            let c_def = vec!["C", fin_cons_2];
            rnd_word_defs.push(c_def);
        }
    }

    let mut f_counter = 0;
    let f_index = loop {
        f_counter += 1;
        if rnd_word_defs[f_counter - 1].iter().any(|&f| f == "F") {
            break (rnd_word_defs
                .iter()
                .position(|i| i == &rnd_word_defs[f_counter - 1]))
            .unwrap();
        }
        if f_counter == rnd_word_defs.len() {
            break 0;
        }
    };

    let mut e_counter = 0;
    let e_index = loop {
        e_counter += 1;
        if rnd_word_defs[e_counter - 1].iter().any(|&e| e == "E") {
            break (rnd_word_defs
                .iter()
                .position(|i| i == &rnd_word_defs[e_counter - 1]))
            .unwrap();
        }
        if e_counter == rnd_word_defs.len() {
            break 0;
        }
    };

    let unvoiced_cons: Vec<&str> = vec!["p", "t", "s", "š", "k", "h"];
    let voiced_cons: Vec<&str> = vec!["m", "b", "n", "d", "z", "r", "l", "ž", "j", "g", "w"];

    if f_index > 0 && e_index > 0 {
        let f_def = rnd_word_defs[f_index][1];
        let e_def = rnd_word_defs[e_index][1];
        if f_def == e_def
            || (unvoiced_cons.iter().any(|&w| w == f_def)
                && voiced_cons
                    .iter()
                    .any(|&w| w == e_def.chars().nth(0).unwrap().to_string().as_str()))
            || (voiced_cons.iter().any(|&w| w == f_def)
                && unvoiced_cons
                    .iter()
                    .any(|&w| w == e_def.chars().nth(0).unwrap().to_string().as_str()))
            || (f_def == "n" && e_def == "m")
            || (f_def == "m" && e_def == "n")
            || (f_def == "l" && e_def == "r")
            || (f_def == "r" && e_def == "l")
        {
            rnd_word_defs.remove(f_index);
        }
    }

    for c in rnd_word_defs {
        print!("{}", c[1]);
    }
    // println!("{rnd_word_defs:?}");
}

fn choose_vowel(rnd_base_vow: &str) -> String {
    let mut rng = rand::rng();
    let vow_pairs;
    let rnd_vow_pairing;
    let rnd_vow_paired;

    let chosen_vowel: String = if rnd_base_vow == "a" || rnd_base_vow == "e" {
        vow_pairs = vec![vec![""], vec!["i", "u"]];

        rnd_vow_pairing = vow_pairs[rng.random_range(0..=vow_pairs.len() - 1)].clone();

        rnd_vow_paired = if rnd_vow_pairing == vow_pairs[0] {
            vow_pairs[0][0].to_string()
        } else {
            rnd_vow_pairing[rng.random_range(0..=rnd_vow_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_vow}{rnd_vow_paired}")
    } else if rnd_base_vow == "o" {
        vow_pairs = vec![vec![""], vec!["a", "i"]];

        rnd_vow_pairing = vow_pairs[rng.random_range(0..=vow_pairs.len() - 1)].clone();

        rnd_vow_paired = if rnd_vow_pairing == vow_pairs[0] {
            vow_pairs[0][0].to_string()
        } else {
            rnd_vow_pairing[rng.random_range(0..=rnd_vow_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_vow}{rnd_vow_paired}")
    } else {
        rnd_base_vow.to_string()
    };

    return chosen_vowel;
}

fn choose_init_cons(rnd_base_cons: &str) -> String {
    let mut rng = rand::rng();
    let cons_pairs;
    let rnd_cons_pairing;
    let rnd_cons_paired;

    let chosen_init_cons = if rnd_base_cons == "m" || rnd_base_cons == "n" {
        cons_pairs = vec![vec![""], vec!["j", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "b" || rnd_base_cons == "p" {
        cons_pairs = vec![vec![""], vec!["r", "j", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "t" {
        cons_pairs = vec![vec![""], vec!["s", "j", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "d" {
        cons_pairs = vec![vec![""], vec!["z", "j", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "s" {
        cons_pairs = vec![vec![""], vec!["m", "p", "n", "t", "r", "k", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "z" {
        cons_pairs = vec![vec![""], vec!["m", "b", "n", "d", "r", "g", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "š" {
        cons_pairs = vec![vec![""], vec!["m", "p", "n", "t", "k", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else if rnd_base_cons == "ž" {
        cons_pairs = vec![vec![""], vec!["m", "b", "n", "d", "g", "w"]];

        rnd_cons_pairing = cons_pairs[rng.random_range(0..=cons_pairs.len() - 1)].clone();

        rnd_cons_paired = if rnd_cons_pairing == cons_pairs[0] {
            cons_pairs[0][0].to_string()
        } else {
            rnd_cons_pairing[rng.random_range(0..=rnd_cons_pairing.len() - 1)].to_string()
        };

        format!("{rnd_base_cons}{rnd_cons_paired}")
    } else {
        rnd_base_cons.to_string()
    };

    return chosen_init_cons;
}
