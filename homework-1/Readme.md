# Rust Development Environment Setup

## Assignment Description
This assignment focuses on setting up a Rust development environment and creating a basic "Hello, World!" project.

## Environment
I decided to go with [devenv](https://devenv.sh/) since I have been already familiar with it. However I also wanted to add crane prebuilt packages to speed up the build process. After some time trying to make it work with devenv I gave up and setteled with [local development flake](https://crane.dev/local_development.html) provided by crane.     t should help iterate faster since it reuses artefacts from previous cargo runs and builds.

## Hello World script
A simple script slowly printing hello to lecturer. Therer a suddle difference between print and println functions, the first one doesnt add newline char ulike the second one.Also flush func doesnt accept unborrowed mut.  
