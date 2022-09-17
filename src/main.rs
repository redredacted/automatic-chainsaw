use clap::{Parser, Subcommand};
use serde_json::value;
use surrealdb::Datastore;
use surrealdb::Error;
use surrealdb::Session;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct TodoTask {
    id: String,
    title: String,
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
   },
   List,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Todo { command } => {
            match command {
                TodoCommands::Add { title } => _ = add(title).await,
                TodoCommands::List => _ = list().await,
            }
        }, 
    }
}

async fn add(title: &String) -> Result<(), Error> {
    let ds = Datastore::new("file://todo.db").await?;
    let ses = Session::for_kv();
    let ast = format!("USE NS todo DB todo; CREATE task SET title = '{}'", title);
    let _res = ds.execute(ast.as_str(), &ses, None, false).await?;
    Ok(())
}

async fn list() -> Result<(), Error> {
    let ds = Datastore::new("file://todo.db").await?;
    let ses = Session::for_kv();
    let ast = "USE NS todo DB todo; SELECT * FROM task";
    let res = ds.execute(ast, &ses, None, false).await?;
    let mut tasks: Vec<TodoTask> = Vec::new();
    for ele in res {
        if let Ok(res) = ele.result {
            let serial = res.serialize(value::Serializer).unwrap();
            let value = serial.to_string();
            if value != "null" {
                // println!("Value: {}", value);
                tasks = serde_json::from_str(value.as_str()).unwrap();
            }
        } else {
            todo!()
        }
    }

    for todo in tasks {
        println!("TodoId: {}, Title: {}", todo.id, todo.title)
    }

    Ok(())
}
