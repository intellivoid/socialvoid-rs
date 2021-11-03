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
    let creds: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string("test_creds.test").unwrap())
            .expect("Couldn't read the credentials. Check the JSON format or something");

    let sv = socialvoid::new_with_defaults().await.unwrap();
    let username = creds["username"].as_str().unwrap().to_string();
    let password = creds["password"].as_str().unwrap().to_string();
    sv.session
        .authenticate_user(username, password, None)
        .await
        .unwrap();

    let peer = sv.network.get_me().await.unwrap();

    println!("{:?}", peer);
    sv.session.logout().await.unwrap();

    assert_eq!(peer.username, username);
}
