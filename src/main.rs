mod heavy_deps;

fn main() -> anyhow::Result<()> {
    let vvv = include_str!("number.txt").trim().parse::<u64>().unwrap();
    println!("value: {vvv}");
    println!("Polars version: {}", polars::VERSION);

    heavy_deps::do_stuff()
}
