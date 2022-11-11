# Rusty Journal

A super simple todo list CLI

## Using split

There is no official Rust SDK at the time of writing therefore using the Split Evaluator is the next best thing. Ensure you have this setup and running to make use of http://split.io

From: https://github.com/splitio/split-evaluator

You can pull the Docker image from Docker Hub and run it into your container environment.
```
docker pull splitsoftware/split-evaluator:latest
```

Run the image:
```
docker run --rm --name split-evaluator \
 -p 7548:7548 \
 -e SPLIT_EVALUATOR_API_KEY=<YOUR-SDK-API-KEY> \
 -e SPLIT_EVALUATOR_AUTH_TOKEN=<YOUR-AUTH-TOKEN> \
 splitsoftware/split-evaluator
 ```

Please refer to our official docs to learn about all the functionality provided by Split Evaluator and the configuration options available for tailoring it to your current application setup.