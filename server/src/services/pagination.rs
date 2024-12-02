// src/services/pagination_service.rs
pub fn paginate(page: Option<i64>, limit: Option<i64>) -> (i64, i64) {
    let page = page.unwrap_or(1);
    let limit = limit.unwrap_or(100);
    let offset = (page - 1) * limit;
    (limit, offset)
}
