# Rust HTTP Server

Educational implementation of an HTTP webserver in Rust

## Webserver Features

* Host & Port configurable via CLI arguments
* Supports `GET`, `UPDATE` & `DELETE` HTTP methods
* Basic Routing
* Has a simple in-memory DB
* Reads from disk on startup
* Writes to disk on each modification request

## Glaring Omissions
* Tests
* Proper Error Handling (unwraps all around)
* Multi-Threading
* Graceful Shutdowns

## Server Logs

![image](https://github.com/user-attachments/assets/2208242e-1a4a-4e29-a366-c471e4ae83fc)

## Client Logs

![image](https://github.com/user-attachments/assets/a3480fd0-9e7b-41f3-9cc1-c34d4860d5d4)
