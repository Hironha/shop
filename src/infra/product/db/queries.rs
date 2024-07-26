use sqlx::PgExecutor;

use domain::catalog;
use domain::extra;
use domain::product;

use crate::infra::product::ProductModel;

// TODO: improve code organization and reduce memory memory allocation

#[derive(Clone, Debug)]
pub(super) struct BindExtrasQuery<'a> {
    pub(super) id: product::Id,
    pub(super) extras: &'a [extra::Extra],
}

impl<'a> BindExtrasQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let extras_ids = self
            .extras
            .iter()
            .map(|extra| extra.id().uuid())
            .collect::<Vec<_>>();

        let product_ids = vec![self.id.uuid(); extras_ids.len()];

        let bind_extras_sql = include_str!("./sql/extras_bind.sql");
        sqlx::query(bind_extras_sql)
            .bind(product_ids)
            .bind(extras_ids)
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct CreateQuery<'a> {
    pub(super) product: &'a product::Product,
}

impl<'a> CreateQuery<'a> {
    pub(super) async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let sql = include_str!("./sql/create.sql");
        sqlx::query(sql)
            .bind(self.product.id().uuid())
            .bind(self.product.catalog_id().uuid())
            .bind(self.product.name.as_str())
            .bind(self.product.price.decimal())
            .bind(self.product.kind.as_str())
            .bind(self.product.metadata.created_at())
            .bind(self.product.metadata.updated_at())
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct DeleteQuery {
    pub(super) id: product::Id,
    pub(super) catalog_id: catalog::Id,
}

impl DeleteQuery {
    pub async fn exec(self, exec: impl PgExecutor<'_>) -> Result<ProductModel, sqlx::Error> {
        let sql = include_str!("./sql/delete.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .bind(self.catalog_id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct FindQuery {
    pub(super) id: product::Id,
    pub(super) catalog_id: catalog::Id,
}

impl FindQuery {
    pub async fn exec(self, exec: impl PgExecutor<'_>) -> Result<ProductModel, sqlx::Error> {
        let sql = include_str!("./sql/find.sql");
        sqlx::query_as(sql)
            .bind(self.id.uuid())
            .bind(self.catalog_id.uuid())
            .fetch_one(exec)
            .await
    }
}

#[derive(Clone, Debug)]
pub(super) struct UnbindExtrasQuery<'a> {
    pub(super) id: product::Id,
    pub(super) extras: &'a [extra::Extra],
}

impl<'a> UnbindExtrasQuery<'a> {
    pub async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let extras_ids = self
            .extras
            .iter()
            .map(|extra| extra.id().uuid())
            .collect::<Vec<_>>();

        let unbind_extras_sql = include_str!("./sql/extras_unbind.sql");
        sqlx::query(unbind_extras_sql)
            .bind(self.id.uuid())
            .bind(extras_ids)
            .execute(exec)
            .await?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
pub(super) struct UpdateQuery<'a> {
    pub(super) product: &'a product::Product,
}

impl<'a> UpdateQuery<'a> {
    pub async fn exec(self, exec: impl PgExecutor<'a>) -> Result<(), sqlx::Error> {
        let update_sql = include_str!("./sql/update.sql");
        let result = sqlx::query(update_sql)
            .bind(self.product.name.as_str())
            .bind(self.product.price.decimal())
            .bind(self.product.kind.as_str())
            .bind(self.product.metadata.updated_at())
            .bind(self.product.id().uuid())
            .bind(self.product.catalog_id().uuid())
            .execute(exec)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use domain::metadata;

    use sqlx::PgPool;

    use super::*;

    #[sqlx::test(fixtures("seed"))]
    async fn bind_extras_query_works(pool: PgPool) {
        let product_id = product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
            .expect("Valid product id from fixtures");

        let result = BindExtrasQuery {
            id: product_id,
            extras: &[get_mocked_hot_sauce(), get_mocked_cheddar()],
        }
        .exec(&pool)
        .await;

        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn create_query_works(pool: PgPool) {
        let extras = vec![get_mocked_cheddar(), get_mocked_hot_sauce()];

        let product = product::Product::new(
            catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            product::Name::new("Cheese Bacon").expect("Valid product name"),
            product::Price::from_cents(2450),
            product::Kind::Burger,
            product::Extras::new(extras).expect("Valid product extras"),
        );

        let result = CreateQuery { product: &product }.exec(&pool).await;
        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn delete_query_works(pool: PgPool) {
        let id = product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
            .expect("Valid product id from fixtures");

        let catalog_id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let deleted = DeleteQuery { id, catalog_id }.exec(&pool).await;
        assert!(deleted.is_ok());
        assert_eq!(deleted.unwrap().id, id.uuid());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn find_query_works(pool: PgPool) {
        let id = product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
            .expect("Valid product id from fixtures");

        let catalog_id = catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
            .expect("Valid catalog id from fixtures");

        let deleted = FindQuery { id, catalog_id }.exec(&pool).await;
        assert!(deleted.is_ok());
        assert_eq!(deleted.unwrap().id, id.uuid());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn unbind_extras_query_works(pool: PgPool) {
        let id = product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
            .expect("Valid product id from fixtures");

        let result = UnbindExtrasQuery {
            id,
            extras: &[get_mocked_cheddar()],
        }
        .exec(&pool)
        .await;

        assert!(result.is_ok());
    }

    #[sqlx::test(fixtures("seed"))]
    async fn update_query_works(pool: PgPool) {
        let product = product::Product::config(product::Config {
            id: product::Id::parse_str("0190ec14-0af8-71d1-9554-f1e5249ae3a2")
                .expect("Valid product id from fixtures"),
            catalog_id: catalog::Id::parse_str("0190ec30-286b-7211-aadb-003fc0449734")
                .expect("Valid catalog id from fixtures"),
            name: product::Name::new("Cheese Bacon").expect("Valid product name"),
            price: product::Price::from_cents(2540),
            kind: product::Kind::Burger,
            extras: Some(
                product::Extras::new(vec![get_mocked_cheddar()]).expect("Valid product extras"),
            ),
            metadata: metadata::Metadata::new(),
        });

        let update_result = UpdateQuery { product: &product }.exec(&pool).await;
        assert!(update_result.is_ok());

        let updated_model = FindQuery {
            id: product.id(),
            catalog_id: product.catalog_id(),
        }
        .exec(&pool)
        .await
        .expect("Update product model");

        assert_eq!(updated_model.id, product.id().uuid());
        assert_eq!(updated_model.catalog_id, product.catalog_id().uuid());
        assert_eq!(updated_model.name.as_str(), product.name.as_str());
        assert_eq!(updated_model.price, product.price.decimal());
        assert_eq!(updated_model.kind.as_str(), product.kind.as_str());
    }

    fn get_mocked_cheddar() -> extra::Extra {
        extra::Extra::config(extra::Config {
            id: extra::Id::parse_str("0190ec13-15cc-7f53-bc0f-d60f0beea824")
                .expect("Cheedar id from seed fixtures"),
            name: extra::Name::new("Cheddar").expect("Valid extra name"),
            price: extra::Price::from_cents(200),
            metadata: metadata::Metadata::new(),
        })
    }

    fn get_mocked_hot_sauce() -> extra::Extra {
        extra::Extra::config(extra::Config {
            id: extra::Id::parse_str("0190ec10-4aa7-7552-ba8f-df997d9f8a8e")
                .expect("Hot sauce id from seed fixtures"),
            name: extra::Name::new("Hot Sauce").expect("Valid extra name"),
            price: extra::Price::from_cents(150),
            metadata: metadata::Metadata::new(),
        })
    }
}
