movies=$(curl --cacert http_ca.crt -u elastic:$1 \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "match": {
        "birth_country": "United States"
      }
    }
  }' \
  'https://localhost:9200/actors/_search?pretty' |
  jq .hits.hits[]._source.movies |
  grep -v '\[' |
  grep -v '\]' |
  sed -e 's/\,//g' |
  sed -e 's/ //g' |
  sort |
  uniq)

movies_json='['
for movie in $movies
do
  movies_json="${movies_json}\"${movie}\","
done
movies_json="${movies_json::-1}]"

echo "Convered to json array: $movies_json"

curl --cacert http_ca.crt -u elastic:$1 \
  -X GET \
  -H 'Content-Type: application/json' \
  -d '{
    "query": {
      "terms": {
        "_id": '$movies_json'
      }
    }
  }' \
  'https://localhost:9200/movies/_search?pretty'
