db.MODEL_METADATA.aggregate([
    { $match: { $or: [
      {task_types: ["TextGeneration"] }
    ] }},
    { $sort: { _id: 1 } },
    { $limit: 20 },
  ])