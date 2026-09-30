# HPC Cluster Seeder

This one-time service loads configured HPC clusters only when the `HPC_CLUSTERS` MongoDB collection does not already exist. Seed configuration omits domain identities; the `HpcCluster` aggregate generates UUIDv7 identifiers for clusters and embedded queues.
