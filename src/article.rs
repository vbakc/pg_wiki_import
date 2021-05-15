use sqlx::{Pool, Postgres};

#[derive(sqlx::FromRow)]
pub struct Article {
    pub title: String,
    pub content: String,
}

impl Article {
    pub async fn save(self, pool: &Pool<Postgres>) -> Result<Article, sqlx::Error> {
        sqlx::query_as!(
            Article,
            "INSERT INTO articles (title, content) VALUES ($1, $2) RETURNING title, content",
            self.title,
            self.content
        )
        .fetch_one(pool)
        .await
    }
}
