# otto-osss

Otto Secure State Service (OSSS) is the contract-first secure state extension for Otto.

Current scope is Phase 0 scaffolding from the OSSS and Otto-Crypto design document:

- vault, session, and audit contracts
- typed command models and value objects
- compile-first skeleton with no storage adapter implementation logic

All command execution must integrate through the Otto command-service layer in downstream phases.
