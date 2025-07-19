# Session-1: Introduction to Solana

## Terminologies
- **Nakamoto Coefficient:** Minimum number of validators needed to control the network (Solana's coefficient is 22).
- **True TPS:** Maximum number of non-validator transactions processed per second (Solana: 1000-1300).
- **Block Time:** Time taken to produce a block (Solana: 400-500ms).
- **Time to Finality:** Time for a transaction to become final and irreversible (Solana: ~12 seconds).
- **Validator:** Node that validates transactions and produces blocks (~1200 validators in Solana).
- **Leader:** Validator responsible for producing a block in a given slot.
- **Multiple Validators Client:** Client connecting to multiple validators (Examples: Agave, Jito, Firedancer).
- **RPC:** Remote Procedure Call protocol for client-blockchain interaction.
- **RPC Node:** Node providing RPC services.
- **RPC Endpoint:** URL for clients to connect to an RPC node.

## What is Solana?
Solana is a high-performance Layer 1 blockchain designed to provide fast and secure transactions at scale. It uses a unique combination of technologies, including Proof of History (PoH), Proof of Stake (PoS), and parallel transaction processing, to achieve high throughput and low latency. Solana's architecture allows it to process thousands of transactions per second, making it one of the fastest blockchains in the world.

## Why Solana is Fast?
When a Transaction is made through the wallet on any dApp, the wallet sends the transaction to the RPC node. The RPC node then forwards the transation to the validator. The validator then processes the transaction and produces a block. As there are multiple validators in the network, the transaction can be processed in parallel, allowing for high throughput and low latency. The RPC node then sends the response back to the wallet, which updates the user interface accordingly. the transaction changes the state of the blockchain, which is then propagated to all the validators in the network. This allows for a fast and efficient transaction processing system.

## Proof of History (PoH)
PoH is not a consensus mechanism but a cryptographic clock that allows validators to agree on the order of transactions without having to communicate with each other. It is a way to create a historical record that proves that an event has occurred at a specific time. PoH is used in conjunction with the Proof of Stake (PoS) consensus mechanism to provide a fast and efficient way to process transactions. In short PoH is a way to create a verifiable and tamper-proof record of the order of transactions in the Solana network (synchronization mechanism). It allows validators to process transactions in parallel, which is a key factor in Solana's high throughput and low latency.

## Gulf Stream
Gulf Stream is a transaction forwarding protocol that allows validators to process transactions in parallel. It is a key component of Solana's architecture that enables high throughput and low latency. Gulf Stream allows validators to forward transactions to each other without having to wait for a block to be produced. This means that transactions can be processed in parallel, allowing for faster transaction processing times. Gulf Stream also allows validators to prioritize transactions based on their fees, which helps to ensure that high-fee transactions are processed quickly.

## Leader Schedule
A epoch is a period of time in which a specific set of validators are responsible for producing blocks in the Solana network. The leader schedule is a list of validators that are responsible for producing blocks during a specific epoch. The leader schedule is determined by the Proof of Stake (PoS) consensus mechanism, which allows validators to stake their tokens to participate in the consensus process. The leader schedule is updated periodically, allowing different validators to take turns producing blocks and ensuring that the network remains decentralized and secure.

## Stake Weighted Quality of Service (SWQoS)
Introduced in Solana's version 1.14, Stake Weighted Quality of Service (SWQoS) is a mechanism that allows validators to prioritize transactions based on the amount of stake they have. This means that transactions from validators with more stake are given higher priority, allowing them to be processed faster. SWQoS helps to ensure that high-value transactions are processed quickly, while also maintaining a fair and decentralized network. For example, 80% conections to leader are given to validators with more stake, while 20% connections are given to open validators. This helps to ensure that the network remains decentralized and secure, while also allowing for fast and efficient transaction processing.

## Quick UDP Internet Connections (QUIC)
QUIC is a transport layer protocol that is used by Solana to provide fast and secure communication between validators and clients. It is designed to reduce latency and improve the performance of the network by allowing for faster connection establishment and data transfer. QUIC is used in conjunction with the Remote Procedure Call (RPC) protocol to provide a fast and efficient way for clients to interact with the Solana network.

## How Solana performs parallel transaction processing?
Solana achieves parallel transaction processing through a mechanism called "Sealevel," its unique runtime for smart contracts. Sealevel allows Solana to execute thousands of smart contract transactions simultaneously by analyzing which accounts each transaction reads from and writes to. If transactions access different accounts, they can be processed in parallel without conflicts. This is possible because Solana’s runtime enforces strict account-based isolation, preventing data races and ensuring consistency. By leveraging this parallelism, Solana dramatically increases throughput and reduces latency compared to blockchains that process transactions sequentially.

## Alpenglow : https://www.helius.dev/blog/alpenglow
 Alpenglow is a new feature introduced in Solana's version 1.14 that allows for faster transaction processing and improved network performance. It is designed to reduce the time it takes for a transaction to be considered final and irreversible in the network, allowing for faster and more efficient transaction processing. With this the time to finality will be reduced to 150 milliseconds.