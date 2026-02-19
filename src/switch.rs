use crate::hamiltonian::get_hamiltonian;

pub struct Switch {
    ports_number: usize,
    queue: Vec<Vec<u32>>,
    matching: Vec<usize>,
    time: u32,
    throughput: i32,
    packets_number: u32,
}

impl Switch {
    pub fn new(ports_number: usize) -> Self {
        Self {
            ports_number: ports_number,
            queue: vec![vec![0; ports_number]; ports_number],
            matching: (0..ports_number).collect(),
            time: 0,
            throughput: 0,
            packets_number: 0,
        }
    }

    pub fn add_a_packet_in_queue(&mut self, input_port: usize, output_port: usize) {
        self.queue[input_port][output_port] += 1;
        self.packets_number += 1;
    }

    pub fn next_match(&mut self) {
        let (I, J, best_neighbor_cost) = self.get_best_neighbor();
    }

    fn get_best_neighbor(&self, &mut matching) -> u32 {
        let (mut I, mut J) = (0, 0);
        let cost = self.get_cost(&self.matching);
        let mut max_cost = cost;

        for i in 0..self.ports_number {
            let cost_i = self.queue[i][self.matching[i]];
            for j in (i + 1)..self.ports_number {
                let cost_j = self.queue[j][self.matching[j]];
                let final_cost = cost - cost_i - cost_j
                    + self.queue[i][self.matching[j]]
                    + self.queue[j][self.matching[i]];
                if final_cost > max_cost {
                    max_cost = final_cost;
                    (I, J) = (i, j);
                }
            }
        }

        (I, J, cost)
    }

    fn get_cost(&self, matching: &Vec<usize>) -> u32 {
        let mut res: u32 = 0;
        for (i, &out_port) in matching.iter().enumerate() {
            res += self.queue[i][out_port];
        }
        res
    }
}
