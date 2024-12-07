use sqlx::{migrate::Migrator, Error};
use std::fs;
use std::path::Path;

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), Error> {
    let migrations_dir = Path::new("migrations");
    if !migrations_dir.exists() {
        println!("Migration directory does not exist.");
    } else {
        let migration_files = fs::read_dir(migrations_dir)?
            .filter_map(Result::ok)
            .collect::<Vec<_>>();
        println!("Migration files found: {:?}", migration_files);
    }
    println!("Runnning migrations");
    let migrator = Migrator::new(migrations_dir).await?;
    if let Err(e) = migrator.run(pool).await {
        eprintln!("Migration failed: {:?}", e);
        return Err(e.into());
    }

    println!("Migrations completed successfully.");
    Ok(())
}
