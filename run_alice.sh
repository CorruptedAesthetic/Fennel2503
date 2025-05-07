#!/bin/bash
cd /home/neurosx/WORKING_WORKSPACE/StandaloneSolochain2503/solochain
./target/release/solochain-template-node \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --rpc-port 9944 \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --validator \
  --unsafe-force-node-key-generation 