use crate::{solution::*, util::GenericResult};
use itertools::Itertools;

const WIDTH: i32 = 40;
const HEIGHT: i32 = 6;
const PIXEL_ON: char = '█';
const PIXEL_OFF: char = ' ';

#[derive(Default)]
pub struct Day10;
impl Solution for Day10 {
    fn info(&self) -> SolutionInfo {
        Title::new(2022, 10, "Cathode-Ray Tube")
    }

    fn part1(&mut self, ctx: &Context) -> SolutionResult {
        let (signal_strength, _) = execute_program(ctx)?;
        Ok(signal_strength.to_string())
    }

    fn part2(&mut self, ctx: &Context) -> SolutionResult {
        let (_, screen) = execute_program(ctx)?;
        let screen = screen
            .chunks(WIDTH as usize)
            .map(|x| x.iter().join(""))
            .join("\n");

        Ok(screen)
    }
}

fn execute_program(ctx: &Context) -> GenericResult<(i32, Vec<char>)> {
    let mut screen: Vec<char> = Vec::new();
    (0..WIDTH * HEIGHT).for_each(|_| screen.push(PIXEL_OFF));

    let mut x = 1;
    let mut cycle_count = 0;
    let mut signal_strength = 0;

    for line in ctx.input().lines() {
        match line.split(" ").collect_vec()[..] {
            ["noop"] => tick(&mut cycle_count, &mut signal_strength, &mut screen, x),
            ["addx", num_str] => {
                tick(&mut cycle_count, &mut signal_strength, &mut screen, x);
                tick(&mut cycle_count, &mut signal_strength, &mut screen, x);
                x += num_str.parse::<i32>().map_err(|_| "invalid addx value")?;
            }
            _ => Err("invalid instruction")?,
        }
    }

    Ok((signal_strength, screen))
}

fn tick(cycle_count: &mut i32, signal_strength: &mut i32, screen: &mut Vec<char>, x: i32) {
    let pos = *cycle_count % screen.len() as i32;
    screen[pos as usize] = match pos % WIDTH - x {
        -1..=1 => PIXEL_ON,
        _ => PIXEL_OFF,
    };

    *cycle_count += 1;
    if *cycle_count >= 20 && *cycle_count % 40 == 20 {
        *signal_strength += *cycle_count * x;
    }
}
