mod queries;

use sqlx::PgPool;

use domain::catalog;
use domain::product;

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

        let create_query = queries::CreateQuery { product };
        create_query.exec(trx.as_mut()).await.map_err(|err| {
            if Self::is_pk_error(&err) {
                product::Error::id_conflict(product.id())
            } else if Self::is_ak_name_error(&err) {
                product::Error::name_conflict(product.name.clone())
            } else if Self::is_fk_catalog_id_error(&err) {
                product::Error::catalog_not_found(product.catalog_id())
            } else {
                product::Error::any(err)
            }
        })?;

        let bind_extras_query = queries::BindExtrasQuery {
            id: product.id(),
            extras: product.extras.as_slice(),
        };

        bind_extras_query
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
        let query = queries::DeleteQuery { id, catalog_id };
        let model = query.exec(&self.pool).await.map_err(|err| match &err {
            sqlx::Error::RowNotFound => product::Error::id_not_found(id, catalog_id),
            _ => product::Error::any(err),
        })?;

        model.try_into_entity().map_err(product::Error::any)
    }

    async fn find(
        &self,
        id: product::Id,
        catalog_id: catalog::Id,
    ) -> Result<product::Product, product::Error> {
        let query = queries::FindQuery { id, catalog_id };
        let model = query.exec(&self.pool).await.map_err(|err| match &err {
            sqlx::Error::RowNotFound => product::Error::id_not_found(id, catalog_id),
            _ => product::Error::any(err),
        })?;

        model.try_into_entity().map_err(product::Error::any)
    }

    async fn update(&mut self, product: &product::Product) -> Result<(), product::Error> {
        let mut trx = self.pool.begin().await.map_err(product::Error::any)?;

        let update_query = queries::UpdateQuery { product };
        update_query.exec(trx.as_mut()).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                product::Error::id_not_found(product.id(), product.catalog_id())
            } else if Self::is_ak_name_error(&err) {
                product::Error::name_conflict(product.name.clone())
            } else {
                product::Error::any(err)
            }
        })?;

        let bind_extras_query = queries::BindExtrasQuery {
            id: product.id(),
            extras: product.extras.as_slice(),
        };

        bind_extras_query
            .exec(trx.as_mut())
            .await
            .map_err(product::Error::any)?;

        let unbind_extras_query = queries::UnbindExtrasQuery {
            id: product.id(),
            extras: product.extras.as_slice(),
        };

        unbind_extras_query
            .exec(trx.as_mut())
            .await
            .map_err(product::Error::any)?;

        trx.commit().await.map_err(product::Error::any)
    }
}
