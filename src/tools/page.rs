use rocket_contrib::Value;

#[derive(Serialize, Deserialize)]
pub struct Page {
    pub data: Value,
    pub meta: Meta
}

impl Page {
    pub fn new(_data: Value, _page: i32, _offset: i32) -> Self {
        Self {
            data: _data,
            meta: Meta::new(_page, _offset)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub page: i32,
    pub offset: i32,
    pub previous_page: String,
    pub current_page: String,
    pub next_page: String
}

impl Meta {
    pub fn new(_page: i32, _offset: i32) -> Self {
        Self {
            page: _page,
            offset: _offset,
            previous_page:
                format!("localhost:8000/users?page={}&offset={}", _page - 1, _offset - 1),
            current_page:
                format!("localhost:8000/users?page={}&offset={}", _page, _offset),
            next_page:
                format!("localhost:8000/users?page={}&offset={}", _page + 1, _offset + 1)
        }
    }
}
