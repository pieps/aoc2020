use aoc2020;
use aoc2020::Day;
use gumdrop::Options;

#[derive(Options)]
struct Opts {
    #[options(help = "Which day to run.")]
    day: u32,

    #[options(help = "Which part to run.")]
    part: u8,
}

fn main() -> Result<(), String> {
    let opts = Opts::parse_args_default_or_exit();

    let day = aoc2020::get_day(opts.day)?;
    let part = match opts.part {
        1 => Day::solve1,
        2 => Day::solve2,
        _ => panic!("Must pick part 1 or 2."),
    };
    println!(
        "Day {}, part {}: {}",
        opts.day,
        opts.part,
        part(day.as_ref())
    );
    Ok(())
}
