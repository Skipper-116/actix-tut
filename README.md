# Hello Rust with Actix

Welcome to the Hello Rust project! This is a simple web application built using the Actix web framework, which is known for its performance and ease of use.

## Setup

To get started with this project, ensure you have Rust and Cargo installed. You can check your versions with the following commands:

```sh
cargo --version
# Output: cargo 1.84.1

rustc --version
# Output: rustc 1.84.1 (e71f9a9a9 2025-01-27) (Homebrew)
```

Follow these steps to set up the project:

1. Clone the repository:

   ```sh
   git clone https://github.com/Skipper-116/actix-tut.git
   cd actix-tut
   ```

2. Build the project:

   ```sh
   cargo build
   ```

3. Run the application:

   ```sh
   cargo run
   ```

4. Open your browser and navigate to `http://localhost:8080` to see the application in action.

## Playing with Actix

I'm playing with Actix because it is the top-rated web framework there is.

Feel free to explore the code and modify it to suit your needs. Happy coding!

## Endpoints

### Endpoints

- `GET /api/v1/hello`: Returns a JSON response with a greeting message.
- `POST /api/v1/hello`: Accepts a JSON payload and returns json response back.
- `PUT /api/v1/hello/{id}`: Accepts a JSON payload and returns json response back.
- `GET /api/v1/hey/{name}`: Returns a JSON response with a different greeting message.
