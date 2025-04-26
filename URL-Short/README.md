# Rust URL Shorten Documentation
## Overview
This Rust URL shortener project is a simple web application designed to generate shortened URLs for given original URLs and redirect users to the original URL when accessing the shortened version. It leverages the Actix Web framework for handling HTTP requests and provides a basic in-memory storage mechanism for mapping URLs.

## Features
1. Shorten any valid URL via POST request.
2. Redirect to the original URL via the shortened key using GET request.
3. Handles invalid or missing keys gracefully with appropriate error messages.

## Prerequisites
### Software Requirements
  - **Rust**: Installed via [rustup](https://www.rust-lang.org/tools/install).
  - **Postman** or `curl` for testing HTTP endpoints.
  - **VS Code**: Recommended for development with the Rust Analyzer extension.

### Rust Libraries (Crates)
The following libraries are used in the project:
  - **actix-web**: Web framework for building HTTP servers.
  - **serde** and **serde_json**: For handliing JSON data.
  - **url**: For parsing and working with URLs.

## Setup Instructions
1. Clone the repository:
   ```bash
   git clone <repository_url>
   cd URL_Short
   ```
2. Install dependencies:
  ```bash
  cargo build
  ```
3. Run the application:
   ```bash
   cargo run
   ```
The server starts on `http://127.0.0.1:8080`.

## Endpoints
1. **Post** `/shorten`
**Description**: Shortens a URL and returns the shortened version.
**Request**:
  - **Method**: POST
  - **Headers**: `Content-Type: application/json`
  - **Body**:
    ```Json
    {
      "URL": "https://example.com"
    }
    ```
**Response**:
  - **Status Code**: 200 OK
  - **Body**:
    ```Json
    {
      "shortened_url": "HTTP://127.0.0.1:8080/<shortened_keys>
    }
    ```
2. **Get** `/{key}`
**Description**: Redirects the user to the original URL corresponding to the shortened key.
**Request**:
  - **Method**: GET
  - **URL**: `http://127.0.0.1:8080/<shortened_key>`
**Response**:
  - **Status Code**: 302 Found (Redirects to original URL)
  - **Error Handling**: Returns 404 Not Found if the key doesn't exist.

## Usage Example
### **Shorten a URL**:
using Postman:
1. Create a new POST request with the URL `http://127.0.0.1:8080/shorten`.
2. Add `Content-Type: application/json` in headers.
3. Send the JSON payload:
   ```JSON
   {
     "URL": "https://example.com"
   }
   ```
### **Access a Shortened URL**:
Using the shortened URL returned (e.g., `http://127.0.0.1/abc123`), open a browser or send a GET request to test redirection.

## Customization
1. Persistent Storage:
   - Integrate a database like SQLite or PostgreSQL to store URLs permanently.
   - Use crates like `sqix` or `diesel` for database interactions.
2. Custom Key Generation:
   - Replace the hashing algorithm in the `shorten_url` function with a more sophisticated or user-defined mechanism.
3. Deploying to Production:
   - Use platforms like Heroku, AWS, or Azure to deploy your application publicly.
  
## Known Issues
  - URLs are stored in memory and lost when the server is restarted.
  - No validation is performed to check if the input URL is valid or already shortened.

## References
### Rust Language and Tools
1. [Official Rust Documentation](https://doc.rust-lang.org/stable/)
   Comprehensive guides. tutorials, and references for the Rust programming language.
2. [Rust Book](https://doc.rust-lang.org/book/)
   A beginner-friendly resource for learning Rust from the ground up.
3. [Rust Crates](https://crates.io/)
   Repository of libraries (crates) to explore and integrate into your Rust Projects.
4. [Cargo](https://doc.rust-lang.org/cargo/)
   Rust's package manager and build system documentation.

### Actix Web Framework
1. [Actix Web Documentation](https://actix.rs/docs/)
   Official documentation for Actix Web, including tutorials and examples for building web applications.
2. [Actix Examples](https://github.com/actix/examples)
   A GitHub Repository with examples of Actix Web projects to learn implementation patterns.

### JSON Handling
1. [Serde](https://serde.rs/)
   Documentation for the serialization and deserialization library used in Rust.

### HTTP Tools for Testing
1. [Postman](https://www.postman.com/)
   A powerful tool for testing and debugging REST APIs.
2. [cURL](https://curl.se/)
   Command-line tools for testing HTTP requests.

### Web and URL Management
1. [Rust Crate](https://docs.rs/url/latest/url/)
   Documentation for the `url` crate, used for parsing and manipulating URLs.

### Deployment
1. [Heroku](https://www.heroku.com/)
   Platform for deploying web applications, including Rust-based services.
2. [AWS Elastic Beanstalk](https://aws.amazon.com/elasticbeanstalk/)
   A managed service for deploying and scaling web apps.
3. [Docker](https://www.docker.com/)
   Learn how to containerize Rust applications for deployment.
