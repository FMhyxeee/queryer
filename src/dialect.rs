use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct HyxDialect;

// 添加自己的方言支持
// 主要是对于网页的解析
impl Dialect for HyxDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_lowercase()
            || ch.is_ascii_uppercase()
            || ch.is_ascii_digit()
            || [':', '/', '?', '&', '=', '-', '_', '.'].contains(&ch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlparser::parser::Parser;

    pub fn example_sql() -> String {
        let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

        let sql = format!(
            "SELECT location name, total_cases, new_cases, total_deaths, new_deaths \
            FROM {url} where new_death >= 0 ORDER BY new_cases DESC LIMIT 6"
        );

        sql
    }

    #[test]
    fn it_works() {
        assert!(Parser::parse_sql(&HyxDialect, &example_sql()).is_ok());
    }
}
