
pub fn add_condition<T: std::fmt::Display>(
    where_clauses: &mut Vec<String>,
    column: &str,
    value: &Option<T>,
    operator: &str,
) {
    if let Some(val) = value {
        where_clauses.push(format!("{} {} {}", column, operator, val));
    }
}
pub fn paginate(page: Option<i32>, limit: Option<i32>, count: Option<i32>) -> (i32, i32) {
    let default_per_page = 10;
    let max_total_count = 4000;
    let max_per_page = 100; 
    let per_page_limit = limit
        .unwrap_or(default_per_page)
        .max(1) // At least 1
        .min(max_per_page); 
    let total_count_limit = count
        .unwrap_or(max_total_count) 
        .max(1) // At least 1
        .min(max_total_count);

    let page_number = page.unwrap_or(1).max(1);

    let offset = (page_number - 1) * per_page_limit;

    let page_limit = per_page_limit.min(total_count_limit - ((page_number - 1) * per_page_limit));

    (page_limit.max(0), offset)
}
