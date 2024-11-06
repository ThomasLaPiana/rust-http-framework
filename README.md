# Rust HTTP Server

Educational implementation of an HTTP webserver in Rust

## The webserver implements the following features

* Host & Port configurable via CLI arguments (DONE)
* Supports `GET`, `UPDATE` & `DELETE` HTTP methods (DONE)
* Basic Routing (DONE)
* Has a simple in-memory DB (DONE)
* Writes to disk upon receipt of SIGTERM (WIP)
* Reads from disk on startup (WIP)
