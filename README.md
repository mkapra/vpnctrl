# Wireguard VPN Configurator

This project provides multiple services that make rolling out wireguard configurations much easier.

## Structure

There are multiple components in this repository:

* [api/](api/): The API is the core of this project. It stores all the data and is the communication point for all the clients.
* [client/](client/): The client is a tool for getting the latest configuration of a server or client automatically. It serves the purpose to automate the rollout process of wireguard configurations

## Installation

For installation instructions see the [Wiki Page](https://github.com/mkapra/vpnctrl/wiki/Installation)

## Development

* [API-Dokumentation libwgbuilder](https://mkapra.github.io/vpnctrl/libwgbuilder/index.html)
