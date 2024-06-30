mod queries;

use sqlx::PgPool;

use domain::catalog;
use domain::product;

use crate::infra::product::ProductModel;

#[derive(Clone, Debug)]
pub struct PgProducts {
    pool: PgPool,
}

impl PgProducts {
    const PK: &'static str = "pk_product";
    const AK_NAME: &'static str = "ak_product_name";
    const FK_CATALOG_ID: &'static str = "fk_product_catalog_id";

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn is_pk_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|db_err| db_err.constraint() == Some(Self::PK))
    }

    fn is_ak_name_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|db_err| db_err.constraint() == Some(Self::AK_NAME))
    }

    fn is_fk_catalog_id_error(err: &sqlx::Error) -> bool {
        err.as_database_error()
            .is_some_and(|db_err| db_err.constraint() == Some(Self::FK_CATALOG_ID))
    }
}

impl product::Repository for PgProducts {
    async fn create(&mut self, product: &product::Product) -> Result<(), product::Error> {
        let mut trx = self.pool.begin().await.map_err(product::Error::any)?;

        queries::CreateQuery { product }
            .exec(trx.as_mut())
            .await
            .map_err(|err| {
                if Self::is_pk_error(&err) {
                    product::Error::id_conflict(product.id())
                } else if Self::is_ak_name_error(&err) {
                    product::Error::name_conflict(product.name().clone())
                } else if Self::is_fk_catalog_id_error(&err) {
                    product::Error::catalog_not_found(product.catalog_id())
                } else {
                    product::Error::any(err)
                }
            })?;

        queries::BindExtrasQuery {
            id: product.id(),
            extras: product.extras().as_slice(),
        }
        .exec(trx.as_mut())
        .await
        .map_err(product::Error::any)?;

        trx.commit().await.map_err(product::Error::any)
    }

    async fn delete(
        &mut self,
        id: product::Id,
        catalog_id: catalog::Id,
    ) -> Result<product::Product, product::Error> {
        queries::DeleteQuery { id, catalog_id }
            .exec(&self.pool)
            .await
            .map_err(|err| match &err {
                sqlx::Error::RowNotFound => product::Error::id_not_found(id, catalog_id),
                _ => product::Error::any(err),
            })
            .and_then(ProductModel::try_into_entity)
    }

    async fn find(
        &self,
        id: product::Id,
        catalog_id: catalog::Id,
    ) -> Result<product::Product, product::Error> {
        queries::FindQuery { id, catalog_id }
            .exec(&self.pool)
            .await
            .map_err(|err| match &err {
                sqlx::Error::RowNotFound => product::Error::id_not_found(id, catalog_id),
                _ => product::Error::any(err),
            })
            .and_then(ProductModel::try_into_entity)
    }

    async fn update(&mut self, product: &product::Product) -> Result<(), product::Error> {
        let mut trx = self.pool.begin().await.map_err(product::Error::any)?;

        let updated_count = queries::UpdateQuery { product }
            .exec(trx.as_mut())
            .await
            .map_err(|err| {
                if Self::is_ak_name_error(&err) {
                    product::Error::name_conflict(product.name().clone())
                } else {
                    product::Error::any(err)
                }
            })?;

        if updated_count == 0 {
            let (id, catalog_id) = (product.id(), product.catalog_id());
            return Err(product::Error::id_not_found(id, catalog_id));
        }

        queries::BindExtrasQuery {
            id: product.id(),
            extras: product.extras().as_slice(),
        }
        .exec(trx.as_mut())
        .await
        .map_err(product::Error::any)?;

        queries::UnbindExtrasQuery {
            id: product.id(),
            extras: product.extras().as_slice(),
        }
        .exec(trx.as_mut())
        .await
        .map_err(product::Error::any)?;

        trx.commit().await.map_err(product::Error::any)
    }
}
