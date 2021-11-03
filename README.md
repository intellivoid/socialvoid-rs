# socialvoid-rs

This repository contains
1. a crate that is a high level implementation of a library to access the Socialvoid API. It can be used to build clients for the platform or build automated programs to perform tasks on the Socialvoid network.
2. a simple CLI client for Socialvoid based on the crate

## Library Documentation

//TODO: link to this crate's documentation once it's up.

Most of the methods are `async`  
Methods can be accessed as `<namespace>.<methodname>`  
For example, to invoke the `authenticate_user` method in `session` namespace, you can use `sv.session.authenticate_user(...)`.  

Incase any method isn't available in this crate, you can make raw requests using the `socialvoid-rawclient` crate, the latest full API Documentation of Socialvoid is [here](https://github.com/intellivoid/Socialvoid-Standard-Documentation)

## Example for the library

A simple example that logs you in and shows peer information.
```rust
// You need to make a file called `test_creds.test` in the root of the project for
// this example to run.
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

```
More examples can be found in [client/examples](https://github.com/intellivoid/socialvoid-rs/tree/master/client/examples)

## Installation of the CLI client
1. Clone this repository
2. `cd` into the repository
3. Install the cli   
`cargo install --path cli`

## Usage of the CLI client
See the full documentation here(TODO: a link to full documentation)

Some of the commands are:
### Register an account
`socialvoid-cli register`
### Login in to an account
`socialvoid-cli login`
### Get the current peer
`socialvoid-cli get-me`

//TODO: add contributors section