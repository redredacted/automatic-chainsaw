use clap::{arg, command, Command};
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

#[tokio::main]
async fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("add")
                .about("Adds files to myapp")
                .arg(arg!([TITLE]))
        )
        .subcommand(
            Command::new("list")
                .about("lists todo records")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => _ = add(sub_matches.get_one::<String>("TITLE").unwrap_or(&String::from(""))).await,
        Some(("list", _)) => _ = list().await, 
        _ => unreachable!("no matching commands try --help"),
    }
}

async fn add(title: &str) -> Result<(), Error> {
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
