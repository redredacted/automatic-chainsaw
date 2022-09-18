use clap::{Parser, Subcommand};
use serde_json::value;
use surrealdb::Datastore;
use surrealdb::Error;
use surrealdb::Session;
use surrealdb::sql::Value;
use serde::{Serialize, Deserialize};
use map_macro::btree_map;
use chrono::{DateTime, Local};

// test comment
const DB_FILE: &str = "file://todo.db";

#[derive(Serialize, Deserialize, Debug)]
struct TodoTask {
    id: String,
    title: String,
    description: Option<String>,
    created_at: DateTime<Local>,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
   Todo {
        #[clap(subcommand)]
        command: TodoCommands,
   }
}

#[derive(Subcommand)]
enum TodoCommands {
   Add {
       #[clap(value_parser)]
       title: String,
       #[clap(short, long, value_parser)]
       description: Option<String>,
   },
   List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Todo { command } => {
            match command {
                TodoCommands::Add {title, description } => _ = add(title, description).await,
                TodoCommands::List => _ = list().await,
            }
        }, 
    }
}

async fn add(title: &str, description: &Option<String>) -> Result<(), Error> {
    let ds = Datastore::new(DB_FILE).await?;
    let ses = Session::for_kv();
    let ast = String::from("USE NS todo DB todo; CREATE task SET title = $title, description = $description, created_at = time::now()");

    let mut vars = btree_map! {
        String::from("title") => Value::from(title),
        /* if let Some(desc) = description { String::from("description") = Value::from(desc) }, */
    };

    if let Some(desc) = description {
        vars.insert(String::from("description"), Value::from(desc.as_str()));
    }


    let _res = ds.execute(ast.as_str(), &ses, Some(vars), false).await?;
    Ok(())
}

async fn list() -> Result<(), Error> {
    let ds = Datastore::new(DB_FILE).await?;
    let ses = Session::for_kv();
    let ast = "USE NS todo DB todo; SELECT * FROM task";
    let res = ds.execute(ast, &ses, None, false).await?;
    
    let mut tasks: Vec<TodoTask> = Vec::new();

    for ele in res {
        if let Ok(res) = ele.output() {
            let serial = res.serialize(value::Serializer).unwrap();
            let value = serial.to_string();
            // println!("Value: {}", value);
            if let Ok(val) = serde_json::from_str(value.as_str()) {
                tasks = val; 
            }
        }
    }

    for todo in tasks {
        println!("TodoId: {}, Title: {}, Description: {}, created_at: {}", todo.id, todo.title, todo.description.unwrap_or_else(|| String::from("")), todo.created_at)
    }

    Ok(())
}
