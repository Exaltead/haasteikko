The software is designed to help tracing reading/playing/watching challenges.
The software has two frontend written in Vue.js located in the `frontend` directory.
The software has a backend written in Go located in the `backend` directory.
The software has some scripts written in Python located in the `scripts` directory.

Infrastructure uses Azure and is managed with Bicep templates located in the `infra` directory.

Database for storage is Cosmos DB, and the software uses Azure Functions for serverless computing.

Database has the following collections:
- `users`: Stores users
- `challenges`: Stores challenges
- `library`: Stores consumed media items
- `answers`: Stores answers to challenges per users library items
- `solutions`: Stores solutions to challenges

For authentication and user management, the software uses Azure Entra External Id.