use clap::Parser;
use futures::stream::StreamExt;
use serde_json::Value;

mod util;

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let client = util::Auth::parse().into_client()?;

    let dbs = client
        .list_databases()
        .into_stream()
        .next()
        .await
        .unwrap()?;

    for db in dbs.databases {
        println!("database == {:?}", db);
        let database = client.database_client(db.name().to_owned());

        let collections = database
            .list_collections()
            .into_stream()
            .next()
            .await
            .unwrap()?;
        for collection in collections.collections {
            println!("collection == {:?}", collection);
            let mut indexing_policy_new = collection.indexing_policy.clone();
            let collection = database.collection_client(collection.id);

            if collection.collection_name() == "democ" {
                println!("democ!");

                let data = r#"
                {
                    "id": "my_id",
                    "name": "John Tonno7",
                    "age": 43,
                    "phones": [
                        "+44 1234567",
                        "+44 2345678"
                    ]
                }"#;
                let document: Value = serde_json::from_str(data)?;

                let resp = collection
                    .create_document(document)
                    .is_upsert(true)
                    .partition_key(&43u32)?
                    .into_future()
                    .await?;

                println!("resp == {:?}", resp);

                // call replace collection
                indexing_policy_new
                    .excluded_paths
                    .push("/\"collo2\"/?".to_owned().into());

                println!("\nReplacing collection");
                let replace_collection_response = collection
                    .replace_collection("/age")
                    .indexing_policy(indexing_policy_new)
                    .into_future()
                    .await?;
                println!(
                    "replace_collection_response == {:#?}",
                    replace_collection_response
                );
            }

            let documents = collection
                .list_documents()
                .into_stream::<Value>()
                .next()
                .await
                .unwrap()?;
            println!("\ndocuments as json == {:?}", documents);
        }
    }

    Ok(())
}
