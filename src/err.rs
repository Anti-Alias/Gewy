use derive_more::{Display, Error};

pub type Result<T> = std::result::Result<T, GuiError>;

#[derive(Error, Clone, Eq, PartialEq, Debug, Display)]
pub enum GuiError {
    #[display(fmt = "Node not found")]
    NodeNotFound,
    #[display(fmt = "Parent not found")]
    ParentNodeNotFound
}