# Pick For Me

In a world where we have too many decisions to make, this app aims to help eliminate some of that stress.

## What's In This Repo

This repo contains a Spin application with two components: a frontend built with vue.js and a Rust component that does some AI inferencing. The frontend collections options and picks one at random while the rust component gives you a random fact about the chosen option.

## Build and Run Locally

1. Set up cloud-gpu plugin and instance: https://github.com/fermyon/spin-cloud-gpu (Helps make the AI inferencing faster during local dev)
2. `$ spin build --up --runtime-config-file <path-to-runtime-config-file>`

## Deploy to Fermyon Cloud

`$ spin deploy`
