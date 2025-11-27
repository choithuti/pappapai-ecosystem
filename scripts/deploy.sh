#!/bin/bash
echo "Deploying Pappap AI Chain Genesis Node 0276..."
git pull origin main
cargo build --release
systemctl restart pappap
echo "Pappap đang sống tại https://pappapai.xyz"