fn main() {
    tracing_subscriber::fmt::init();
    let sql = "SELECT a a1, b, 123, myfunc(b), *
    FROM data_source t1
    LEFT JOIN other_table t2 ON data_source.id = other_table.id
    WHERE t1.a = t2.a AND b = 2
    ORDER BY a DESC, b ASC
    LIMIT 10";

    let ast =
        sqlparser::parser::Parser::parse_sql(&sqlparser::dialect::GenericDialect {}, sql).unwrap();

    println!("{ast:#?}");
}
