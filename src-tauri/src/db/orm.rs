use sqlx::{sqlite::SqliteRow, FromRow, Pool, QueryBuilder, Sqlite};
use std::marker::PhantomData;

// Declare the lifetime 'a on the struct so it can hold a pool reference.
pub struct Model<'a, T> {
    pub pool: &'a Pool<Sqlite>,
    _marker: PhantomData<T>,
}

pub trait TableMetadata {
    const TABLE_NAME: &'static str;
}

// Pass the lifetime parameter 'a down into the implementation block.
impl<'a, T> Model<'a, T>
where
    T: TableMetadata + for<'r> FromRow<'r, SqliteRow> + Send + Unpin + Boxable,
{
    // Link the input reference lifetime to the struct instance lifetime.
    pub fn new(pool: &'a Pool<Sqlite>) -> Self {
        Self {
            pool,
            _marker: PhantomData,
        }
    }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<T>, sqlx::Error> {
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT * FROM ");

        // Cleaned up to use T::TABLE_NAME automatically from your trait metadata!
        qb.push(T::TABLE_NAME);
        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let query = qb.build_query_as::<T>();

        // This borrow now safely survives until the async task resolves.
        let result = query.fetch_optional(self.pool).await?;

        Ok(result)
    }

    pub async fn find_all(&self, cols: Vec<&str>) -> Result<Vec<T>, sqlx::Error> {
        // Initialize the query prefix
        let mut qb: QueryBuilder<Sqlite> = QueryBuilder::new("SELECT ");

        // Safely add columns or fallback to "*" if the vector is empty
        if cols.is_empty() {
            qb.push("*");
        } else {
            // Automatically injects a comma separator between every item
            let mut separated = qb.separated(", ");
            for col in cols {
                separated.push(col);
            }
        }

        // Append target table dynamically from metadata
        qb.push(" FROM ");
        qb.push(T::TABLE_NAME);

        // Construct the query object, execute, and return rows
        let query = qb.build_query_as::<T>();
        let results = query.fetch_all(self.pool).await?;

        Ok(results)
    }
}

pub trait Boxable {}
impl<T> Boxable for T {}
