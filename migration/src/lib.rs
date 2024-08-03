use sea_orm_migartion::prelude::*;

#[async_std::main]
async fn main() {
    cli::run_cli(migartion::Migrator).await;
}