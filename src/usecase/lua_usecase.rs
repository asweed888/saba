use crate::domain::model::lua_usecase::entity::LuaUsecase;
use crate::domain::repository::lua_usecase::LuaUsecaseRepository;

pub fn create<R>(repo: &R)
where
    R: LuaUsecaseRepository
{}

pub fn get_all<R>(repo: &R)
where
    R: LuaUsecaseRepository
{}

pub fn update<R>(repo: &R)
where
    R: LuaUsecaseRepository
{}

pub fn delete<R>(repo: &R)
where
    R: LuaUsecaseRepository
{}

