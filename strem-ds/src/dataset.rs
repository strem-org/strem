use std::error::Error;

pub mod lyft;

pub trait DataSet {
    // Create a new instance of a [`DataSet`] with an associated path.
    //
    // This path should point to the root directory of the dataset (i.e., it is
    // the top-most level from which all other data/folders reside).
    //
    // It should be the responsibility of the implementing dataset to navigate
    // the structure underneath this folder.
    fn load(&mut self) -> Result<(), Box<dyn Error>>;

    // Retrieve the next sample from this dataset.
    //
    // This should be scene as one continuous stream of data as the current
    // interface of a stream of perception data is seen as individual adjacent
    // samples.
    //
    // This should be a lazy function (i.e., it should not load ALL data into one
    // structure and send samples; instead, it should load only the minimal set
    // of data required to construct a sample).
    fn export(&self) -> Result<(), Box<dyn Error>>;
}
