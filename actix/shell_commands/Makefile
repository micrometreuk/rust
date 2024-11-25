.PHONY: run
dev:
	cargo watch -x run
clean: 
	rm static/images/*.jpg

start:
	docker compose up -d 

stop:
	docker compose down 

update:
	docker compose down 
	docker compose pull
	docker compose up -d --build

restart: 
	docker compose restart

remove:
	docker compose down -v
	docker compose rm -f
ffmpeg_serve:
	ffmpeg -i  static/videos/alprVideo1.mp4 -listen 1 -f mp4 -movflags frag_keyframe+empty_moov  http://127.0.0.1:5001 -stream_loop -9
clean:
	rm static/images/*.jpg
