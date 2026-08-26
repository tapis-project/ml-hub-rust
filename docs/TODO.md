# MLHub TODOs

## Security

- Prevent Agent registrations from using MLHub-controlled hosts or routes as target endpoints.
  This must block self-directed traffic to internal MLHub APIs and prevent denial-of-service
  conditions. Define the authoritative service-host and route policy before implementing the
  domain validation.
