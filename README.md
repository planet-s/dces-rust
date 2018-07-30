# DCES: Dedicated Component Entity System
Based on Entity Component System: https://en.wikipedia.org/wiki/Entity–component–system

[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## DCES_ENGINE
* Developer Interface
* Iterate over ES
* Handles ECS

## ECM: Entity Component Manager (singelton)
* Knows all entities as ids
* Contains vector of all components
* Components referenced by entity ids

## ES: Entity System (0..n)
* Knows filtered subset of entities e.g. render entities for render system
* Provides one system run function
* Read and write to components