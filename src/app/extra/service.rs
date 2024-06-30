mod dto;

pub use dto::{CreateInput, DeleteInput, UpdateInput};

use domain::extra::{Error, Id, Name, Price, Extra, Repository};

#[derive(Clone, Debug)]
pub struct ExtraService<T> {
    extras: T,
}

impl<T: Repository> ExtraService<T> {
    pub fn new(extras: T) -> Self {
        Self { extras }
    }

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

        let extra = self.extras.find(id).await?;
        let updated_extra = extra
            .into_setter()
            .name(name)
            .price(Price::from_cents(input.price))
            .commit();

        self.extras.update(&updated_extra).await?;

        Ok(updated_extra)
    }
}
