use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PetRequestDTO{
    name: String,
    category: CategoryRequestDTO,
    tags: Vec<TagRequestDTO>,
    status: Status,
}

#[derive(Debug, Deserialize)]
pub struct CategoryRequestDTO {
    name: String,
}

#[derive(Debug, Deserialize)]
pub struct TagRequestDTO {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct PetResponseDTO{
    id: u64,
    name: String,
    category: CategoryResponseDTO,
    tags: Vec<TagResponseDTO>,
    status: Status,
}

#[derive(Debug, Serialize)]
pub struct CategoryResponseDTO {
    id: u64,
    name: String,
}

#[derive(Debug, Serialize)]
pub struct TagResponseDTO {
    id: u64,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Available,
    Pending,
    Sold,
}