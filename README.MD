# Description 
If you are looking to simply check if a node is publically available use Max Indens wonderful Libp2p-lookup [here](https://github.com/mxinden/libp2p-lookup) :) 

This project is simply for my personal interests but feel free to use if you'd like. It requires a list of Multiaddrs to dial. The main usage for the time being is for testing and pruning of the peerlist. 

ToDo:
- Generate Default PeerList
- Optional get ipfs config.json 

# Usage 
To use specify a .yaml file with a list of Multiaddrs like so 

````
---
addrs: ["/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
		"/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
		"/dnsaddr/bootstrap.libp2p.io/p2p/QmbLHAnMoJPWSCR5Zhtx6BHJX9KiKNN6tpvbUcqanj75Nb",
		"/dnsaddr/bootstrap.libp2p.io/p2p/QmcZf59bWwK5XFi76CZX8cbJ4BhTzzA3gU1ZjYZcYW3dwt",
		"/ip4/104.131.131.82/tcp/4001/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ",
		"/ip4/104.131.131.82/udp/4001/quic/p2p/QmaCpDMGvV2BGHeYERUEnRQAwe3N8SzbUtfsmvsqQLuvuJ"]
````
Then run

```cargo run -- --file "FILENAME.yaml" ```

