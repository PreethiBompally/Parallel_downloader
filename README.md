# Parallel_downloader

Parallel Downloader is a Rust-based command-line tool aimed at creating a highly efficient download manager. The project utilizes parallel TCP connections to split files into parts, download them concurrently, and merge them efficiently. Our objective is to leverage Rust's concurrency and memory safety to achieve enhanced speed and reliability in file downloads.

## Phase 1: Basic Networking Setup and CLI

### Current Features:
- DNS resolution for translating domain names into IP addresses.
- TCP connection establishment to initiate downloads.
- Simple command-line interface for users to specify download parameters.
- Outcome: A basic version that can initiate a single download using command-line inputs.

### Current Architecture

 - src/
    - main.rs: This is the entry point of the program. It is responsible for:
        - Parsing command-line arguments using a user-friendly command-line interface.
        - Coordinating the download by using different modules like dns, tcp, and downloader.
        - Initiating download processes, managing the flow, and eventually merging downloaded parts into a complete file.
    - dns.rs: This module is responsible for domain name resolution. It translates domain names to IP addresses using ToSocketAddrs from the standard library, enabling reliable connection establishment.

    - tcp.rs: This module is used for establishing a TCP connection to the download server. It manages low-level networking for connecting to the specified IP address and port.


## Data Flow Overview
    - User Input: The program starts by parsing user inputs from the command line (such as the URL, output filename, and number of connections).
    - DNS Resolution: The domain name is resolved to obtain the IP address of the server.
    - TCP : A TCP connection is established. 
    - TLS Connection(to be implemented): If secure transmission is required, TLS manages the secure session.
    - Download Initiation(to be implemented):  HTTP requests is used to download the specified file, and these downloaded segments are saved.
    - File Merging(to be implemented): Once all parts are downloaded, the program (in future releases) will merge them into a single coherent file.

## Crates Dependencies
    The project uses the following crates to help achieve the desired functionality:

        - clap: This crate is used for parsing command-line arguments. It provides an easy-to-use interface for defining and retrieving user inputs, making the downloader versatile and easy to interact with.

        - rustls: This crate is used for establishing secure connections using TLS. It ensures encrypted communication when downloading files over HTTPS. (Planned for integration in a future release.)

        - webpki_roots: This crate provides trusted root certificates required by rustls for TLS handshake. (To be implemented along with TLS integration.)

        - Standard Library (std): Rust’s standard library is used for foundational functionalities like networking (std::net), file handling (std::fs), multithreading (std::thread), and input/output operations (std::io).


### Current State:
 
    - DNS resolution and TCP connection functionalities are implemented.
    - Command-line interface and basic single-threaded download support are available.
    - TLS integration and file merging are still under development.
 
### Future Plans:
 
    - v2: Introduce secure data transmission using rustls and extend the downloader for single-threaded downloads over secure connections.
    - v3: Enable multi-threaded downloads and file merging to significantly improve download speed and efficiency.
    - final: Implement comprehensive testing, fuzzing, and publish the crate on crates.io.
 
## Usage Instructions
    To compile and run the project:
    - cargo build
    - cargo run -- --url "https://example.com/file.zip" --output "output_file.zip" --num-connections 1