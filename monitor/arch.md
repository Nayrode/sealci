# Monitor Architecture for sealCI

## Overview

The monitor is the foundational component of the sealCI project. It is designed to listen to specific events on a GitHub repository and trigger the CI/CD pipeline accordingly. The future vision for this component includes the ability to listen to tag events and automate the generation, signing, and storage of tarballs.

## Components

The core components of the monitor include:

- **HTTP Server**
- **GitHub Client**
- **Event Listener**
- **CI/CD Pipeline**

### HTTP Server

- **Purpose**: Serves as the entry point for the monitor.
- **Function**: Listens for incoming HTTP requests, which could be webhooks or API calls from GitHub indicating events such as code pushes, pull requests, or tag creations.
- **Interaction**: Upon receiving a request, it triggers the appropriate event listener or directly initiates the CI/CD pipeline.

### GitHub Client

- **Purpose**: Interfaces with GitHub to listen for specific events.
- **Function**: Utilizes GitHub's API to subscribe to events like `push`, `pull_request`, or `tag`.
- **Interaction**: Communicates with the event listener to notify it of relevant events happening in the repository.

### Event Listener

- **Purpose**: Monitors for specific events that are configured to trigger actions.
- **Function**: Listens for events from the GitHub client and decides whether to trigger the CI/CD pipeline or other actions like generating a release.
- **Interaction**: Upon detecting an event, it either triggers the CI/CD pipeline or communicates with the release agent.

### CI/CD Pipeline

- **Purpose**: Automates the process of building, testing, and deploying the project.
- **Function**: Executes a series of predefined steps to build the project, run automated tests, and deploy the application to the desired environment.
- **Interaction**: Triggered by the event listener, it performs the necessary actions to ensure the code is ready for deployment.

## Workflow

1. **Event Detection**:
   - An event occurs in the GitHub repository, such as a code push or a new release tag.
   - The GitHub client detects this event and notifies the event listener.

2. **Event Processing**:
   - The event listener processes the event and determines the appropriate action.
   - If the event is related to code changes (e.g., a push or pull request), it triggers the CI/CD pipeline.
   - If the event is a release tag, it triggers the release agent to generate a tarball, sign it, and store it.

3. **CI/CD Execution**:
   - The CI/CD pipeline is initiated, which involves steps like checking out the code, building the project, running tests, and deploying the application.
   - The results of the pipeline are reported back, indicating success or failure.

4. **Release Management**:
   - For release events, the release agent generates a tarball of the code, signs it for integrity, and stores it in a designated location.
   - This ensures that the release is secure and can be verified for authenticity.
