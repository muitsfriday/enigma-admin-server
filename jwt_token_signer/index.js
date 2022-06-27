const fs = require('fs');
const jwt = require('jsonwebtoken');

const args = process.argv.slice(2);

const payload = fs
    .readFileSync(args[0])
    .toString();

const secretOrPrivateKey = fs
    .readFileSync(args[1])
    .toString()
    // required to handle newlines at the end of file, otherwise jsonwebtoken
    // doesn't like it!
    .replace(/\n$/, '');

const algorithm = args[2] || 'HS256';

console.log(jwt.sign(payload, secretOrPrivateKey, { algorithm: algorithm }));