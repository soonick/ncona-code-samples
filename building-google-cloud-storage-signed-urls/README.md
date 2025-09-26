# Building Google Cloud Storage Signed URLs

## Dependencies

- Docker
- Make

## Preparation

In order to run this code, you will need a JSON key for a service account. Save the key in the same path as this file with the name `service-acocunt.json`.

The service account must have at least these permissions:

- iam.serviceAccounts.signBlob
- storage.objects.get

Update the values for `BUCKET_NAME` and `OBJECT_PATH` in `src/main.rs` to match the bucket and object you will be reading.

## Run

To run the code that will generate a signed URL:

```
make run
```
