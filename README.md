### Bank API
A test application to call Open Banking API


### Usage
#### First change the parameters in the .env file
#### Install ngrok to enable the callback request to get the code
```bash
#Run ngrok
ngrok http 3000

# SQLX CLI install
cargo install --git https://github.com/launchbadge/sqlx sqlx-cli

# docker-composer to start local test postgres database
docker-compose up -d

# Export DATABASE_URL
DATABASE_URL=postgres://bank-api:bank-api@localhost:5432/bank-api

# Run sqlx commands to create local database
sqlx database create
sqlx mig run

#Run ngrok
ngrok http 3000

# Run the server (http://localhost:300)
cargo run
```

### To make API requests
- Health endpoint `GET` /
```
curl http://localhost:3000/
```
- Create a user account `POST` /signup
```
curl --request POST \
    --url http://localhost:3000/signup \
    --header 'content-type: application/json' \
    --data '{
        "username": "john",
        "email": "john@example.com",
        "password": "doe"
    }'
```
- Authentication and the link to reques the code `POST` /auth
```
curl --request POST \
    --url http://localhost:3000/auth \
    --user john
```
- User profile: `GET` /me
  ```
  curl --request GET \
  --url http://localhost:3000/me \
  --header 'authorization: Bearer <jwt_token>'

- Get user bank transactions: `GET` /transactions
  ```
  curl --request GET \
  --url http://localhost:3000/transactions \
  --header 'authorization: Bearer <jwt_token>'





