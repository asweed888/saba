use crate::domain::repository;
use crate::usecase;
use crate::presentation;

pub struct App {}

pub struct DIContainer {}

impl DIContainer {
    pub fn new() -> Self {
        Self{}
    }
    pub fn new_app(&self) -> App {
        App{}
    }
}
