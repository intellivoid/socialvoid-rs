# socialvoid-rs

This crate is a high level implementation to access the SocialVoid API. It can be used to build clients for the platform or build automated programs to perform tasks on the SocialVoid network.

## Installation of the CLI client
1. Clone this repository
2. `cd` into the repository
3. Install the cli   
`cargo install --path cli`

## Usage of the CLI client
### Register an account
`socialvoid-cli register`
### Login in to an account
`socialvoid-cli login`
### Get the current peer
`socialvoid-cli get-me`

## API Documentation and usage

To create your own client based on this library, you can add `socialvoid == 1.0` in your `Cargo.toml`   
TODO: everything   
Link to API Documentation

## Example for the library

```
use socialvoid::Client;

let client = Client::new_with_defaults();

```