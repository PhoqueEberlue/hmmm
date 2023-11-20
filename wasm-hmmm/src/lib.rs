mod utils;
use wasm_bindgen::prelude::*;
use std::fmt::Display;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Processor {
    // The id of the processor
    id: u32,
    
    // The 3 registers
    pub a: i64,
    pub b: i64,
    pub c: i64,
}

#[wasm_bindgen]
pub struct Hypercube {
    processor_list: Vec<Processor>,
    pub number_bits_per_dimension: u32,
    pub number_processor_per_dimension: u32,
    pub number_bits_total: u32,
    pub number_processors_total: u32,
    current_step: usize,
}

/// ^ Y
/// |  / Z
/// | /
/// O- - - > X
#[wasm_bindgen]
impl Hypercube {
    pub fn new(matrix_size: usize) -> Hypercube {
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

        Hypercube { processor_list, number_bits_per_dimension, number_bits_total, number_processors_total, number_processor_per_dimension, current_step: 0 }
    }

    /// Initialize the hypercube with two matrices
    pub fn init(&mut self, matrix_a: &[i64], matrix_b: &[i64]) {
        let number_processor_per_dimension = self.number_processor_per_dimension;
        for z in 0..number_processor_per_dimension {
            for x in 0..number_processor_per_dimension {
                let processor = self.get_mut_processor(x, 0, z);
                processor.a = *matrix_a.get((z * number_processor_per_dimension + x) as usize).unwrap();
                processor.b = *matrix_b.get((z * number_processor_per_dimension + x) as usize).unwrap();
            }
        }
    } 

    pub fn next_step(&mut self) -> bool {
        match self.current_step {
            0 => { self.step_1(); self.current_step += 1; true },
            1 => { self.step_2(); self.current_step += 1; true },
            2 => { self.step_3(); self.current_step += 1; true },
            3 => { self.step_4(); self.current_step += 1; true },
            4 => { self.step_5(); self.current_step += 1; true },
            _ => { false }
        }
    }

    pub fn get_state(&self) -> Vec<Processor> {
        self.processor_list.clone()
    }

    pub fn get_result(&self) -> Box<[i64]> {
        let mut res = vec![];

        for z in 0..self.number_processor_per_dimension {
            for x in 0..self.number_processor_per_dimension {
                res.push(self.get_processor(x, 0, z).c);
            }
        }

        res.into()
    }

    pub fn get_processor_copy(&self, x: u32, y: u32, z: u32) -> Processor {
        self.get_processor(x, y, z).clone()
    }

    pub fn repr_register(&self, register_name: String) -> String {
        let mut res = String::new();

        // Printing from top to bottom
        for y in (0..self.number_processor_per_dimension).rev() {

            // Printing the depth of the cube on multiple lines
            for z in 0..self.number_processor_per_dimension {

                res.push_str(&" ".repeat(2 * (self.number_processor_per_dimension - z - 1) as usize));
                // Printing a register
                for x in 0..self.number_processor_per_dimension {
                    let processor = self.get_processor(x, y, z);
                    let value = match register_name.as_str() {
                        "a" => processor.a,
                        "b" => processor.b,
                        "c" => processor.c,
                        _ => panic!("")
                        
                    };

                    res.push_str(&format!("{} ", value));
                } 

                res.push_str("\n");
            }
        }

        res
    }
}

/// Private hypercube functions
impl Hypercube {
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
        // Looping through y by skipping the first index because we access the underneath value
        for y in 1..self.number_processor_per_dimension {
            for z in 0..self.number_processor_per_dimension {
                for x in 0..self.number_processor_per_dimension {
                    // Getting the processor underneath
                    let underneath_processor = self.get_processor(x, y - 1, z);
                    let a_value = underneath_processor.a;
                    let b_value = underneath_processor.b;

                    // Changing a and b registers with the value from the underneath processor
                    let processor = self.get_mut_processor(x, y, z);
                    processor.a = a_value;
                    processor.b = b_value;
                }
            }
        }
    }

    fn step_2(&mut self) {
        // Looping through x
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::new();

        res.push_str("         A               B               C\n");

        // Printing from top to bottom
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
