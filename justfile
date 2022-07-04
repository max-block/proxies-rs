set dotenv-load := false

project_name := `grep APP_NAME .env | cut -d '=' -f 2-`
version := "latest"


clean:
	rm -rf target

dev:
    cargo watch -x run


docker:
    docker build --target runtime -t {{project_name}}:{{version}} . 


docker-compose: docker
	docker compose up --build
