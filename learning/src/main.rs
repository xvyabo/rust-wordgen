fn main() {
    let i = 65;
    let v = 12;
    let f = 2;

    let ivfivf = i * v * f * i * v * f;
    let ivfiv = i * v * f * i * v;
    let ivfvf = i * v * f * v * f;
    let ivfv = i * v * f * v;
    let ivf = i * v * f;
    let iviv = i * v * i * v;
    let ivivf = i * v * i * v * f;
    let iv = i * v;
    let vfivf = v * f * i * v * f;
    let vfiv = v * f * i * v;
    let vfvf = v * f * v * f;
    let vfv = v * f * v;
    let vf = v * f;
    let vivf = v * i * v * f;
    let viv = v * i * v;
    // add v

    let two_syll_sum = ivfivf
        + ivfiv
        + ivfvf
        + ivfv
        + ivf
        + iviv
        + ivivf
        + iv
        + vfivf
        + vfiv
        + vfvf
        + vfv
        + vf
        + vivf
        + viv
        + v;

    let one_syll_sum = ivf + iv + vf + v;

    println!(
        "Maximum of 1 syllable: {}, Maximum of 2 syllables: {}",
        one_syll_sum, two_syll_sum
    );
}
