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
$ cargo lambda deploy -p marimari --binary-name marimari-api --enable-function-url marimari-api-prod --iam-role IAM_ROLE

```
