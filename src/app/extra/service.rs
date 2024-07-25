mod dto;

pub use dto::{CreateInput, DeleteInput, UpdateInput};

use domain::extra;

#[derive(Clone, Debug)]
pub struct ExtraService<T> {
    extras: T,
}

impl<T: extra::Repository> ExtraService<T> {
    pub fn new(extras: T) -> Self {
        Self { extras }
    }
}

impl<T: extra::Repository> ExtraService<T> {
    pub async fn all(&self) -> Result<Vec<extra::Extra>, extra::Error> {
        self.extras.all().await
    }

    pub async fn create(&mut self, input: CreateInput) -> Result<extra::Extra, extra::Error> {
        let extra = extra::Extra::new(input.name, input.price);
        self.extras.create(&extra).await?;

        Ok(extra)
    }

    pub async fn delete(&mut self, input: DeleteInput) -> Result<extra::Extra, extra::Error> {
        self.extras.delete(input.id).await
    }

    pub async fn update(&mut self, input: UpdateInput) -> Result<extra::Extra, extra::Error> {
        let mut extra = self.extras.find(input.id).await?;
        extra.name = input.name;
        extra.price = input.price;
        extra.metadata.update();

        self.extras.update(&extra).await?;

        Ok(extra)
    }
}
