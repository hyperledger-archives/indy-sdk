# Python samples for Indy SDK

Each sample can be run individually with `python -m src.<filename without extension>`.

`python -m src.main` will run all samples sequentially.

Some of the samples like `ledger.py`, `getting_started.py` and `main.py` require validators running. They expect the validators to be running either at localhost (127.0.0.1) or at an IP configured in the environment variable `TEST_POOL_IP`. There is a validator pool that can be used which resides at `doc/getting-started/docker-compose.yml`, which runs at IP 10.0.0.2.
