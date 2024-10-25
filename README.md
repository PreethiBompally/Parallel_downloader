# Parallel_downloader

Parallel Downloader is a Rust-based command-line tool aimed at creating a highly efficient download manager. The project utilizes parallel TCP connections to split files into parts, download them concurrently, and merge them efficiently. Our objective is to leverage Rust's concurrency and memory safety to achieve enhanced speed and reliability in file downloads.

## Phase 2:

### Current Features:
- Simple command-line interface for users to specify download parameters.
- DNS resolution for translating domain names into IP addresses.
- TCP connection establishment to initiate downloads.
- TLS connection integration.
- Single threaded download of file.
- Outcome: A basic version that can initiate a single-threaded download using command-line inputs.


- Multi-threaded download and file merging are still under development.


## Data Flow Overview
    - User Input: The program starts by parsing user inputs from the command line (such as the URL, output filename, and number of connections).
    - DNS Resolution: The domain name is resolved to obtain the IP address of the server.
    - TCP : A TCP connection is established. 
    - TLS Connection: If secure transmission is required, TLS manages the secure session.
    - Download Initiation(single-threaded):  HTTP requests is used to download the specified file, and these downloaded file is saved.
    - Multi-threaded download(to be implemented): Splitting and downloaded the file and saving the parts.
    - File Merging(to be implemented): Once all parts are downloaded, the program will merge them into a single coherent file.

## Crates Dependencies
    The project uses the following crates to help achieve the desired functionality:

        - clap: This crate is used for parsing command-line arguments. It provides an easy-to-use interface for defining and retrieving user inputs, making the downloader versatile and easy to interact with.

        - rustls: This crate is used for establishing secure connections using TLS. It ensures encrypted communication when downloading files over HTTPS. (Planned for integration in a future release.)

        - webpki_roots: This crate provides trusted root certificates required by rustls for TLS handshake. (To be implemented along with TLS integration.)

        - Standard Library (std): Rust’s standard library is used for foundational functionalities like networking (std::net), file handling (std::fs), multithreading (std::thread), and input/output operations (std::io).

        - native-tls: This crate provides bindings to native TLS libraries, allowing the downloader to establish secure connections using the underlying system's TLS implementation. It simplifies handling encrypted communication for file downloads over HTTPS.

        - url: The url crate is utilized for URL parsing and manipulation. It provides robust tools to handle, construct, and normalize URLs, ensuring that the URLs provided for downloading are correctly formatted and processed.


## Usage Instructions
    To compile and run the project:
    - cargo build
    - cargo run
    - input url to download
    - input number of connections
    - input output filename