use z3::{ast::Ast as _, FuncDecl};

use crate::puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Combo {
    Literal(u8),
    Register(Register),
}

fn to_combo(operand: u8) -> Option<Combo> {
    match operand {
        0 | 1 | 2 | 3 => Some(Combo::Literal(operand)),
        4 => Some(Combo::Register(Register::A)),
        5 => Some(Combo::Register(Register::B)),
        6 => Some(Combo::Register(Register::C)),
        _ => None,
    }
}

fn combo_value(combo: Combo, registers: [i64; 3]) -> i64 {
    match combo {
        Combo::Literal(lit) => i64::from(lit),
        Combo::Register(register) => registers[register as usize] as i64,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Xdv(Register, Combo),
    Bxl(u8),
    Bst(Combo),
    Bxc,
}

fn decode_tape(tape: &[u8]) -> Option<(Vec<Instruction>, Register)> {
    match tape {
        [block @ .., 5, operand, 3, 0] => {
            let output_register = to_combo(*operand).and_then(|combo| match combo {
                Combo::Literal(_) => None,
                Combo::Register(reg) => Some(reg),
            })?;

            let mut instructions = Vec::with_capacity(block.len() / 2);

            for chunk in block.chunks_exact(2) {
                let instr = chunk[0];
                let operand = chunk[1];

                let instr = match instr {
                    0 => Instruction::Xdv(Register::A, to_combo(operand)?),
                    1 => Instruction::Bxl(operand),
                    2 => Instruction::Bst(to_combo(operand)?),
                    4 => Instruction::Bxc,
                    6 => Instruction::Xdv(Register::B, to_combo(operand)?),
                    7 => Instruction::Xdv(Register::C, to_combo(operand)?),

                    3 | 5 | _ => return None,
                };

                instructions.push(instr);
            }

            Some((instructions, output_register))
        }
        _ => None,
    }
}

fn execute_instr(registers: &mut [i64; 3], instr: Instruction) {
    match instr {
        Instruction::Xdv(register, combo) => {
            let combo = combo_value(combo, *registers);
            let denominator = 2_i64.pow(combo as u32);

            registers[register as usize] = registers[0] / denominator;
        }
        Instruction::Bxl(lit) => {
            registers[1] = registers[1] ^ lit as i64;
        }
        Instruction::Bst(combo) => {
            registers[1] = combo_value(combo, *registers) % 8;
        }
        Instruction::Bxc => {
            registers[1] = registers[1] ^ registers[2];
        }
    }
}

fn register_continuity<'ctx>(
    registers: &RegisterModel<'ctx>,
    register: Register,
    step: z3::ast::Int<'ctx>,
) -> z3::ast::Bool<'ctx> {
    let previous_step = step.clone() - 1_i64;
    let previous_register_value = registers.apply(register, previous_step);

    registers
        .apply(register, step)
        ._eq(&previous_register_value)
}

fn model_combo<'ctx>(
    ctx: &'ctx z3::Context,
    registers: &RegisterModel<'ctx>,
    step: z3::ast::Int<'ctx>,
    combo: Combo,
) -> z3::ast::BV<'ctx> {
    match combo {
        Combo::Literal(lit) => z3::ast::BV::from_i64(ctx, i64::from(lit), 64),
        Combo::Register(reg) => registers.apply(reg, step),
    }
}

fn model_instruction<'ctx>(
    ctx: &'ctx z3::Context,
    solver: &z3::Optimize<'ctx>,
    registers: &RegisterModel<'ctx>,
    instr: Instruction,
    step: z3::ast::Int<'ctx>,
) {
    let previous_step = step.clone() - 1_i64;

    match instr {
        Instruction::Bxc => {
            let equality = registers.apply(Register::B, step.clone())._eq(
                &(registers.apply(Register::B, previous_step.clone())
                    ^ (registers.apply(Register::C, previous_step.clone()))),
            );

            solver.assert(&equality);
            solver.assert(&register_continuity(registers, Register::A, step.clone()));
            solver.assert(&register_continuity(registers, Register::C, step.clone()));
        }
        Instruction::Bxl(lit) => {
            let equality = registers.apply(Register::B, step.clone())._eq(
                &(registers.apply(Register::B, previous_step)
                    ^ z3::ast::BV::from_i64(ctx, i64::from(lit), 64)),
            );

            solver.assert(&equality);
            solver.assert(&register_continuity(registers, Register::A, step.clone()));
            solver.assert(&register_continuity(registers, Register::C, step.clone()));
        }
        Instruction::Bst(combo) => {
            let equality = registers.apply(Register::B, step.clone())._eq(
                &(model_combo(ctx, registers, previous_step, combo)
                    .bvurem(&z3::ast::BV::from_i64(ctx, 8, 64))),
            );

            solver.assert(&equality);
            solver.assert(&register_continuity(registers, Register::A, step.clone()));
            solver.assert(&register_continuity(registers, Register::C, step.clone()));
        }
        Instruction::Xdv(assigned_reg, combo) => {
            let equality = registers.apply(assigned_reg, step.clone())._eq(
                &(registers
                    .apply(Register::A, previous_step.clone())
                    .bvlshr(&model_combo(ctx, registers, previous_step, combo))),
            );

            solver.assert(&equality);
            for other_reg in [Register::A, Register::B, Register::C] {
                if other_reg != assigned_reg {
                    solver.assert(&register_continuity(registers, other_reg, step.clone()));
                }
            }
        }
    }
}

struct RegisterModel<'ctx> {
    registers: [FuncDecl<'ctx>; 3],
}

impl<'ctx> RegisterModel<'ctx> {
    fn apply(&self, reg: Register, step: z3::ast::Int<'ctx>) -> z3::ast::BV<'ctx> {
        self.registers[reg as usize]
            .apply(&[&step])
            .as_bv()
            .unwrap()
    }
}

fn z3_model<'ctx>(
    ctx: &'ctx z3::Context,
    instructions: Vec<Instruction>,
    output_register: Register,
    desired_output: Vec<u8>,
) -> Option<i64> {
    let reg_a = z3::FuncDecl::new(
        &ctx,
        "register_A",
        &[&z3::Sort::int(&ctx)],
        &z3::Sort::bitvector(&ctx, 64),
    );

    let reg_b = z3::FuncDecl::new(
        &ctx,
        "register_b",
        &[&z3::Sort::int(&ctx)],
        &z3::Sort::bitvector(&ctx, 64),
    );

    let reg_c = z3::FuncDecl::new(
        &ctx,
        "register_c",
        &[&z3::Sort::int(&ctx)],
        &z3::Sort::bitvector(&ctx, 64),
    );

    let registers = RegisterModel {
        registers: [reg_a, reg_b, reg_c],
    };

    let solver = z3::Optimize::new(ctx);

    let step_zero = z3::ast::Int::from_i64(ctx, 0);
    let bv_zero = z3::ast::BV::from_i64(ctx, 0, 64);

    solver.assert(
        &registers
            .apply(Register::B, step_zero.clone())
            ._eq(&bv_zero),
    );
    solver.assert(
        &registers
            .apply(Register::C, step_zero.clone())
            ._eq(&bv_zero),
    );
    solver.minimize(&registers.apply(Register::A, step_zero.clone()));

    let mut step = step_zero.clone();

    for desired_output in desired_output.into_iter() {
        for instr in instructions.iter() {
            step = step.clone() + 1_i64;

            model_instruction(ctx, &solver, &registers, *instr, step.clone());
        }

        solver.assert(
            &registers
                .apply(output_register, step.clone())
                .bvurem(&z3::ast::BV::from_i64(ctx, 8, 64))
                ._eq(&z3::ast::BV::from_i64(ctx, i64::from(desired_output), 64)),
        )
    }

    // At the end, register A must equal 0 otherwise we can't exit the loop
    solver.assert(&registers.apply(Register::A, step_zero.clone()));

    match solver.check(&[]) {
        z3::SatResult::Unsat | z3::SatResult::Unknown => None,
        z3::SatResult::Sat => {
            let model = solver.get_model().unwrap();

            let answer = model
                .eval(
                    &registers
                        .apply(Register::A, z3::ast::Int::from_i64(ctx, 0))
                        .to_int(true),
                    true,
                )?
                .as_i64()?;

            Some(answer)
        }
    }
}

// We only handle machines whose program is of the form "while { some block;
// print() }", meaning the penultimate instruction is Out and the last
// instruction is Jnz to label 0. There must also be no other Out or Jnz
// instruction in the block
#[derive(Clone)]
struct Machine {
    registers: [i64; 3],
    block: Vec<Instruction>,
    output_of_interest: Register,
    target_output: Vec<u8>,
}

impl Machine {
    fn run(&mut self) -> Vec<u8> {
        let mut output = Vec::new();

        loop {
            for &instr in self.block.iter() {
                execute_instr(&mut self.registers, instr);
            }

            // Push the output
            output.push((self.registers[self.output_of_interest as usize] % 8) as u8);

            if self.registers[0] == 0 {
                break;
            }
        }

        output
    }
}

pub struct Day17 {
    initial_machine: Machine,
}

impl Puzzle for Day17 {
    type Output = String;

    fn parse(input: &str) -> Option<Self> {
        let mut lines = input.lines();

        let mut registers = [0; 3];

        for register in registers.iter_mut() {
            let line = lines.next()?;
            let mut words = line.split_whitespace();
            let _ = words.next()?; // Register
            let _ = words.next()?; // A: or B: or C:
            let value = words.next()?;

            *register = value.parse::<i64>().ok()?;
        }

        let _ = lines.next()?; // Skip an empty line
        let program_line = lines.next()?;

        let mut words = program_line.split_whitespace();
        let _ = words.next()?; // Program:
        let tape = words.next()?;

        let tape = tape
            .split(',')
            .map(|instr| instr.parse::<u8>().ok())
            .collect::<Option<Vec<_>>>()?;

        let (block, output_of_interest) = decode_tape(&tape)?;

        Some(Day17 {
            initial_machine: Machine {
                registers,
                block,
                output_of_interest,
                target_output: tape,
            },
        })
    }

    fn part1(mut self) -> Option<Self::Output> {
        let output = self.initial_machine.run();
        let string_list = output
            .into_iter()
            .map(|value| format!("{value}"))
            .collect::<Vec<_>>();

        Some(string_list.join(","))
    }

    fn part2(self) -> Option<Self::Output> {
        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);

        let answer = z3_model(
            &ctx,
            self.initial_machine.block,
            self.initial_machine.output_of_interest,
            self.initial_machine.target_output,
        )?;

        Some(format!("{answer}"))
    }
}
