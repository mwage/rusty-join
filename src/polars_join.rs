use std::io::stdout;

use polars::prelude::*;

pub fn custom_polars(args: Vec<String>) {
    let mut df1 = CsvReadOptions::default()
        .with_has_header(false)
        .try_into_reader_with_file_path(Some((&args[1]).into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut df2 = CsvReadOptions::default()
        .with_has_header(false)
        .try_into_reader_with_file_path(Some((&args[2]).into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut df3 = CsvReadOptions::default()
        .with_has_header(false)
        .try_into_reader_with_file_path(Some((&args[3]).into()))
        .unwrap()
        .finish()
        .unwrap();

    let mut df4 = CsvReadOptions::default()
        .with_has_header(false)
        .try_into_reader_with_file_path(Some((&args[4]).into()))
        .unwrap()
        .finish()
        .unwrap();

    // Rename columns to avoid conflicts
    let df1 = df1
        .rename("column_1", PlSmallStr::from("f1_col1"))
        .unwrap()
        .rename("column_2", PlSmallStr::from("f1_col2"))
        .unwrap();
    let df2 = df2
        .rename("column_1", PlSmallStr::from("f2_col1"))
        .unwrap()
        .rename("column_2", PlSmallStr::from("f2_col2"))
        .unwrap();
    let df3 = df3
        .rename("column_1", PlSmallStr::from("f3_col1"))
        .unwrap()
        .rename("column_2", PlSmallStr::from("f3_col2"))
        .unwrap();
    let df4 = df4
        .rename("column_1", PlSmallStr::from("f4_col1"))
        .unwrap()
        .rename("column_2", PlSmallStr::from("f4_col2"))
        .unwrap();

    let join1_2 = df1
        .join(
            &df2,
            ["f1_col1"],
            ["f2_col1"],
            JoinArgs::new(JoinType::Inner),
        )
        .unwrap();
    let join1_2_3 = join1_2
        .join(
            &df3,
            ["f1_col1"],
            ["f3_col1"],
            JoinArgs::new(JoinType::Inner),
        )
        .unwrap();

    let final_join = df4
        .join(
            &join1_2_3,
            ["f4_col1"],
            ["f3_col2"],
            JoinArgs::new(JoinType::Inner),
        )
        .unwrap();

    let mut result = final_join
        .select([
            "f4_col1", // file4.field1
            "f1_col1", // file1.field1
            "f1_col2", // file1.field2
            "f2_col2", // file2.field2
            "f4_col2", // file4.field2
        ])
        .unwrap();

    CsvWriter::new(stdout())
        .include_header(false)
        .with_separator(b',')
        .finish(&mut result);
}
