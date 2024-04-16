### Solana Indexer Stack (SIS)

The Solana Indexer Stack (SIS) is a broad indexing solution for Solana. It enables dApp developers to take advantage of custom indexes that target specific programs, accounts, and transactions required by their dApp. SIS offers a framework for creating program parsers. These parsers can be assembled together to process real-time change events from Dragonmouth, converting them into program-aware structures. These structures are then stored in a database or used in other data pipelines.

**Design Objectives**

1. **Cost Efficiency**: Using Dragonmouth, multiple SIS instances can share a single geyser stream. This, combined with a variety of filter options, ensures the storage costs focus on what's essential for the dApp.
2. **Operational Simplicity**: The operation of SIS shouldn't require numerous dependent systems. It should be intuitive, demanding minimal configuration.
3. **Recovery**: In scenarios such as a crash or cold start, operators can designate a starting slot. SIS, working in conjunction with Dragonmouth, replays all transactions and accounts from the specified slot until it reaches the active slot, at which point it switches to real-time processing.
4. **Auditability**: Operators can trust the contents of the index by conducting verifiable audits. These audits check which accounts and transactions the index processed and at which slot.
5. **Observability**: Operators can monitor the health of their installation, gaining insights on aspects like lag, throughput, and error rates.
6. **Composability**: Program parsers should be developed as separate modules (cargo crates). This design will enable programs to include other program parsers needed to deserialize cross program invocations (CPI).

![diagram](/diagram.jpeg)