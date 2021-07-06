# About
Work in progress. I am likely to add more imports or fix bugs if someone
opens issue in github.

This crate provides auth0-spa-js wasm-bindgen imports for some functionality.

Also, this crate includes opinionated yew implementation of auth0-spa-js (cargo feature "auth0-yew").

# Run tests

## Make sure chrome driver is running
chromedriver --port=4444

## Run testing app
cd testing/app

trunk serve --port=8000

## Run selenium tests
cd testing/selenium

cargo run
