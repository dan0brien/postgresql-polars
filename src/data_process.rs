use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use polars::prelude::*;

pub async fn request_handler() -> String {
    let database_url = "postgres://postgres:postgres@localhost:5432/Adventureworks";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to create pool");

    let pg_result = sqlx::query("SELECT * FROM person.address")
        .fetch_all(&pool)
        .await;
    match pg_result {
        Ok(rows) => {
            let (address_ids, address_line1s, postalcodes): (Vec<i32>, Vec<String>, Vec<String>) = rows.iter().fold(
                (Vec::with_capacity(rows.len()), Vec::with_capacity(rows.len()), Vec::with_capacity(rows.len())),
                |(mut address_ids, mut address_lines, mut postalcodes), row| {
                    address_ids.push(row.get("addressid"));
                    address_lines.push(row.get("addressline1"));
                    postalcodes.push(row.get("postalcode"));
                    (address_ids, address_lines, postalcodes)
                },
            );

            let df = DataFrame::new(vec![
                Series::new("address_id".into(), address_ids).into(),
                Series::new("address_line1".into(), address_line1s).into(),
                Series::new("postalcode".into(), postalcodes).into(),
            ]);

            match df {
                Ok(df) => {
                    let lazy_df = df.clone().lazy();
                    // Change something in LazyFrame
                    let result = lazy_df.select([
                        col("address_id").alias("id"),
                        col("address_line1").alias("address"),
                        col("postalcode").alias("postcode")
                    ]);
                    match result.collect() {
                        Ok(collected_df) => {
                            println!("{}", collected_df);
                        }
                        Err(e) => {
                            println!("Error collecting LazyFrame: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error creating DataFrame: {:?}", e);
                }
            }
        }
        Err(e) => println!("Error querying database: {}", e),
    }
    "Processed Request\n".to_string()
}
