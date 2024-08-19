use rand::seq::SliceRandom;
use std::collections::HashMap;

// Copies Vector and Returns a Shuffled Vector.
fn clone_shuffle(p: Vec<&str>) -> Vec<&str> {
    let mut kp = p.clone();
    kp.as_mut_slice().shuffle(&mut rand::thread_rng());
    kp
}

// Method to reshuffle at Algortihm 
fn reshuffle(p: Vec<&str>) -> Vec<&str> {
    let mut rs = p.clone();
    rs.as_mut_slice().shuffle(&mut rand::thread_rng());
    rs.to_vec()
}

// Pairs a Buyer and Receiver.
fn pair_br(p: Vec<&str>) -> HashMap<&str, &str> {
    let mut ssl: HashMap<&str, &str> = HashMap::new();

    // Shuffle buyers and receivers
    let mut buyers = p.clone();
    let mut receivers = p.clone();

    // Ensure the vectors are different
    loop {
        buyers.shuffle(&mut rand::thread_rng());
        receivers.shuffle(&mut rand::thread_rng());

        // Check if no one is paired with themselves
        let mut valid = true;
        for i in 0..p.len() {
            if buyers[i] == receivers[i] {
                valid = false;
                break;
            }
        }

        if valid {
            break;
        }
    }

    for i in 0..p.len() {
        ssl.insert(buyers[i], receivers[i]);
    }

    ssl
}

// Output for Secret Santa List 
fn output_ssl(ssl: HashMap<&str, &str>) {
    for x in ssl {
        println!("{} is buying for {}", x.0, x.1);
    }
}

fn main() {
    // Participants
    let parts: Vec<&str> = vec![
        "Jack", "Cal", "Scott", "Fara", "Dom", "Dan", "Liam", "Kirsty", "Hehe"
    ];

    // Secret Santa List
    let ssl: HashMap<&str, &str> = pair_br(parts);

    output_ssl(ssl);
}
