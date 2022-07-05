use azure_data_cosmos::prelude::*;
use clap::Parser;
use futures::StreamExt;

mod util;

#[derive(Debug, clap::Parser)]
struct Args {
    #[clap(flatten)]
    auth: util::Auth,
    /// The name of the database
    database_name: String,
    /// The name of the collection
    collection_name: String,
    /// The name of the second collection
    collection_name2: String,
    /// The name of the user
    user_name: String,
}

#[tokio::main]
async fn main() -> azure_core::Result<()> {
    let args = Args::parse();
    let client = args.auth.into_client()?;

    let database = client.database_client(args.database_name);
    let collection = database.collection_client(args.collection_name);
    let collection2 = database.collection_client(args.collection_name2);
    let user = database.user_client(args.user_name);

    let get_database_response = database.get_database().into_future().await?;
    println!("get_database_response == {:#?}", get_database_response);

    let get_collection_response = collection.get_collection().into_future().await?;
    println!("get_collection_response == {:#?}", get_collection_response);

    let get_collection2_response = collection2.get_collection().into_future().await?;
    println!(
        "get_collection2_response == {:#?}",
        get_collection2_response
    );

    let create_user_response = user.create_user().into_future().await?;
    println!("create_user_response == {:#?}", create_user_response);

    // create the first permission!
    let permission = user.permission_client("matrix");
    let permission_mode = get_collection_response.collection.read_permission();

    let create_permission_response = permission
        .create_permission(permission_mode)
        .consistency_level(&create_user_response)
        .expiry_seconds(18000u64)
        .into_future()
        .await
        .unwrap();
    println!(
        "create_permission_response == {:#?}",
        create_permission_response
    );

    // create the second permission!
    let permission = user.permission_client("neo".to_owned());
    let permission_mode = get_collection2_response.collection.all_permission();

    let create_permission2_response = permission
        .create_permission(permission_mode)
        .consistency_level(&create_user_response)
        .into_future()
        .await
        .unwrap();

    println!(
        "create_permission2_response == {:#?}",
        create_permission2_response
    );
    let user = user;

    let list_permissions_response = user
        .list_permissions()
        .consistency_level(ConsistencyLevel::Session(
            create_permission2_response.session_token,
        ))
        .into_stream()
        .next()
        .await
        .unwrap()?;
    println!(
        "list_permissions_response == {:#?}",
        list_permissions_response
    );

    let get_permission_response = permission
        .get_permission()
        .consistency_level(ConsistencyLevel::Session(
            list_permissions_response.session_token,
        ))
        .into_future()
        .await
        .unwrap();
    println!("get_permission_response == {:#?}", get_permission_response);

    let permission_mode = get_permission_response.permission.permission_mode;

    // renew permission extending its validity for 60 seconds more.
    let replace_permission_response = permission
        .replace_permission(permission_mode)
        .expiry_seconds(600u64)
        .consistency_level(ConsistencyLevel::Session(
            get_permission_response.session_token,
        ))
        .into_future()
        .await
        .unwrap();
    println!(
        "replace_permission_response == {:#?}",
        replace_permission_response
    );

    let delete_permission_response = permission
        .delete_permission()
        .consistency_level(ConsistencyLevel::Session(
            replace_permission_response.session_token,
        ))
        .into_future()
        .await
        .unwrap();

    println!(
        "delete_permission_response == {:#?}",
        delete_permission_response
    );

    let delete_user_response = user
        .delete_user()
        .consistency_level(ConsistencyLevel::Session(
            delete_permission_response.session_token,
        ))
        .into_future()
        .await?;
    println!("delete_user_response == {:#?}", delete_user_response);

    Ok(())
}
