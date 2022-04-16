This project uses Docker and Docker-Compose. If you don't have it already, you can get it frome here: https://docs.docker.com/compose/install/

To run the project execute the following command from this folder:

```
docker-compose up
```

The whoami server will be automatically detected by traefik and requests to it will be automatically forwarded:

```
curl -H Host:example.com http://127.0.0.1/whoami/
```
