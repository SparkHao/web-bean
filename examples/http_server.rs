use serde_json::Value;
use sqlx::{MySqlPool};
use hyper::StatusCode;
use web_bean::OrdDomain;


#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    // mysql://root:123456@localhost:3306/ord
    // let opts = MySqlConnectOptions::new().host("127.0.0.1:3306").username("root").password("123456");
    // let pool = MySqlPoolOptions::new().connect_with(opts).await?;
    let pool = MySqlPool::connect("mysql://root:123456@localhost:3306/ord").await?;
    // let row: (i64, ) = sqlx::query_as("select ?").bind(150_i64).fetch_one(&pool).await?;
    // println!("row: {}", row.0);

    // let result = sqlx::query("select * from ord_domain").fetch_all(&pool)
    //     .await?;
    
    // println!("size: {}", result.len());
    // for row in result {
    //     println!("value: {:?}", &row);
        
    // }

    // sqlx::query("select * from ord_domain").fetch_all(&pool)
    //     .await.map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))
    //     .and_then(|p: Vec<OrdDomain>| {
    //         p.into_iter().map(|row| {
    //             println!("value: {:?}", row);
    //             p
    //         }).collect::<Result<Vec<OrdDomain>, _>>()
    //     }).map(axum::Json);

    Ok(())
}

