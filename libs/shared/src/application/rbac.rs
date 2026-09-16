pub enum Role {
    // Can perform CRUD operations on globally-scoped objects
    SiteAdmin,

    // Can perform CRUD operations on tenant-scoped objects
    TenantAdmin,

    // Manages all HpcClusters for a given datacenter
    DataCenterAdmin,

    // Manages HpcCluster queues, scheduling, and billing
    HpcClusterAdmin,
}
