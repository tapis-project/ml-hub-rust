const cur = db.MODEL_METADATA.aggregate([
    { 
      $match: {
        $or: [
          { task_types: ["TextGeneration"] }
        ]
      }
    },
    { $sort: { _id: 1 } },
    { $limit: 20 },
  ], {
    maxTimeMS: 2000,
    allowDiskUse: true,
    cursor: { batchSize: 100 },
    comment: "Model Discovery Search"
  })

  while (cur.hasNext()) {
    cur.next()
  }