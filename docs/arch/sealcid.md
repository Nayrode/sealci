# sealcid Architecture

## Contributors

- DURAT Mathias
- PONTHIEU Hugo
- TCHILINGUIRIAN Théo

## What is sealcid?

sealcid is a gRPC server and its command line for managing the deployment of SealCI components. Deploying, re-running, pointing to another service.

sealcid's CLI can be used remotely (such as from your local machine) to manage a sealcid instance on your servers.

A component can be managed by sealcid as long as it properly implements the App trait from the sealcid_traits package (defined as a workspace package of sealcid).

You could run multiple sealcid daemon instances on the same machine if you configure the ports to not clash.

The scheduler receives a stream of CI actions (CI job units) as its main input. It also tracks a set of CI agents (registered computing nodes) that provide a dynamic resource pool, where the CI actions will be executed.

The main role of a scheduler instance is to select CI agents on which to run the received actions. This selection of an agent and distribution of a action on this agent is called 'scheduling'.

## Why sealcid

sealcid is an easy way to deploy SealCI locally or on servers. Moreover, it allows for tracking service logs together.

## How sealcid works

sealcid implements three main components:

- A common App trait for all SealCI services to implement.
- A sealcid daemon that runs all other services (imported as dependencies).
- A sealcid command line interface that connects to the daemon via gRPC, using a sealci config file.

This is sealcid's workflow for running SealCI's services:

- Configure each service.
- Start its gRPC server asynchronsouly for listening to commands.
- When receiving a command to start a service, start the service asynchronously.

Later, it should be made to work as such:

- Start its gRPC server asynchronsouly for listening to commands.
- When receiving a command to start a service, configure and start the service asynchronously.

Currently, sealcid can only manage one instance of each service at a time. Later, it will be refactored to be able to scale services.
