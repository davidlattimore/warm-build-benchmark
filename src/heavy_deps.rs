use anyhow::Result;
use polars::io::mmap::MmapBytesReader;
use polars::prelude::*;
use std::path::Path;

pub(crate) fn do_stuff() -> Result<()> {
    let path = Path::new("foo.tbl");

    // This code isn't really intended to run, just to make use of various features of the libraries
    // that we depend on so that the compiler can't just get rid of stuff. We're trying to create
    // the scenario where we're editing a trivial bit of code (above) that is part of a larger
    // codebase with heavy dependencies.
    if path.exists() {
        let file = std::fs::File::open(path).unwrap();
        let file = Box::new(file) as Box<dyn MmapBytesReader>;
        let mut reader = CsvReader::new(file).batched_mmap(None)?;
        reader.next_batches(1)?;

        let mut df = DataFrame::default();

        let f = std::fs::File::create("out1.parquet").unwrap();
        ParquetWriter::new(f)
            .with_statistics(true)
            .finish(&mut df)?;

        let f = std::fs::File::create("out2.ipc").unwrap();
        IpcWriter::new(f).finish(&mut df)?;

        let _ = PrimitiveChunkedBuilder::<UInt16Type>::new("foo", 10);

        let d = polars::time::Duration::new(10);
        println!("{d:?}");
    }
    Ok(())
}
