use crate::imports::*;

#[derive(Getters)]
pub struct Storage<T>
{
    contents: HashMap<String, T>
}

impl<T> Storage<T>
{

    pub fn init() -> Self
    {
        Storage {
            contents: HashMap::new()
        }
    }

    pub fn push(&mut self, identifier: &str, input: T)
    {
        self.contents.insert(identifier.to_owned(), input);
    }

    pub fn get(&self, identifier: &str) -> Option<&T>
    {
        self.contents.get(identifier)
    }

}
