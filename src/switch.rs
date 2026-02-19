pub struct Switch {
    ports_number: usize,
    queue: Vec<Vec<u32>>,
    matching: Vec<usize>,
    hamilton_matching: Vec<usize>,
    time: u32,
    throughput: u32,
    packets_number: u32,
}

impl Switch {
    pub fn new(ports_number: usize) -> Self {
        Self {
            ports_number: ports_number,
            queue: vec![vec![0; ports_number]; ports_number],
            matching: (0..ports_number).collect(),
            hamilton_matching: (0..ports_number).collect(),
            time: 0,
            throughput: 0,
            packets_number: 0,
        }
    }

    pub fn add_a_packet_in_queue(&mut self, input_port: usize, output_port: usize) {
        self.queue[input_port][output_port] += 1;
        self.packets_number += 1;
    }

    pub fn process(&mut self) {
        let matching = &self.matching;
        let queue = &mut self.queue;

        for i in 0..self.ports_number {
            if queue[i][matching[i]] > 0 {
                queue[i][matching[i]] -= 1;
                self.throughput += 1;
            }
        }
        self.time += 1;
    }

    pub fn get_resault(&self) -> f64 {
        let throughput: f64 = self.throughput.into();
        let packets_number: f64 = self.packets_number.into();

        throughput / packets_number
    }

    pub fn next_match(&mut self) {
        let ports = self.ports_number;
        let queue = &self.queue;
        let neighbor_cost = Self::get_best_neighbor_cost(queue, ports, &mut self.matching);
        Self::get_hamiltonian(&mut self.hamilton_matching);
        let hamilton_cost = Self::get_cost(queue, &self.hamilton_matching);

        if neighbor_cost < hamilton_cost {
            self.matching = self.hamilton_matching.clone();
        }
    }

    fn get_best_neighbor_cost(
        queue: &[Vec<u32>],
        ports_number: usize,
        matching: &mut [usize],
    ) -> u32 {
        let (mut ci, mut cj) = (0, 0);
        let cost = Self::get_cost(&queue, &matching);
        let mut max_cost = cost;

        for i in 0..ports_number {
            let cost_i = queue[i][matching[i]];
            for j in (i + 1)..ports_number {
                let cost_j = queue[j][matching[j]];
                let swaped_cost =
                    cost - cost_i - cost_j + queue[i][matching[j]] + queue[j][matching[i]];
                if swaped_cost > max_cost {
                    max_cost = swaped_cost;
                    (ci, cj) = (i, j);
                }
            }
        }
        matching.swap(ci, cj);

        max_cost
    }

    fn get_cost(queue: &[Vec<u32>], matching: &[usize]) -> u32 {
        matching.iter().enumerate().map(|(i, &o)| queue[i][o]).sum()
    }

    fn get_hamiltonian(matching: &mut Vec<usize>) {
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
}
