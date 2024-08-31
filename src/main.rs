use std::io::stdin;

fn bf(program: &str) {
    let mut memory: [u8; 30000] = [0; 30000];
    let mut ptr = 0; // index in memory
    let mut i = 0; // index in program
    let mut l = Vec::new(); // loops

    let p: Vec<char> = program.chars().collect(); // program

    while i < p.len() {
        match p[i] {
            '+' => {
                memory[ptr] += 1;
            }
            '-' => {
                memory[ptr] -= 1;
            }
            '>' => {
                ptr += 1;
            }
            '<' => {
                ptr -= 1;
            }
            '[' => {
                if memory[ptr] == 0 {
                    let mut d = 1;
                    while d > 0 {
                        i += 1;
                        if p[i] == '[' {
                            d += 1;
                        }
                        if p[i] == ']' {
                            d -= 1;
                        }
                    }
                } else {
                    l.push(i);
                }
            }
            ']' => {
                if memory[ptr] != 0 {
                    i = *l.last().unwrap();
                } else {
                    l.pop();
                }
            }
            ',' => {
                let mut input = String::new();
                stdin()
                    .read_line(&mut input)
                    .expect("Failed to read from stdin");
                if let Some(ch) = input.chars().next() {
                    memory[ptr] = ch as u8;
                }
            }
            '.' => {
                print!("{}", memory[ptr] as char);
            }
            _ => {}
        }
        i += 1;
    }
}

fn main() {
    let program = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]
    >++++++++[<++++>-] <.>+++++++++++[<++++++++>-]<-.--------.+++
    .------.--------.[-]>++++++++[<++++>- ]<+.[-]++++++++++.";
    bf(program);
}
