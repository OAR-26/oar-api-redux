# OAR Infrastructure

This crate contains the infrastructure implementations for the OAR  application.

## What it does

The infrastructure crate provides:
- Database implementations for repositories
- External service integrations
- Configuration and setup code
- Data access layer implementations

## How it works

- Implements the repository interfaces defined in the domain layer
- Handles database connections and queries
- Provides concrete implementations for abstract interfaces

## Key Components

- **Repositories**: Database implementations (like PostgresUserRepository)
- **Database**: Database connection and query logic
- **External Services**: Integrations with third-party services

## Contributing

To add new infrastructure implementations:

1. Implement repository interfaces from the domain crate
2. Add database models and migrations if needed

