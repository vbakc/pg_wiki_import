mod article;
mod environment;

#[macro_use]
extern crate serde_derive;

use article::Article;
use quick_xml::events::Event;
use sqlx::postgres::PgPoolOptions;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = environment::get_env();

    let db_pool = PgPoolOptions::new()
        .max_connections(env.db_pool as u32)
        .connect(&env.database_url)
        .await?;

    sqlx::migrate!().run(&db_pool).await?;

    let mut reader = quick_xml::Reader::from_file(env.file_path)?;
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut curr_event: String = "".into();
    let mut curr_title: Option<String> = None;
    let mut curr_text: Option<String> = None;

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                curr_event = e
                    .unescape_and_decode(&reader)?
                    .split(" ")
                    .collect::<Vec<&str>>()[0]
                    .to_owned();
            }
            Ok(Event::Text(ref e)) => match &curr_event[..] {
                "title" => {
                    curr_title = Some(e.unescape_and_decode(&reader)?);
                }
                "text" => {
                    if curr_title.is_some() {
                        curr_text = Some(e.unescape_and_decode(&reader)?);
                    }
                }
                _ => (),
            },
            Ok(Event::End(ref e)) if e.name() == b"page" => {
                if curr_title.is_some() && curr_text.is_some() {
                    let pool = db_pool.clone();

                    tokio::spawn(async move {
                        Article {
                            title: curr_title.unwrap(),
                            content: curr_text.unwrap(),
                        }
                        .save(&pool)
                        .await
                        .unwrap()
                    });
                }
                curr_title = None;
                curr_text = None;
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
        buf.clear();
    }

    Ok(())
}
