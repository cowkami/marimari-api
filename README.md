# marimari-admin-api

## Run develop serevr

```bash
$ cargo run
```

## Build

```bash
$ cargo build
```

## Deploy

```bash
$ cargo install cargo-lambda  # if you don't have cargo-lambda
$ cargo lambdba deploy -p AWS_CLI_PROFILE --enable-function-url marimari-admin-api --iam-role IAM_ROLE
```