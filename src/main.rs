use anyhow::Result;
use ccsv::CCSV;

fn main() -> Result<()> {
    CCSV::new().run()
}
