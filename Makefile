PHONY := all
all:
	docker compose up --build -d

PHONY += down
down:
	docker compose down

PHONY += re
re: down all

PHONY += clean
clean:
	rm -rf ./debug a.out tmp.s

PHONY += test
test:
	./test.sh
