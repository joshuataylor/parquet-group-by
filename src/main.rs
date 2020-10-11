use datafusion::execution::context::ExecutionContext;
use datafusion::logical_plan::{col, count, min};

#[tokio::main]
async fn main() {
    sql_df().await;
}

async fn sql_df() {
    let sql = format!("select O_ORDERKEY from something group by O_ORDERKEY");

    let mut ctx = ExecutionContext::new();
    ctx.register_parquet("something", "demo.parquet");

    ctx.sql(sql.as_str())
        .unwrap()
        .collect()
        .await
        .unwrap();
}

async fn builder_df() {
    ExecutionContext::new()
        .read_parquet("demo.parquet")
        .unwrap()
        .aggregate(vec![col("O_ORDERKEY")], vec![count(col("O_ORDERKEY"))]).unwrap().collect().await;
}