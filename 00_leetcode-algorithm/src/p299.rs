pub fn get_hint(secret: String, guess: String) -> String {
    let mut bulls = 0;
    let mut cows = 0;

    let mut secret_ht = [0; 10];
    let mut guess_ht = [0; 10];

    for (&ns, &ng) in secret.as_bytes().iter().zip(guess.as_bytes()) {
        if ns == ng {
            bulls += 1;
        } else {
            secret_ht[(ns - b'0') as usize] += 1;
            guess_ht[(ng - b'0') as usize] += 1;
        }
    }

    for i in 0..10 {
        cows += secret_ht[i].min(guess_ht[i]);
    }

    format!("{bulls}A{cows}B")
}
