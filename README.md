# enigma-admin-server
API server that manage the statistical experiment data.

## Local development
Start develop locally by active following command.
```
docker-compose up
```

Test the API by visit the `request.http` file and edit the JWT token (see how to generate the token for local development below).
```
@jwt_token = REPLACE_JWT_VALUE_HERE
``` 


## Generate JWT Token for locally test.
Go to folder `jwt_token_signer` and fun the following command.
```
npm install
npm run generate:token
```

## Custom Claim
To defined the custom claim struct for your JWT
See `/src/handler/mod.rs` search for `Claim` struct and modify it to match up with your JWT data.

