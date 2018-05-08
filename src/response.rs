#[derive(PartialEq, Debug, Clone)]
pub enum Status {
    Ok,
    Reserved,
    Inserted,
    Using,
    Deleted,
    Watching,
    NotIgnored,
}

#[derive(Clone)]
pub struct Response {
    pub status: Status,
    pub data: String,
}
