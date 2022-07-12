set dotenv-load := false

project_name := `grep APP_NAME .env | cut -d '=' -f 2-`
version := `sed -En 's/version[[:space:]]*=[[:space:]]*"([^"]+)"/\1/p' Cargo.toml | head -1`
docker_registry := `grep DOCKER_REGISTRY .env | cut -d '=' -f2`
project_image := docker_registry+"/"+project_name

default: (dev)


clean:
	rm -rf target

dev:
    cargo watch -x run


docker:
    docker build --target runtime -t {{project_name}}:{{version}} . 
    docker tag {{project_name}}:{{version}} {{project_name}}:latest


upload: docker
	docker tag {{project_name}}:{{version}} {{project_image}}:{{version}}
	docker push {{project_image}}:{{version}}


docker-compose: docker
	docker compose up --build
