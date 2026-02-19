pub fn get_hamiltonian(time: u32, ports_number: usize) -> Vec<usize> {
    let mut matching: Vec<usize> = (0..ports_number).collect();

    for _ in 0..(time + 1) {
        next_permutation(&mut matching);
    }

    matching
}

fn next_permutation(matching: &mut Vec<usize>) {
    let mut k = matching.len() - 2;

    while matching[k] > matching[k + 1] {
        k -= 1;
    }

    if k == 0 {
        return;
    }

    let mut j = matching.len() - 1;

    while matching[k] > matching[j] {
        j -= 1;
    }
    matching.swap(j, k);

    let mut r = matching.len() - 1;

    k += 1;

    while r > k {
        matching.swap(r, k);
        r -= 1;
        k += 1;
    }
}
