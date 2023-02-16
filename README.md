# cleaning-social-media

## Main goal
Repository that will be use for cleaning all publication and information in the social media


### Steps to reach the goal
1. Build the strcuture cli in rust
2. create a way to the user can login
2. save in cache the session
2. Read publication on twitter
3. List all publications in cli
3. user can interact with the cli with the publications
4. Cache the result
5. Choose dates when the user want to delete
5. give the user the posibiity to delete a porchion of the posts
6. read the likes
7. remove the likes the user has
8. list the followers
9. create an UI to do all the implementation

----

### Requirements

version of the rust
version of the docker


### For installing run the following steps

``` shell
    cp .env.examples .env
    # edit your env
    docker compose build
    docker compose run

    docker exec -it name-of-the-cointer bash
    source .env
    cargo run test test/test.txt
```


