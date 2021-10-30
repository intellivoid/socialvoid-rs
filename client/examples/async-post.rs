// You need to make a file called `test_creds.test` in the root of the project for
// this example to run. Though, it makes more sense to work using a session instead of saving
// the password in plaintext in a file in real applications
// The file is a JSON file with the following format
// {
//     "username":"yourusername",
//     "password":"yourpassword"
// }

#[tokio::main]
async fn main() {
    let creds1: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string("test_creds.test").unwrap())
            .expect("Couldn't read the credentials. Check the JSON format or something");

    let mut client1 = socialvoid::new_with_defaults().await.unwrap();
    client1
        .authenticate_user(
            creds1["username"].as_str().unwrap().to_string(),
            creds1["password"].as_str().unwrap().to_string(),
            None,
        )
        .await
        .unwrap();

    let handle = tokio::spawn(async move {
        let post = client1.compose_post("Yayaya", vec![]).await.unwrap();
        println!("Made post!");
        if client1.delete_post(post.id).await.unwrap() {
            println!("Deleted successfully!");
        }
    });

    handle.await.unwrap();
}
