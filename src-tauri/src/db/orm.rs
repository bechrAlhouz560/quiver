use sqlx::query_builder::Separated;
use sqlx::{sqlite::SqliteRow, FromRow, Pool, QueryBuilder, Sqlite};

use std::marker::PhantomData;

pub struct Model<'a, T> {
    pub pool: &'a Pool<Sqlite>,
    _marker: PhantomData<T>,
}

pub trait TableMetadata {
    const TABLE_NAME: &'static str;
}

// Redefine Boxable as a trait object capable of appending SQL chunks and binding parameters.
pub trait Boxable {
    // We pass a mutable reference to the `Separated` helper, allowing it to inject delimiters safely
    fn append_to(&self, sep: &mut Separated<'_, Sqlite, &str>);
}

impl<'a, T> Model<'a, T>
where
    T: TableMetadata + for<'r> FromRow<'r, SqliteRow> + Send + Unpin,
{
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<T>, sqlx::Error> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT * FROM ");
        qb.push(T::TABLE_NAME);
        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let query = qb.build_query_as::<T>();
        let result = query.fetch_optional(self.pool).await?;
        Ok(result)
    }

    // Updated find_all to take select columns AND an optional slice of boxed trait objects.
    pub async fn find_all(
        &self,
        cols: Vec<&str>,
        filters: Option<&[Box<dyn Boxable + Send + Sync>]>,
    ) -> Result<Vec<T>, sqlx::Error> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT ");

        if cols.is_empty() {
            qb.push("*");
        } else {
            let mut separated = qb.separated(", ");
            for col in cols {
                separated.push(col);
            }
        }

        qb.push(" FROM ");
        qb.push(T::TABLE_NAME);

        if let Some(filter_list) = filters {
            if !filter_list.is_empty() {
                qb.push(" WHERE ");

                // Initialize the structural separator loop instance
                let mut separated = qb.separated(" AND ");
                for filter in filter_list {
                    // Pass the active borrow context downwards without touching `qb` directly
                    filter.append_to(&mut separated);
                }
            }
        }

        let query = qb.build_query_as::<T>();
        let results = query.fetch_all(self.pool).await?;
        Ok(results)
    }

    pub async fn create(
        &self,
        fields: &[(&str, Box<dyn Boxable + Send + Sync>)],
    ) -> Result<i64, sqlx::Error> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("INSERT INTO ");
        qb.push(T::TABLE_NAME);
        qb.push(" (");

        // Push column names
        let mut col_sep = qb.separated(", ");
        for (col, _) in fields {
            col_sep.push(*col);
        }

        qb.push(") VALUES (");

        // Push bound values
        let mut val_sep = qb.separated(", ");
        for (_, value) in fields {
            value.append_to(&mut val_sep);
        }

        qb.push(")");

        let result = qb.build().execute(self.pool).await?;

        println!("last inserted row: {}", result.last_insert_rowid());
        Ok(result.last_insert_rowid())
    }
}

//  Filters for search
pub struct EqFilter {
    pub column: &'static str,
    pub value: String,
}

impl Boxable for EqFilter {
    fn append_to(&self, sep: &mut Separated<'_, Sqlite, &str>) {
        // Separated mirrors QueryBuilder methods natively
        sep.push(self.column);
        sep.push(" = ");
        sep.push_bind(self.value.clone());
    }
}

pub struct GreaterThanFilter {
    pub column: &'static str,
    pub value: i64,
}

pub struct Field {
    pub value: String,
}

impl Boxable for Field {
    fn append_to(&self, sep: &mut Separated<'_, Sqlite, &str>) {
        sep.push_bind(self.value.clone());
    }
}
impl Boxable for GreaterThanFilter {
    fn append_to(&self, sep: &mut Separated<'_, Sqlite, &str>) {
        sep.push(self.column);
        sep.push(" > ");
        sep.push_bind(self.value);
    }
}
