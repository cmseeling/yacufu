use color_eyre::Result;

pub(crate) mod apt;

pub trait Repository {
    fn check_for_repository(&self) -> bool;

    fn load_repository_list(&mut self) -> Result<i32>;

    fn get_repository_list(&self) -> Vec<String>;
}
