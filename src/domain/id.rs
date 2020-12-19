use uuid::Uuid;

pub type Id = Uuid;

pub fn generate() -> Id {
    Uuid::new_v4()
}
