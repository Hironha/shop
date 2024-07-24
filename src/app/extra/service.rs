mod dto;

pub use dto::{CreateInput, DeleteInput, UpdateInput};

use domain::extra::{Error, Extra, Id, Name, Price, Repository};

#[derive(Clone, Debug)]
pub struct ExtraService<T> {
    extras: T,
}

impl<T: Repository> ExtraService<T> {
    pub fn new(extras: T) -> Self {
        Self { extras }
    }
}

impl<T: Repository> ExtraService<T> {
    pub async fn all(&self) -> Result<Vec<Extra>, Error> {
        self.extras.all().await
    }

    pub async fn create(&mut self, input: CreateInput) -> Result<Extra, Error> {
        let name = Name::new(input.name)?;
        let extra = Extra::new(name, Price::from_cents(input.price));

        self.extras.create(&extra).await?;

        Ok(extra)
    }

    pub async fn delete(&mut self, input: DeleteInput) -> Result<Extra, Error> {
        let id = Id::parse_str(&input.id)?;
        self.extras.delete(id).await
    }

    pub async fn update(&mut self, input: UpdateInput) -> Result<Extra, Error> {
        let id = Id::parse_str(&input.id)?;
        let name = Name::new(input.name)?;

        let mut extra = self.extras.find(id).await?;
        extra.name = name;
        extra.price = Price::from_cents(input.price);
        extra.metadata.update();

        self.extras.update(&extra).await?;

        Ok(extra)
    }
}
