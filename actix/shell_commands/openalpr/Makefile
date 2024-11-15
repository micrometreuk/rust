.PHONY: run
start:
	docker compose up -d 
stop:
	docker compose down 
update:
	docker compose build --no-cache
restart:
	docker compose restart
remove:
	docker compose down -v
	docker compose rm -f
