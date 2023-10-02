use std::{env, path::PathBuf};
use strem_ds::dataset::lyft::DataSet;
use strem_ds::dataset::DataSet as DataSetTrait;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = PathBuf::from(&args[1]);

    let mut dataset = DataSet::new(&path);
    dataset.load().expect("loading error...");

    dataset.export().expect("exporting error...");
}
