use polars::df;
use polars::prelude::*;

fn main() -> PolarsResult<()> {
    // filter
    let df = df![
        "a" => [1, 2, 3],
        "b" => [None, Some("a"), Some("b")]
    ]?;

    let filtered = df.lazy().filter(col("a").gt(lit(2))).collect()?;

    println!("{}", filtered);

    // sort
    let df = df![
        "a" => [1, 2, 3],
        "b" => ["a", "a", "b"]
    ]?;
    // sort this DataFrame by multiple columns

    // ordering of the columns
    let descending = vec![true, false];

    let sorted = df
        .lazy()
        .sort_by_exprs(vec![col("b"), col("a")], descending, false)
        .collect()?;

    println!("{}", sorted);
    Ok(())
}
