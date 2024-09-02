use crate::domain::model::html_usecase::entity::HtmlUsecase;
use crate::domain::repository::html_usecase::HtmlUsecaseRepository;

pub fn create<R>(repo: &R)
where
    R: HtmlUsecaseRepository
{}

pub fn get_all<R>(repo: &R)
where
    R: HtmlUsecaseRepository
{}

pub fn update<R>(repo: &R)
where
    R: HtmlUsecaseRepository
{}

pub fn delete<R>(repo: &R)
where
    R: HtmlUsecaseRepository
{}

