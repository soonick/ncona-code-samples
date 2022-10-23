# Introduction to Elasticsearch

To run the samples in the examples we will need `docker`.

## Start elasticsearch

```
docker network create elastic
docker run --rm -it --name es01 --net elastic -p 9200:9200 -p 9300:9300 -it docker.elastic.co/elasticsearch/elasticsearch:8.4.3
```

We will get a message that includes something like this:

```bash
->  Password for the elastic user (reset with `bin/elasticsearch-reset-password -u elastic`):
  GzAfOPy=u5jbG1rAmTOD
```

Copy the password somewhere safe. We'll need it later.

To verify the cluster is running correctly, we can use these commands:

```bash
ES_PASS=<password from previous step>
docker cp es01:/usr/share/elasticsearch/config/certs/http_ca.crt .
curl --cacert http_ca.crt -u elastic:$ES_PASS https://localhost:9200
```

If everything goes well we will get something like this as a result:

```json
{
  "name" : "5c455685428d",
  "cluster_name" : "docker-cluster",
  "cluster_uuid" : "QNUDuJcDQuiXElhYb7FP3A",
  "version" : {
    "number" : "8.4.3",
    "build_flavor" : "default",
    "build_type" : "docker",
    "build_hash" : "42f05b9372a9a4a470db3b52817899b99a76ee73",
    "build_date" : "2022-10-04T07:17:24.662462378Z",
    "build_snapshot" : false,
    "lucene_version" : "9.3.0",
    "minimum_wire_compatibility_version" : "7.17.0",
    "minimum_index_compatibility_version" : "7.0.0"
  },
  "tagline" : "You Know, for Search"
}
```

## Working with indices

Before we create an index, let's list all the indices currently in the cluster:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  'https://localhost:9200/_stats/indexing?pretty=true'
```

We will get something like this back:

```json
{
  "_shards" : {
    "total" : 0,
    "successful" : 0,
    "failed" : 0
  },
  "_all" : {
    "primaries" : { },
    "total" : { }
  },
  "indices" : { }
}
```

We can see that there is nothing under the `indices` key.

Now, let's create an index:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X PUT \
  https://localhost:9200/indexone?pretty
```

The command creates an index named `indexone`. The result will be something like this:

```json
{
  "acknowledged" : true,
  "shards_acknowledged" : true,
  "index" : "indexone"
}
```

We can list the indices again to see that the index is now there. The result will be something like this:

```json
{
  ...
  "indices" : {
    "indexone" : {
      "uuid" : "v3ZM6TSqTPS7bkNHUEJMow",
      "health" : "yellow",
      "status" : "open",
      "primaries" : {
        "indexing" : {
          "index_total" : 0,
          "index_time_in_millis" : 0,
          "index_current" : 0,
          "index_failed" : 0,
          "delete_total" : 0,
          "delete_time_in_millis" : 0,
          "delete_current" : 0,
          "noop_update_total" : 0,
          "is_throttled" : false,
          "throttle_time_in_millis" : 0
        }
      },
      "total" : {
        "indexing" : {
          "index_total" : 0,
          "index_time_in_millis" : 0,
          "index_current" : 0,
          "index_failed" : 0,
          "delete_total" : 0,
          "delete_time_in_millis" : 0,
          "delete_current" : 0,
          "noop_update_total" : 0,
          "is_throttled" : false,
          "throttle_time_in_millis" : 0
        }
      }
    }
  }
}
```

We can delete the index using this command:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X DELETE \
  https://localhost:9200/indexone?pretty
```

## Working with documents

To create a document with a random id:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "carlos",
        "age": 4
      }' \
  'https://localhost:9200/indextwo/_doc?pretty'
```

We will get this as result:

```json
{
  "_index" : "indextwo",
  "_id" : "MXtdBYQB1Lo3Pw9zqFEw",
  "_version" : 1,
  "result" : "created",
  "_shards" : {
    "total" : 2,
    "successful" : 1,
    "failed" : 0
  },
  "_seq_no" : 0,
  "_primary_term" : 1
}
```

To save a document with a specific id, we can use this command:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "adrian",
        "age": 55
      }' \
  'https://localhost:9200/indextwo/_doc/1234?pretty'
```

Let's now get a document by its id:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  'https://localhost:9200/indextwo/_doc/1234?pretty'
```

The result looks like this:

```json
{
  "_index" : "indextwo",
  "_id" : "1234",
  "_version" : 1,
  "_seq_no" : 1,
  "_primary_term" : 1,
  "found" : true,
  "_source" : {
    "name" : "adrian",
    "age" : 55
  }
}
```

To update a document:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X PUT \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "fernando",
        "age": 99
      }' \
  'https://localhost:9200/indextwo/_doc/1234?pretty'
```

The result will look like this:

```bash
{
  "_index" : "indextwo",
  "_id" : "1234",
  "_version" : 2,
  "result" : "updated",
  "_shards" : {
    "total" : 2,
    "successful" : 1,
    "failed" : 0
  },
  "_seq_no" : 4,
  "_primary_term" : 1
}
```

To delete a document we can:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X DELETE \
  -H 'Content-Type: application/json' \
  'https://localhost:9200/indextwo/_doc/1234?pretty'
```

The result will be something like:

```json
{
  "_index" : "indextwo",
  "_id" : "1234",
  "_version" : 3,
  "result" : "deleted",
  "_shards" : {
    "total" : 2,
    "successful" : 1,
    "failed" : 0
  },
  "_seq_no" : 2,
  "_primary_term" : 1
}
```

## Searching

Let's add some documents so we can search through them:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "The best book in the world",
        "category": "books",
        "description": "The best book in the world talks about many things that everybody finds interesting"
      }' \
  'https://localhost:9200/products/_doc/1?pretty'

curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "Just average",
        "category": "books",
        "description": "This book can entertain you for a bit, but there are better ones"
      }' \
  'https://localhost:9200/products/_doc/2?pretty'


curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "Cleaner robot",
        "category": "electronics",
        "description": "Cleans the house, does laundry, washes dishes, irons"
      }' \
  'https://localhost:9200/products/_doc/3?pretty'

curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
        "name": "Smart speaker",
        "category": "electronics",
        "description": "Plays ads, music and can talk to you when you feel lonely"
      }' \
  'https://localhost:9200/products/_doc/4?pretty'
```

To get all products we can use this command:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
        "query": {
            "match_all": {}
        }
    }' \
  'https://localhost:9200/products/_search?pretty'
```

As expected, we get all products:

```json
{
  "took" : 17,
  "timed_out" : false,
  "_shards" : {
    "total" : 1,
    "successful" : 1,
    "skipped" : 0,
    "failed" : 0
  },
  "hits" : {
    "total" : {
      "value" : 4,
      "relation" : "eq"
    },
    "max_score" : 1.0,
    "hits" : [
      {
        "_index" : "products",
        "_id" : "1",
        "_score" : 1.0,
        "_source" : {
          "name" : "The best book in the world",
          "category" : "books",
          "description" : "The best book in the world talks about many things that everybody finds interesting"
        }
      },
      {
        "_index" : "products",
        "_id" : "2",
        "_score" : 1.0,
        "_source" : {
          "name" : "Just average",
          "category" : "books",
          "description" : "This book can entertain you for a bit, but there are better ones"
        }
      },
      {
        "_index" : "products",
        "_id" : "3",
        "_score" : 1.0,
        "_source" : {
          "name" : "Cleaner robot",
          "category" : "electronics",
          "description" : "Cleans the house, does laundry, washes dishes, irons"
        }
      },
      {
        "_index" : "products",
        "_id" : "4",
        "_score" : 1.0,
        "_source" : {
          "name" : "Smart speaker",
          "category" : "electronics",
          "description" : "Plays ads, music and can talk to you when you feel lonely"
        }
      }
    ]
  }
}
```

To get only the documents where `category = books`:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
        "query": {
          "match": {
            "category": "books"
          }
        }
      }' \
  'https://localhost:9200/products/_search?pretty'
```

The result:

```json
{
  "took" : 62,
  "timed_out" : false,
  "_shards" : {
    "total" : 1,
    "successful" : 1,
    "skipped" : 0,
    "failed" : 0
  },
  "hits" : {
    "total" : {
      "value" : 2,
      "relation" : "eq"
    },
    "max_score" : 0.6931471,
    "hits" : [
      {
        "_index" : "products",
        "_id" : "1",
        "_score" : 0.6931471,
        "_source" : {
          "name" : "The best book in the world",
          "category" : "books",
          "description" : "The best book in the world talks about many things that everybody finds interesting"
        }
      },
      {
        "_index" : "products",
        "_id" : "2",
        "_score" : 0.6931471,
        "_source" : {
          "name" : "Just average",
          "category" : "books",
          "description" : "This book can entertain you for a bit, but there are better ones"
        }
      }
    ]
  }
}
```

If we try to search for documents with books in the description:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
        "query": {
          "match": {
            "description": "books"
          }
        }
      }' \
  'https://localhost:9200/products/_search?pretty'
```

We get nothing back:

```
{
  "took" : 3,
  "timed_out" : false,
  "_shards" : {
    "total" : 1,
    "successful" : 1,
    "skipped" : 0,
    "failed" : 0
  },
  "hits" : {
    "total" : {
      "value" : 0,
      "relation" : "eq"
    },
    "max_score" : null,
    "hits" : [ ]
  }
}
```

If we make the search fuzzy:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
        "query": {
          "match": {
            "description": {
              "query": "books",
              "fuzziness": "AUTO"
            }
          }
        }
      }' \
  'https://localhost:9200/products/_search?pretty'
```

Then we get some results:

```json
{
  "took" : 115,
  "timed_out" : false,
  "_shards" : {
    "total" : 1,
    "successful" : 1,
    "skipped" : 0,
    "failed" : 0
  },
  "hits" : {
    "total" : {
      "value" : 2,
      "relation" : "eq"
    },
    "max_score" : 0.4981795,
    "hits" : [
      {
        "_index" : "products",
        "_id" : "2",
        "_score" : 0.4981795,
        "_source" : {
          "name" : "Just average",
          "category" : "books",
          "description" : "This book can entertain you for a bit, but there are better ones"
        }
      },
      {
        "_index" : "products",
        "_id" : "1",
        "_score" : 0.48209476,
        "_source" : {
          "name" : "The best book in the world",
          "category" : "books",
          "description" : "The best book in the world talks about many things that everybody finds interesting"
        }
      }
    ]
  }
}
```

To search multiple fields we can use something like this:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
        "query": {
          "bool": {
            "must": [
              {
                "match": {
                  "category": "books"
                }
              },
              {
                "match": {
                  "name": "average"
                }
              }
            ]
          }
        }
      }' \
  'https://localhost:9200/products/_search?pretty'
```

The result will be this:

```json
{
  "took" : 80,
  "timed_out" : false,
  "_shards" : {
    "total" : 1,
    "successful" : 1,
    "skipped" : 0,
    "failed" : 0
  },
  "hits" : {
    "total" : {
      "value" : 1,
      "relation" : "eq"
    },
    "max_score" : 2.087221,
    "hits" : [
      {
        "_index" : "products",
        "_id" : "2",
        "_score" : 2.087221,
        "_source" : {
          "name" : "Just average",
          "category" : "books",
          "description" : "This book can entertain you for a bit, but there are better ones"
        }
      }
    ]
  }
}
```
