# Agent Utilities

The `alice_agent_pool.dockerfile` will set up an docker container that already has all getting started agencies configured for Alice. It uses `agents_setup.sh`, `getting_started.indyscript`, and `node_supervisord.conf`to set it all up.

To build and run the dockerfile:

``` bash
docker build -f alice_agent_pool.dockerfile -t {IMAGE_NAME} . 
docker run -itd --name {CONTAINER_NAME} -p 9701-9708:9701-9708 {IMAGE_NAME}
```
...where `{IMAGE_NAME}` and `{CONTAINER_NAME}` are names of your choice.

After that, Faber, Acme, and Thrift should all be set up and ready to go for Alice to interact with.
