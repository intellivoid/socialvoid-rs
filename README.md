# socialvoid-rs

This repository contains
1. a crate that is a high level implementation of a library to access the Socialvoid API. It can be used to build clients for the platform or build automated programs to perform tasks on the Socialvoid network.
2. a simple CLI client for Socialvoid based on the crate

## Library Documentation

//TODO: link to this crate's documentation once it's up.

Incase any method isn't available in this crate, you can make raw requests using the `socialvoid-rawclient` crate, the latest full API Documentation of Socialvoid is [here](https://github.com/intellivoid/Socialvoid-Standard-Documentation)

## Example for the library

A simple example that logs you in and shows peer information.
```rust
use socialvoid::Client;

let mut client = new_with_defaults();

client
    .authenticate_user(
        0,
        creds["username"].as_str().unwrap().to_string(),
        creds["password"].as_str().unwrap().to_string(),
        None,
    )
    .await?;

let peer = client.get_me(0).await?;
client.logout(0).await?;

println!("{:?}", peer);
assert_eq!(
    peer.username,
    creds["username"].as_str().unwrap().to_string()
);

```
More examples can be found in client/examples (TODO+add a link)

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