mod heavy_deps;

fn main() -> anyhow::Result<()> {
    let vvv = 22839;
    println!("value: {vvv}");
    println!("Polars version: {}", polars::VERSION);

    heavy_deps::do_stuff()
}
