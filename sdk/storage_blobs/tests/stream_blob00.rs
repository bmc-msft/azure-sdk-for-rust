#![cfg(all(test, feature = "test_e2e"))]
use azure_core::{
    error::{ErrorKind, ResultExt},
    prelude::*,
};
use azure_storage::core::prelude::*;
use azure_storage_blobs::prelude::*;
use futures::StreamExt;
use uuid::Uuid;

#[tokio::test]
async fn create_blob_and_stream_back() {
    println!("once");
    code().await.unwrap();
}

async fn code() -> azure_core::Result<()> {
    let container_name = format!("create-{}", Uuid::new_v4().to_string());
    let file_name = "azure_sdk_for_rust_stream_test.txt";

    // First we retrieve the account name and access key from environment variables.
    let account =
        std::env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        std::env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");

    println!("a");
    let http_client = azure_core::new_http_client();
    println!("new client");

    let storage = StorageAccountClient::new_access_key(http_client.clone(), &account, &access_key)
        .as_storage_client();
    println!("storage containers");
    let blob_service = storage.as_blob_service_client();
    let container = storage.as_container_client(&container_name);
    let blob = container.as_blob_client(file_name);

    println!("list containers");
    if blob_service
        .list_containers()
        .into_stream()
        .next()
        .await
        .unwrap()?
        .containers
        .iter()
        .find(|x| x.name == container_name)
        .is_none()
    {
        println!("create container");
        container
            .create()
            .public_access(PublicAccess::None)
            .into_future()
            .await?;
    }
    println!("create container");

    let string = "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF";
    println!("put block {}", string.len());

    blob.put_block_blob(string)
        .content_type("text/plain")
        .into_future()
        .await?;

    println!("{}/{} blob created!", container_name, file_name);

    let mut stream = blob.get().chunk_size(5u64).into_stream();
    let mut result = vec![];
    while let Some(entry) = stream.next().await {
        let entry = entry?;
        println!("got {:?}", entry.data);
        result.extend(&entry.data);
    }

    let returned_string = { String::from_utf8(result).map_kind(ErrorKind::DataConversion)? };
    assert_eq!(string, returned_string);

    // test streaming a blob smaller than the chunk size issue 239.
    let mut stream = blob.get().chunk_size(0xFFFFu64).into_stream();
    let first = stream.next().await.expect("first chunk")?;
    let result = String::from_utf8(first.data.to_vec()).map_kind(ErrorKind::DataConversion)?;
    assert_eq!(result, string);

    assert!(stream.next().await.is_none(), "second chunk should be None");
    //while let Some(_value) = stream.next().await {}

    blob.delete()
        .delete_snapshots_method(DeleteSnapshotsMethod::Include)
        .into_future()
        .await?;

    println!("{}/{} blob deleted!", container_name, file_name);

    container.delete().into_future().await?;
    Ok(())
}
