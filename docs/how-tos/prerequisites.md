# Prerequisites

All how-tos assume that the developer has three things:

1. A functional developer environment, including an IDE/compiler for their
language of choice
2. A version of libindy that's built, installed, and callable in the
system path.
3. A running indy pool.

Instructions for #2 (building and installing libindy) can be found [here](../../README.md#installation). Note that we
recommend building the SDK to eliminate versioning problems; pre-built binaries
may be slightly stale.

Instructions for running an indy pool can be found [here](../../README.md#how-to-start-local-nodes-pool-with-docker).

## Wrapper's specific prerequisites

# C#
Additionally (and depending on your environment), you will need .NET installed. These demos were tested with .NET Core 2.1.302.

# Nodejs
Install all dependencies running `npm install`.

# Python
Ensure you have the 64-bit version of Python 3 installed, as the 32-bit version may have problems loading the Indy .dll files.

Install the required python packages by executing: `$ pip install python3-indy asyncio`