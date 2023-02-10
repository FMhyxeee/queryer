fn main() {
    tracing_subscriber::fmt::init();
    let sql = "WITH
    Mytable AS (
        SELECT a, b, c
        FROM data_source
        WHERE a = 1
    ),
    Mytable2 AS (
        SELECT e, f, g
        FROM data_source
        WHERE a = 2
    )
    SELECT a a1, b, 123, myfunc(b), *
    FROM data_source t1
    LEFT JOIN other_table t2 ON data_source.id = other_table.id
    WHERE t1.a = t2.a AND b = 2
    ORDER BY a DESC, b ASC
    LIMIT 10;";

    let ast =
        sqlparser::parser::Parser::parse_sql(&sqlparser::dialect::GenericDialect {}, sql).unwrap();

    for s in ast {
        if let sqlparser::ast::Statement::Query(q) = s {
            // println!("{:#?}", q);
            println!("{:#?}", q.body);
        }
    }
}
