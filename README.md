# Swagger Petstore API with Actix Web

This project implements the Swagger Petstore API using Actix Web, a powerful and ergonomic web framework for Rust. The API provides endpoints for managing pets, inventory, orders, and users as specified by the Swagger Petstore API.

## Introduction

This API is designed to mimic the functionality of the Swagger Petstore, allowing users to interact with pet data, manage inventory, handle orders, and manage user accounts. It adheres to the Swagger API specification, providing a RESTful interface for seamless integration and easy adoption.

## Endpoints

- **Pets**:
  - `POST /pet`: Create a new pet.
  - `GET /pet/{petId}`: Retrieve a pet by ID.
  - `PUT /pet`: Update an existing pet.
  - `DELETE /pet/{petId}`: Delete a pet by ID.

- **Store**:
  - `GET /store/inventory`: Retrieve current inventory status.
  - `POST /store/order`: Place a new order.
  - `GET /store/order/{orderId}`: Retrieve an order by ID.
  - `DELETE /store/order/{orderId}`: Cancel an order by ID.

- **User**:
  - `POST /user`: Create a new user.
  - `GET /user/{username}`: Retrieve a user by username.
  - `PUT /user/{username}`: Update a user by username.
  - `DELETE /user/{username}`: Delete a user by username.
  - `POST /user/login`: User login with username and password.
  - `GET /user/logout`: User logout.
