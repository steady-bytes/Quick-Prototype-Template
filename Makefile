##############
# ROOT SCRIPTS

#######################
### SQL DEVELOPMENT ### 
#######################

# db configuration variables
DB_NAME=application
DB_USER=application
DB_PASSWORD=password
DB_NETWORK=localhost
DB_SERVER_NAME=application-sql-server

####################
### database targets 
.PHONY: db-image db-start-server db-migrate-up db-terminate db-stop-server db-run

# start a local database, migrate up to the current migration
db-run: db-start-server wait db-migrate-up

# build docker images for database
db-image:
	cd ./deployments/sql && docker build -t $(DB_SERVER_NAME) .

# start the local database server
db-start-server:
	docker run --name $(DB_SERVER_NAME) -p 5432:5432 \
	 	-e POSTGRES_DB=$(DB_NAME) \
		-e POSTGRES_PASSWORD=$(DB_PASSWORD) \
		-e POSTGRES_USER=$(DB_USER) \
		-d $(DB_SERVER_NAME)

# run the db migration tool
db-migrate-up:
	cd migration && cargo run -- up

db-migrate-down:
	sea-orm-cli migrate down

### Terminate - the whole db stack, it's configuration, and data
db-terminate: db-stop-server db-clean

# stop dev database
db-stop-server:
	docker stop $(DB_SERVER_NAME)

## cleans volumn
db-clean:
	docker rm $(DB_SERVER_NAME)

#####################
### Local development
.PHONY: development clean

# run `$ make development` to run the db, and server locally
# todo - add command to run the server
development: db-run watch-server

# nuke your local dev state, and start from scratch
clean: db-terminate

# deploy the database server, run migrations, generate local config, build server image, and run the server
# If your working on one of the front-end applications this is a good place to get the back-end system up and
# running. All you need is docker on your local system.	

##################
### server targets 
.PHONY: watch-server

# use cargo watch to reload the server when changes have been made
watch-server:
	export RUST_LOG=info && cargo watch -x 'run --bin server server'

##################
### client targets 
.PHONY: watch-client

watch-client:
	cd dashboard && trunk serve --address=0.0.0.0