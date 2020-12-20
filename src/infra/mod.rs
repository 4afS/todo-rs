pub mod route;
pub mod db {
    pub mod mock;

    static mut DB: mock::Db = mock::Db;
}
