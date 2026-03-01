# Feature PRD: Optimized Beacon Format and Size

## Overview

Beacons sent from the client to the server collection agent will necessarily have a performance impact on the client processor and additional network traffic on the Internet connection. It is important to minimize the impact of both as much as possible during download, monitoring, and submission. 


## Goals
* Minimize impact of running the monitoring SDK on the client
* Reduce the size of the beacons that are sent to the server
* Do not interrupt video playback due to monitoring and processing