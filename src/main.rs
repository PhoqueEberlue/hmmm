use std::fmt::Display;

#[derive(Debug)]
struct Processor {
    // The id of the processor
    id: u32,
    
    // The 3 registers
    a: i64,
    b: i64,
    c: i64,
}

struct Hypercube {
    processor_list: Vec<Processor>,
    number_bits_per_dimension: u32,
    number_bits_total: u32,
    number_processors_total: u32,
    number_processor_per_dimension: u32,
}

impl Hypercube {
    fn new(matrix_size: usize) -> Hypercube {
        let number_processors_total = matrix_size.pow(3) as u32; 

        // Computing the number of bits needed for 1 group
        //         n = 2^q
        // <=> ln(n) = ln(2^q)
        // <=> ln(n) = q*ln(2)
        // <=> ln(n) / ln(2) = q
        let number_bits_per_dimension = ((matrix_size as f64).ln() / 2.0_f64.ln()) as u32;
        let number_processor_per_dimension = 2_u32.pow(number_bits_per_dimension);

        // Number of bits to represent a processor id
        let number_bits_total = 3 * number_bits_per_dimension;

        let mut processor_list = Vec::with_capacity(number_processors_total as usize);

        for i in 0..number_processors_total {
            processor_list.push(Processor { id: i as u32, a: 0, b: 0, c: 0 })
        }

        Hypercube { processor_list, number_bits_per_dimension, number_bits_total, number_processors_total, number_processor_per_dimension }
    }

    /// Initialize the hypercube with two matrices
    fn init(&mut self, matrix_a: Vec<Vec<i64>>, matrix_b: Vec<Vec<i64>>) {
        for z in 0..self.number_processor_per_dimension {
            for x in 0..self.number_processor_per_dimension {
                let processor = self.get_mut_processor(x, 0, z);
                processor.a = matrix_a[z as usize][x as usize];
                processor.b = matrix_b[z as usize][x as usize];
            }
        }
    }

    /// Returns a reference of the processor at the given coordinates
    fn get_processor(&self, x: u32, y: u32, z: u32) -> &Processor {
        let proc_id = x + (y << self.number_bits_per_dimension) + (z << (2 * self.number_bits_per_dimension));
        let processor = self.processor_list.get(proc_id as usize);

        match processor {
            Some(proc) => &proc,
            None => panic!("Processor with coordinates x: {x}, y: {y}, z: {z} and proc_id: {proc_id} was not found")
        }
    }

    /// Returns a mutable reference of the processor at the given coordinates
    fn get_mut_processor(&mut self, x: u32, y: u32, z: u32) -> &mut Processor {
        let proc_id = x + (y << self.number_bits_per_dimension) + (z << (2 * self.number_bits_per_dimension));
        let processor = self.processor_list.get_mut(proc_id as usize);

        match processor {
            Some(proc) => proc,
            None => panic!("Processor with coordinates x: {x}, y: {y}, z: {z} and proc_id: {proc_id} was not found")
        }
    }

    fn step_1(&mut self) {
        // Looping through y in reverse
        for y in 1..self.number_processor_per_dimension {
            for z in 0..self.number_processor_per_dimension {
                for x in 0..self.number_processor_per_dimension {
                    // Getting the processor underneath
                    let underneath_processor = self.get_processor(x, y - 1, z);
                    let a_value = underneath_processor.a;
                    let b_value = underneath_processor.b;

                    // Changing a and b registers with the value from the bottom
                    let processor = self.get_mut_processor(x, y, z);
                    processor.a = a_value;
                    processor.b = b_value;
                }
            }
        }
    }

    fn step_2(&mut self) {
        // Looping through x in reverse
        for x in 0..self.number_processor_per_dimension {
            for y in 0..self.number_processor_per_dimension {
                for z in 0..self.number_processor_per_dimension {
                    let next_processor = self.get_processor(y, y, z);
                    let a_value = next_processor.a;

                    let processor = self.get_mut_processor(x, y, z);
                    processor.a = a_value;
                }
            }
        }
    }

    fn step_3(&mut self) {
        // Looping through z in reverse
        for z in 0..self.number_processor_per_dimension {
            for y in 0..self.number_processor_per_dimension {
                for x in 0..self.number_processor_per_dimension {
                    // Getting the processor
                    let next_processor = self.get_processor(x, y, y);
                    let b_value = next_processor.b;

                    // Changing a and b registers with the value from the bottom
                    let processor = self.get_mut_processor(x, y, z);
                    processor.b = b_value;
                }
            }
        }
    }

    fn step_4(&mut self) {
        for x in 0..self.number_processor_per_dimension {
            for y in 0..self.number_processor_per_dimension {
                for z in 0..self.number_processor_per_dimension {
                    // Changing a and b registers with the value from the bottom
                    let processor = self.get_mut_processor(x, y, z);
                    processor.c = processor.a * processor.b;
                }
            }
        }
    }

    fn step_5(&mut self) {
        for y in (0..self.number_processor_per_dimension - 1).rev() {
            for z in 0..self.number_processor_per_dimension {
                for x in 0..self.number_processor_per_dimension {
                    // Getting the processor on the top of the current proccesor index
                    let up_processor = self.get_processor(x, y + 1, z);
                    let c_value = up_processor.c;

                    // Changing a and b registers with the value
                    let processor = self.get_mut_processor(x, y, z);
                    processor.c += c_value;
                }
            }
        }
    }
}

impl Display for Hypercube {
    /// ^ Y
    /// |  / Z
    /// | /
    /// O-----> X
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        res.push_str("         A               B               C\n");

        // Printing for top to bottom
        for y in (0..self.number_processor_per_dimension).rev() {

            // Printing the depth of the cube on multiple lines
            for z in 0..self.number_processor_per_dimension {

                res.push_str(&" ".repeat(2 * (self.number_processor_per_dimension - z - 1) as usize));
                // Printing a register
                for x in 0..self.number_processor_per_dimension {
                    let processor = self.get_processor(x, y, z);
                    res.push_str(&format!("{} ", processor.a));
                }

                res.push_str(&" ".repeat(2*self.number_processor_per_dimension as usize));
                // Printing b register
                for x in 0..self.number_processor_per_dimension {
                    let processor = self.get_processor(x, y, z);
                    res.push_str(&format!("{} ", processor.b));
                }

                res.push_str(&" ".repeat(2*self.number_processor_per_dimension as usize));
                // Printing c register
                for x in 0..self.number_processor_per_dimension {
                    let processor = self.get_processor(x, y, z);
                    res.push_str(&format!("{} ", processor.c));
                }

                res.push_str("\n");
            }
        }

        write!(f, "{}", res)
    }
}

fn main() {
    // TODO: fix cases where the size is not a power of 2
    let mut hypercube = Hypercube::new(4);
    println!("{}", hypercube);

    let matrix_a = vec![
        vec![1,2,0,0],
        vec![0,1,1,3],
        vec![1,0,0,2],
        vec![0,0,2,0],
    ];

    let matrix_b = vec![
        vec![1,2,2,0],
        vec![0,3,2,0],
        vec![1,0,1,2],
        vec![0,1,2,0],
    ];

    hypercube.init(matrix_a, matrix_b);
    println!("Init");
    println!("{}", hypercube);

    hypercube.step_1();
    println!("step 1");
    println!("{}", hypercube);

    hypercube.step_2();
    println!("step 2");
    println!("{}", hypercube);

    hypercube.step_3();
    println!("step 3");
    println!("{}", hypercube);

    hypercube.step_4();
    println!("step 4");
    println!("{}", hypercube);

    hypercube.step_5();
    println!("step 5");
    println!("{}", hypercube);
}

