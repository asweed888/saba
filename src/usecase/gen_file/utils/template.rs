use crate::domain::model::template::entity::Template;
use crate::domain::repository::template::TemplateRepositoryAct;

pub fn create<R>(repo: &R)
where
    R: TemplateRepositoryAct
{}

pub fn get_all<R>(repo: &R)
where
    R: TemplateRepositoryAct
{}

pub fn update<R>(repo: &R)
where
    R: TemplateRepositoryAct
{}

pub fn delete<R>(repo: &R)
where
    R: TemplateRepositoryAct
{}

