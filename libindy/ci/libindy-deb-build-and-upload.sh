#!/bin/bash

 fpm --input-type "dir" \
    --verbose \
    --output-type "deb" \
    --architecture "amd64" \
    --name "indy-sdk" \
    --version "0.1.1" \
    --license "MIT/Apache-2.0" \
    --maintainer "Hyperledger <hyperledger-indy@lists.hyperledger.org>" \
    --description "This is the official SDK for Hyperledger Indy, which provides a
  distributed-ledger-based foundation for self-sovereign identity.
  The major artifact of the SDK is a c-callable library; there are
  also convenience wrappers for various programming languages.
  All bugs, stories, and backlog for this project are managed through
  Hyperledger's Jira in project IS (note that regular Indy tickets are
  in the INDY project instead...). Also, join us on Jira's Rocket.Chat
  at #indy-sdk to discuss." \
    --depends 'libssl1.0.0'\
    --depends 'libsodium18'\
    --depends 'libsqlite0'\
    --package "/home/indy/debs" \
    .