#!/bin/bash
cd /home/neurosx/WORKING_WORKSPACE/StandaloneSolochain2503/solochain
./target/release/solochain-template-node \
  --base-path /tmp/eve \
  --chain local \
  --name Eve \
  --port 30337 \
  --rpc-port 9948 \
  --rpc-external \
  --rpc-cors all \
  --rpc-methods Unsafe \
  --validator \
  --unsafe-force-node-key-generation \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWDkxfuBaCuX9cBQqpQN1RVHNL3uqaAUpUidCP7BaDgo1F

# Alice's node ID: 12D3KooWDkxfuBaCuX9cBQqpQN1RVHNL3uqaAUpUidCP7BaDgo1F 