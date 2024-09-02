use crate::domain::model::bash_usecase::entity::BashUsecase;
use crate::domain::repository::bash_usecase::BashUsecaseRepository;

pub fn create<R>(repo: &R)
where
    R: BashUsecaseRepository
{}

pub fn get_all<R>(repo: &R)
where
    R: BashUsecaseRepository
{}

pub fn update<R>(repo: &R)
where
    R: BashUsecaseRepository
{}

pub fn delete<R>(repo: &R)
where
    R: BashUsecaseRepository
{}

