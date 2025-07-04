use async_stream::stream;
use futures_util::stream;
use gcloud_sdk::{
    google::storage::v2::{
        Bucket,
        ChecksummedData,
        CreateBucketRequest,
        DeleteBucketRequest,
        DeleteObjectRequest,
        GetBucketRequest,
        ListBucketsRequest,
        ListObjectsRequest,
        Object,
        StartResumableWriteRequest,
        WriteObjectRequest,
        WriteObjectSpec,
        storage_client::StorageClient,
        write_object_request::{Data, FirstMessage},
    },
    GoogleApi,
    GoogleAuthMiddleware
};
use rand::Rng;
use std::{fs, io::Read};
use tonic::{metadata::{MetadataValue}, Request};

const PROJECT_ID: &str = ""; // TODO: Set this to your google cloud project id
const LOCAL_FILE_NAME: &str = "image.jpg";
const CHUNK_SIZE: usize = 120_800; // Must be a multiple of 256
const RESUMABLE_CHUNK_SIZE: usize = 262_144; // Must be a multiple of 262,144

#[tokio::main]
async fn main() {
    let gcs : GoogleApi<StorageClient<GoogleAuthMiddleware>> = match GoogleApi::from_function(StorageClient::new, "https://storage.googleapis.com", None).await {
        Ok(c) => c,
        Err(_) => panic!("Oh no!")
    };

    // Create a bucket
    println!("-----");
    let mut rng = rand::rng();
    let rn: u16 = rng.random();
    let bucket_id = format!("gcloud-sdk-example-bucket-{}", rn);
    println!("Creating bucket: {}", bucket_id);

    let bucket = Bucket {
        project : format!("projects/{}", PROJECT_ID),
        ..Default::default()
    };
    let mut request = Request::new(CreateBucketRequest {
        parent: "projects/_".to_string(),
        bucket_id: bucket_id.clone(),
        bucket: Some(bucket),
        ..Default::default()
    });

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}", PROJECT_ID)).unwrap(),
    );

    match gcs
        .get()
        .create_bucket(request)
        .await {
            Ok(_) => {
                println!("Bucket {} created succesfully", bucket_id);
            },
            Err(e) => {
                panic!("Error creating bucket. Code: {} Full response: {:?}", e.code(), e)
            }
        };

    // List all buckets
    println!("-----");
    let mut request = Request::new(ListBucketsRequest {
        parent: format!("projects/{}", PROJECT_ID),
        ..Default::default()
    });

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}", PROJECT_ID)).unwrap(),
    );

    match gcs
        .get()
        .list_buckets(request)
        .await {
            Ok(r) => {
                println!("Buckets:");
                for b in r.get_ref().buckets.clone() {
                    println!("{}", b.name);
                }
            },
            Err(e) => {
                panic!("Error listing buckets. Code: {} Full response: {:?}", e.code(), e)
            }

        };

    // Get bucket
    println!("-----");
    let mut request = Request::new(GetBucketRequest {
        name: bucket_id.clone(),
        ..Default::default()
    });

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}", PROJECT_ID)).unwrap(),
    );

    match gcs
        .get()
        .get_bucket(request)
        .await {
            Ok(r) => {
                println!("Bucket retrieved: {:?}", r);
            },
            Err(e) => {
                panic!("Error getting bucket. Code: {} Full response: {:?}", e.code(), e)
            }

        };

    // Write object in one shot
    println!("-----");
    let file_name = "one_shot.jpg";
    println!("Writing file {} to bucket {} in one shot", file_name, bucket_id);
    let file_bytes: Vec<u8> = match fs::read(LOCAL_FILE_NAME) {
        Ok(b) => b,
        Err(e) => {
            panic!("Error reading file bytes: {:?}", e);
        }
    };

    let write_request = WriteObjectRequest {
        first_message: Some(FirstMessage::WriteObjectSpec(WriteObjectSpec {
            resource: Some(Object {
                name: file_name.to_string(),
                bucket: format!("projects/_/buckets/{}", bucket_id),
                ..Default::default()
            }),
            ..Default::default()
        })),
        data: Some(Data::ChecksummedData(ChecksummedData {
            content: file_bytes,
            ..Default::default()
        })),
        finish_write: true,
        ..Default::default()
    };
    let req_stream = stream::iter(vec![write_request]);
    let mut request = Request::new(req_stream);

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
    );

    match gcs
        .get()
        .write_object(request)
        .await {
            Ok(r) => {
                println!("Object written: {:?}", r);
            },
            Err(e) => {
                panic!("Error writing object. Code: {} Full response: {:?}", e.code(), e)
            }

        };

    // Write object as stream
    println!("-----");
    let file_name = "stream.jpg";
    println!("Writing file {} to bucket {} in a stream", file_name, bucket_id);

    let bucket_id_for_stream = bucket_id.clone(); // clone for the stream
    let req_stream = stream! {
        let mut offset = 0;
        let mut finish = false;
        let mut buffer = vec![0u8; CHUNK_SIZE];
        let mut file = match std::fs::File::open(LOCAL_FILE_NAME) {
            Ok(f) => f,
            Err(e) => {
                panic!("Error openning file: {:?}", e);
            }
        };

        while !finish {
            let n = match file.read(&mut buffer) {
                Ok(r) => r,
                Err(e) => {
                    panic!("Error reading file: {:?}", e);
                }
            };
            println!("Bytes read: {}, offset: {}", n, offset);

            if n == 0 || n < CHUNK_SIZE {
                finish = true;
            }

            let fm = if offset == 0 {
                Some(FirstMessage::WriteObjectSpec(WriteObjectSpec {
                    resource: Some(Object {
                        name: file_name.to_string(),
                        bucket: format!("projects/_/buckets/{}", bucket_id_for_stream),
                        ..Default::default()
                    }),
                    ..Default::default()
                }))
            } else {
                None
            };

            let request = WriteObjectRequest {
                write_offset: offset,
                first_message: fm,
                data: Some(Data::ChecksummedData(ChecksummedData {
                    content: buffer[..n].to_vec(),
                    ..Default::default()
                })),
                finish_write: finish,
                ..Default::default()
            };

            offset += n as i64;
            yield request;
        }
    };

    let mut request = Request::new(req_stream);

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
    );

    match gcs
        .get()
        .write_object(request)
        .await {
            Ok(r) => {
                println!("Object written: {:?}", r);
            },
            Err(e) => {
                panic!("Error writing object. Code: {} Full response: {:?}", e.code(), e)
            }
        };

    // Write object in chunks
    println!("-----");
    let file_name = "resumable.jpg";
    println!("Writing file {} to bucket {} as resumable update", file_name, bucket_id);
    let mut first_req = Request::new(StartResumableWriteRequest {
        write_object_spec: Some(WriteObjectSpec {
            resource: Some(Object {
                name: file_name.to_string(),
                bucket: format!("projects/_/buckets/{}", bucket_id),
                ..Default::default()
            }),
            ..Default::default()
        }),
        ..Default::default()
    });

    first_req.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
    );

    let resumable_id = match gcs
        .get()
        .start_resumable_write(first_req)
        .await {
            Ok(r) => {
                println!("Started resumable write with id: {}", r.get_ref().upload_id);
                r.get_ref().upload_id.clone()
            },
            Err(e) => {
                panic!("Error starting resumable write. Code: {} Full response: {:?}", e.code(), e)
            }
        };

    let mut finish = false;
    let mut offset = 0;
    let mut buffer = vec![0u8; RESUMABLE_CHUNK_SIZE];
    let mut file = match std::fs::File::open(LOCAL_FILE_NAME) {
        Ok(f) => f,
        Err(e) => {
            panic!("Error openning file: {:?}", e);
        }
    };

    while !finish {
        let n = match file.read(&mut buffer) {
            Ok(r) => r,
            Err(e) => {
                panic!("Error reading file: {:?}", e);
            }
        };
        println!("Bytes read: {}, offset: {}", n, offset);

        if n == 0 || n < RESUMABLE_CHUNK_SIZE {
            finish = true;
        }

        let write_request = WriteObjectRequest {
            write_offset: offset,
            first_message: Some(FirstMessage::UploadId(resumable_id.clone())),
            data: Some(Data::ChecksummedData(ChecksummedData {
                content: buffer[..n].to_vec(),
                ..Default::default()
            })),
            finish_write: finish,
            ..Default::default()
        };

        let req_stream = stream::iter(vec![write_request]);
        let mut request = Request::new(req_stream);

        request.metadata_mut().insert(
            "x-goog-request-params",
            MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
        );

        match gcs
            .get()
            .write_object(request)
            .await {
                Ok(_) => {
                    println!("Object chunk written. Finished: {}", finish);
                },
                Err(e) => {
                    panic!("Error writing object chunk. Code: {} Full response: {:?}", e.code(), e)
                }
            };

        offset += n as i64;
    }

    // List objects
    println!("-----");
    println!("Listing objects in bucket {}", bucket_id);

    let mut request = Request::new(ListObjectsRequest {
        parent: format!("projects/_/buckets/{}", bucket_id),
        ..Default::default()
    });

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
    );

    let mut objects = vec![];
    match gcs
        .get()
        .list_objects(request)
        .await {
            Ok(r) => {
                println!("Objects found in bucket {}:", bucket_id);
                for o in &r.get_ref().objects {
                    println!("{}:", o.name);
                    objects.push(o.name.clone());
                }
            },
            Err(e) => {
                panic!("Error listing objects in bucket. Code: {} Full response: {:?}", e.code(), e)
            }
        };

    // Delete object
    println!("-----");
    println!("Deleting all objects in bucket {}", bucket_id);

    for o in objects {
        let mut request = Request::new(DeleteObjectRequest {
            bucket: format!("projects/_/buckets/{}", bucket_id),
            object: o.clone(),
            ..Default::default()
        });

        request.metadata_mut().insert(
            "x-goog-request-params",
            MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
        );

        match gcs
            .get()
            .delete_object(request)
            .await {
                Ok(_) => {
                    println!("Object {} deleted.", o);
                },
                Err(e) => {
                    panic!("Error deleting object {}. Code: {} Full response: {:?}", o, e.code(), e)
                }
            };
    }

    // Delete bucket
    println!("-----");
    println!("Deleting bucket {}", bucket_id);

    let mut request = Request::new(DeleteBucketRequest {
        name: format!("projects/_/buckets/{}", bucket_id),
        ..Default::default()
    });

    request.metadata_mut().insert(
        "x-goog-request-params",
        MetadataValue::try_from(format!("project=projects/{}&bucket=projects/_/buckets/{}", PROJECT_ID, bucket_id)).unwrap(),
    );

    match gcs
        .get()
        .delete_bucket(request)
        .await {
            Ok(r) => {
                println!("Bucket {} deleted", bucket_id);
            },
            Err(e) => {
                panic!("Error deleting bucket. Code: {} Full response: {:?}", e.code(), e)
            }
        };
}
