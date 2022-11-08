# Searching related documents with Elasticsearch

To run the samples in the examples we will need `docker`.

## Start elasticsearch

```
docker network create elastic
docker run --rm -it --name es01 \
  --net elastic -p 9200:9200 -p 9300:9300 \
  -e "ES_JAVA_OPTS=-Xms1024m -Xmx1024m" \
  -it docker.elastic.co/elasticsearch/elasticsearch:8.4.3
```

We will get a message that includes something like this:

```bash
->  Password for the elastic user (reset with `bin/elasticsearch-reset-password -u elastic`):
  GzAfOPy=u5jbG1rAmTOD
```

Copy the password somewhere safe. We'll need it for the following curl requests.

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

## A problem with denormalized data

Create a document:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "My little food spot",
    "owner": "Carlos",
    "locations": [
      {
        "address": "32 Some street name",
        "city": "Mytown",
        "country": "Some country"
      },
      {
        "address": "398 Another street",
        "city": "Bigger town",
        "country": "Some country"
      }
    ]
  }' \
  'https://localhost:9200/restaurants/_doc?pretty'
```

Search a document by `city` and `address`:

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
              "locations.city": "Mytown"
            }
          },
          {
            "match": {
              "locations.address": "Some"
            }
          }
        ]
      }
    }
  }' \
  'https://localhost:9200/restaurants/_search?pretty'
```

We get the expected result:

```json
{
  "took" : 4,
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
    "max_score" : 0.970927,
    "hits" : [
      {
        "_index" : "restaurants",
        "_id" : "7zBcRYQB1JvA4r1h7mXr",
        "_score" : 0.970927,
        "_source" : {
          "name" : "My little food spot",
          "owner" : "Carlos",
          "locations" : [
            {
              "address" : "32 Some street name",
              "city" : "Mytown",
              "country" : "Some country"
            },
            {
              "address" : "398 Another street",
              "city" : "Bigger town",
              "country" : "Some country"
            }
          ]
        }
      }
    ]
  }
}
```

Now, let's try to find a restaurant that has an `address` that contains `Some`, but this time in `Bigger town`:

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
              "locations.city": "Bigger town"
            }
          },
          {
            "match": {
              "locations.address": "Some"
            }
          }
        ]
      }
    }
  }' \
  'https://localhost:9200/restaurants/_search?pretty'
```

We get an incorrect result:

```json
{
  "took" : 4,
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
    "max_score" : 1.258609,
    "hits" : [
      {
        "_index" : "restaurants",
        "_id" : "7zBcRYQB1JvA4r1h7mXr",
        "_score" : 1.258609,
        "_source" : {
          "name" : "My little food spot",
          "owner" : "Carlos",
          "locations" : [
            {
              "address" : "32 Some street name",
              "city" : "Mytown",
              "country" : "Some country"
            },
            {
              "address" : "398 Another street",
              "city" : "Bigger town",
              "country" : "Some country"
            }
          ]
        }
      }
    ]
  }
}
```

## Fixing the problem

Create new index with a mapping. Notice the `"type": "nested"` property:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X PUT \
  -H 'Content-Type: application/json' \
  -d '{
        "mappings": {
          "properties": {
            "name": { "type": "text" },
            "owner": { "type": "text" },
            "locations": {
              "type": "nested",
              "properties": {
                "country": { "type": "text" },
                "city": { "type": "text" },
                "address": { "type": "text" }
              }
            }
          }
        }
      }' \
  https://localhost:9200/restaurants-nested?pretty
```

Create the same document as before, but in this new index:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "My little food spot",
    "owner": "Carlos",
    "locations": [
      {
        "address": "32 Some street name",
        "city": "Mytown",
        "country": "Some country"
      },
      {
        "address": "398 Another street",
        "city": "Bigger town",
        "country": "Some country"
      }
    ]
  }' \
  'https://localhost:9200/restaurants-nested/_doc?pretty'
```

Search for the restaurant:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "match": {
        "owner": "Carlos"
      }
    }
  }' \
  'https://localhost:9200/restaurants-nested/_search?pretty'
```

And we will find it:

```json
{
  "took" : 5,
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
    "max_score" : 0.2876821,
    "hits" : [
      {
        "_index" : "restaurants-nested",
        "_id" : "KB8VVYQBmF8fzs58LwT1",
        "_score" : 0.2876821,
        "_source" : {
          "name" : "My little food spot",
          "owner" : "Carlos",
          "locations" : [
            {
              "address" : "32 Some street name",
              "city" : "Mytown",
              "country" : "Some country"
            },
            {
              "address" : "398 Another street",
              "city" : "Bigger town",
              "country" : "Some country"
            }
          ]
        }
      }
    ]
  }
}
```

Search using for a locations field will not find anything

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "match": {
        "locations.city": "Mytown"
      }
    }
  }' \
  'https://localhost:9200/restaurants-nested/_search?pretty'
```

For it to work, we need to use the `nested` argument:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "nested": {
        "path": "locations",
        "query": {
          "match": {
            "locations.city": "Mytown"
          }
        }
      }
    }
  }' \
  'https://localhost:9200/restaurants-nested/_search?pretty'
```

Will return the restaurant:

```json
{
  "took" : 8,
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
    "max_score" : 0.8025915,
    "hits" : [
      {
        "_index" : "restaurants-nested",
        "_id" : "KB8VVYQBmF8fzs58LwT1",
        "_score" : 0.8025915,
        "_source" : {
          "name" : "My little food spot",
          "owner" : "Carlos",
          "locations" : [
            {
              "address" : "32 Some street name",
              "city" : "Mytown",
              "country" : "Some country"
            },
            {
              "address" : "398 Another street",
              "city" : "Bigger town",
              "country" : "Some country"
            }
          ]
        }
      }
    ]
  }
}
```

We can also search multiple fields like we did before:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "nested": {
        "path": "locations",
        "query": {
          "bool": {
            "must": [
              {
                "match": {
                  "locations.city": "Mytown"
                }
              },
              {
                "match": {
                  "locations.address": "Some"
                }
              }
            ]
          }
        }
      }
    }
  }' \
  'https://localhost:9200/restaurants-nested/_search?pretty'
```

But this time, we can use the following match and it won't return any results:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "nested": {
        "path": "locations",
        "query": {
          "bool": {
            "must": [
              {
                "match": {
                  "locations.city": "Mytown"
                }
              },
              {
                "match": {
                  "locations.address": "Another"
                }
              }
            ]
          }
        }
      }
    }
  }' \
  'https://localhost:9200/restaurants-nested/_search?pretty'
```

Returns nothing because there is no location with city matching `Mytown` and address matching `Another`.

## Application side joins

Let's add two movies in a new index:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "The Machinist",
    "release_year": 2004,
    "director": "Brad Anderson"
  }' \
  'https://localhost:9200/movies/_doc/1?pretty'

curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "American Psycho",
    "release_year": 2000,
    "director": "Mary Harron"
  }' \
  'https://localhost:9200/movies/_doc/2?pretty'
```

And three actors in a different index:

```bash
curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Christian Bale",
    "birth_year": 1974,
    "birth_country": "United Kingdom",
    "movies": [1, 2]
  }' \
  'https://localhost:9200/actors/_doc/1?pretty'

curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Jennifer Jason Leigh",
    "birth_year": 1962,
    "birth_country": "United States",
    "movies": [1]
  }' \
  'https://localhost:9200/actors/_doc/2?pretty'

curl --cacert http_ca.crt -u elastic:$ES_PASS \
  -X POST \
  -H 'Content-Type: application/json' \
  -d '{
    "name": "Reese Witherspoon",
    "birth_year": 1976,
    "birth_country": "United States",
    "movies": [2]
  }' \
  'https://localhost:9200/actors/_doc/3?pretty'
```

Notice how actors have links to movies.

To find all the movies that have actors that were born in the United States:

```
bash ./application-side-joins.sh $ES_PASS
```

And the result should be both movies:

```json
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
      "value" : 2,
      "relation" : "eq"
    },
    "max_score" : 1.0,
    "hits" : [
      {
        "_index" : "movies",
        "_id" : "2",
        "_score" : 1.0,
        "_source" : {
          "name" : "American Psycho",
          "release_year" : 2000,
          "director" : "Mary Harron"
        }
      },
      {
        "_index" : "movies",
        "_id" : "1",
        "_score" : 1.0,
        "_source" : {
          "name" : "The Machinist",
          "release_year" : 2004,
          "director" : "Brad Anderson"
        }
      }
    ]
  }
}
```
