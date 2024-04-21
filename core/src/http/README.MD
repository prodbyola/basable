# http module
This module contains all http REST request handlers for Basable. All examples in this documentation are written in Javascript/Typescript using [axios library](https://axios-http.com/docs/intro). However, any other client-side implementation will work in similar ways.  

### POST: /create-guest 
Creates a Basable guest `User` and returns a `JwtSession`. The created `User` is unregistered but has access to Basable protected routes via [JWT](https://en.wikipedia.org/wiki/JSON_Web_Token) token. The generated user token has a 2hrs lifespan (by default) and must be passed as `B-Session-Id` into subsequent request headers.

#### Body:
* None

#### Response:
* `token`: JWT token generated for the guest user.
* `exp`: The expiration time of the token (in seconds).

#### Example:
```js
import axios from 'axios'

const session = await axios.post('/create-guest').then(resp => resp)

console.log(session)
// {
//     "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIxMjcuMC4wLjEiLCJleHAiOjE3MTM2OTY4MTl9.mNJ7NXxF9iW8Tgqf-tt9nlUSqi2NvxcIdM1x6kcuzqpWsp2sQtcCkBa98g-ApCZqCTkz6YgA9D6WbwoBUHBNXA",
//     "exp": 7200
// }
```

### POST: /connect
Initiates a new `BasableConnection` for current user. It expects `Config` as request's body. If user is registered (not guest user), it saves the associated `Config` for them for easier subsequent access.

#### Body:
* `source_type` (required): The type of connection source. Available options are:
    * `database` for relational and other database connections. Please [see supported databases] for available source options.
    * `cloud`: for cloud-based data sources like firebase...etc.
    * `file`: for file-based data sources like `csv`, `feather`...etc

* `source` (required): The actual source of the data, depending on the `source_type` option.
* `username` (optional): The username to access the data source where applicable.
* `password` (optional): The password to access the data source where applicable.
* `host` (optional): The host url to access the data source where applicable.
* `port` (optional): The host port to access the data source where applicable.
* `db_name` (optional): The name of the database to access. Required if `data_source` is `database`.

#### Response:
Response depends on the value `source_type` in the request body.

#### Example:
```js
import axios from 'axios'

const userToken = user.token

const data = {
    source_type: 'database',
    source: 'mysql',
    username: 'root',
    password: 'dbpass'
    host: 'localhost',
    port: 3306,
    db_name: 'basable'
}

const details = await axios.post('/connect', data, {
    "B-Session-Id": userToken // use 'Authorization' header if this is a logged user.
}).then(resp => resp);

console.log(details)
// { 
//     tables: [...],
//     status: {...},
//     variables: {...}
// }
```