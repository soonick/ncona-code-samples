use firestore::{
    FirestoreDb,
    FirestoreDbOptions,
    errors::{
        BackoffError,
        FirestoreError,
    },
};
use futures::FutureExt;
use serde::{Deserialize, Serialize};

const COLLECTION_NAME: &'static str = "users";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct User{
    username: String,
    views: i32,
}

async fn save_user() {
    let main_db = match FirestoreDb::with_options(
        FirestoreDbOptions::new("dummy-id".to_string())
            .with_database_id("test_db".to_string()),
    )
    .await
    {
        Ok(db) => db,
        Err(err) => panic!(
            "Unable create firestore db for project: test_project. Error: {}", err
        ),
    };

    let jose = User {
        username: "jose".to_string(),
        views: 0,
    };

    match main_db.fluent()
        .insert()
        .into(COLLECTION_NAME)
        .document_id("jose".to_string())
        .object(&jose)
        .execute::<()>()
        .await {
            Ok(_) => {
                println!("Jose inserted");
            },
            Err(err) => {
                panic!("Error inserting user: {}", err);
            }
        };


    let tx_res = main_db.run_transaction(|db, tx| {
        async move {
            // Get the user
            let found_user_opt: Option<User> = match db
                .fluent()
                .select()
                .by_id_in(COLLECTION_NAME)
                .obj()
                .one("jose")
                .await {
                    Ok(f) => f,
                    Err(err) => {
                        println!("Error finding user: {}", err);
                        return Ok::<bool, BackoffError<FirestoreError>>(false);
                    }
                };

            if found_user_opt.is_some() {
                println!("User found");
            } else {
                println!("User not found");
                return Ok(false);
            }

            // Update a field
            let mut found_user = found_user_opt.unwrap();
            found_user.views = found_user.views + 1;

            // Write the update
            match db.fluent()
                .update()
                .in_col(COLLECTION_NAME)
                .document_id("jose".to_string())
                .object(&found_user)
                .add_to_transaction(tx) {
                    Ok(_) => {},
                    Err(err) => {
                        panic!("Error updating user: {}", err);
                    }
                };

            return Ok(true);
        }.boxed()
    }).await;

    match tx_res {
        Ok(r) => {
            if r {
                println!("Transaction succeeded");
            } else {
                println!("Transaction failed");
            }
        },
        Err(e) => {
            println!("Error executing transaction: {}", e);
        }
    }
}

#[tokio::main]
async fn main() {
    save_user().await;
}
