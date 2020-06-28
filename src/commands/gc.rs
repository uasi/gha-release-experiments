use anyhow::Result;

use crate::common::count;
use crate::config;
use crate::database::Connection;

pub fn run() -> Result<()> {
    let db = Connection::open(config::database_path())?;
    db.create()?;

    let n = db.prune_tweets()?;
    println!("Pruned {}.", count(n, "tweet"));

    if n > 0 {
        db.vacuum()?;
        println!("Vacuumed database.");
    }

    Ok(())
}
