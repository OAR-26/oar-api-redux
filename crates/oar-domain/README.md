# OAR Domain

This crate contains the core business logic and domain models for the OAR application.

## What it does

The domain crate provides:
- Core business entities and models
- Repository interfaces (ports) that define what the infrastructure layer must implement
- Business logic and validation rules
- Domain services and use cases

## How it works

- Follows Domain-Driven Design (DDD) principles
- Contains pure business logic without external dependencies
- Defines repository interfaces that infrastructure implementations must satisfy

## Key Components

- **Models**: Core entities like User with their data structures
- **Ports**: Repository interfaces that define contracts for data access
- **Services**: Business logic and use case implementations

## Contributing

To add new business logic:

1. Define new models in `src/users/models.rs`
2. Add repository interfaces in `src/users/ports.rs`
3. Implement business logic and validation
4. Keep dependencies minimal - this should be pure business logic only

