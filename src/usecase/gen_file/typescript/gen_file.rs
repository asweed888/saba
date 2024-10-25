use crate::domain::model::gen_file::entity::GenFile;
use crate::domain::repository::gen_file::GenFileRepositoryAct;

pub fn create<R>(repo: &R)
where
    R: GenFileRepositoryAct
{}

pub fn get_all<R>(repo: &R)
where
    R: GenFileRepositoryAct
{}

pub fn update<R>(repo: &R)
where
    R: GenFileRepositoryAct
{}

pub fn delete<R>(repo: &R)
where
    R: GenFileRepositoryAct
{}

