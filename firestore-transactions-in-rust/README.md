# Firestore Transactions in Rust

This example uses firestore emulator. Sadly, the emulator requires a service account key to work, so we need to create one from [google cloud console](https://console.cloud.google.com/iam-admin/serviceaccounts). It's not necessary to give any permissions to the service account. Save the json key to a file named `service-account-key.json` in the same directory as this README file.

There are 2 working examples that can be run with:

```
make run-basic
make run-insert
```

There is one example showing that select and insert of the same document fails:

```
make run-conflict
```
