# Python samples for Indy SDK

Some of the samples like `ledger.py`, `getting_started.py` and `main.py` requires validators running. It expects the validators to be either at localhost (127.0.0.1) or an IP configured at environment varaible `TEST_POOL_IP`. There is a validator pool that can be used which resides at `doc/getting-started/docker-compose.yml`, which runs at IP 10.0.0.2. 
