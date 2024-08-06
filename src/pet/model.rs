pub struct Pet{
    id: u64,
    name: String,
    category: Category,
    tags: Vec<Tag>,
    status: Status,
}

pub struct Category {
    id: u64,
    name: String,
}

pub struct Tag {
    id: u64,
    name: String,
}

pub enum Status {
    Available,
    Pending,
    Sold,
}