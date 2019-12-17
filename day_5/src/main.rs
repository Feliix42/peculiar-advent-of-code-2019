use intcode::IntProgram;

fn main() {
    let program = IntProgram::from_file("../input/day_5.txt");

    let _ = program.run();
}
