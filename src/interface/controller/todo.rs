use crate::domain::repository::Repository;
use crate::usecase::todo::Usecase;

pub struct Controller<T: Repository> {
    pub usecase: Usecase<T>
}

impl<T: Repository> Controller<T> {

}