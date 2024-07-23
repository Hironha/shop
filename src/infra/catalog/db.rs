mod queries;

use sqlx::PgPool;

use domain::catalog;

use super::CatalogWithProductsModel;

#[derive(Clone, Debug)]
pub struct PgCatalogs {
    pool: PgPool,
}

impl PgCatalogs {
    const PK: &'static str = "pk_catalog";
    const AK_NAME: &'static str = "ak_catalog_name";

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
}

impl catalog::Repository for PgCatalogs {
    async fn create(&mut self, catalog: &catalog::Catalog) -> Result<(), catalog::Error> {
        let query = queries::CreateQuery { catalog };
        query.exec(&self.pool).await.map_err(|err| {
            if Self::is_pk_error(&err) {
                catalog::Error::id_conflict(catalog.id())
            } else if Self::is_ak_name_error(&err) {
                let name = catalog.name.clone();
                catalog::Error::name_conflict(name)
            } else {
                catalog::Error::any(err)
            }
        })?;

        Ok(())
    }

    async fn delete(&self, id: catalog::Id) -> Result<catalog::CatalogProducts, catalog::Error> {
        let query = queries::DeleteQuery { id };
        let model = query.exec(&self.pool).await.map_err(|err| match err {
            sqlx::Error::RowNotFound => catalog::Error::id_not_found(id),
            _ => catalog::Error::any(err),
        })?;

        model.try_into_entity().map_err(catalog::Error::any)
    }

    async fn find(&self, id: catalog::Id) -> Result<catalog::CatalogProducts, catalog::Error> {
        let query = queries::FindQuery { id };
        let model = query.exec(&self.pool).await.map_err(|err| match err {
            sqlx::Error::RowNotFound => catalog::Error::id_not_found(id),
            _ => catalog::Error::any(err),
        })?;

        model.try_into_entity().map_err(catalog::Error::any)
    }

    async fn list(&self, query: catalog::ListQuery) -> Result<catalog::Pagination, catalog::Error> {
        let count = queries::CountQuery
            .exec(&self.pool)
            .await
            .map_err(catalog::Error::any)?;

        let list_query = queries::ListQuery {
            limit: u16::try_from(query.limit).unwrap_or(u16::MAX),
            page: query.page,
        };

        let models = list_query
            .exec(&self.pool)
            .await
            .map_err(catalog::Error::any)?;

        let catalogs = models
            .into_iter()
            .map(CatalogWithProductsModel::try_into_entity)
            .collect::<Result<Vec<_>, _>>()
            .map_err(catalog::Error::any)?;

        Ok(catalog::Pagination {
            count,
            page: query.page,
            limit: query.limit,
            items: catalogs,
        })
    }

    async fn update(&mut self, catalog: &catalog::Catalog) -> Result<(), catalog::Error> {
        let query = queries::UpdateQuery { catalog };
        query.exec(&self.pool).await.map_err(|err| {
            if matches!(err, sqlx::Error::RowNotFound) {
                catalog::Error::id_conflict(catalog.id())
            } else if Self::is_ak_name_error(&err) {
                catalog::Error::name_conflict(catalog.name.clone())
            } else {
                catalog::Error::any(err)
            }
        })?;

        Ok(())
    }
}
