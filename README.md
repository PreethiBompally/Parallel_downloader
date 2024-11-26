# Parallel Downloader

Parallel Downloader is a Rust-based command-line tool aimed at creating a highly efficient download manager. The project utilizes parallel TCP connections to split files into parts, download them concurrently, and merge them efficiently. Our objective is to leverage Rust's concurrency and memory safety to achieve enhanced speed and reliability in file downloads. It supports .jpg, .png, .gif files.

## Phase 3:

### Current Features:
- Simple command-line interface for user input on download parameters.
- DNS resolution to translate domain names into IP addresses.
- TCP connection establishment for downloads.
- TLS connection integration for secure data transfer.
- HTTP requests fetch file metadata and initiate range-based downloads.
- Multi-threaded downloading to optimize download speeds and efficiency.
- File parts are saved and merged to reconstruct the original file.


## Data Flow Overview
- **User Input:** Starts by collecting user inputs such as the URL, output filename, and number of connections from the command line.
- **DNS Resolution:** Resolves the domain name to an IP address of the server.
- **TCP Connection:** Establishes a TCP connection.
- **TLS Connection:** Manages secure sessions if required.
- **HTTP Request:** Retrieves file metadata and starts range-based downloads.
- **Multi-threaded Download:** Downloads file parts concurrently.
- **File Merging:** Merges downloaded parts into a single file after all parts are downloaded.


## Crates Dependencies
    The project uses the following crates to help achieve the desired functionality:

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

    To run the project using docker
    - docker build -t parallel-downloader .
    - docker run -it --init -v $(pwd)/downloads:/downloads parallel-downloader
    - input url to download
    - input number of connections
    - input output filename

## Example urls to test the application:
    - https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/Uga-VII.jpg
    - https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/story_hairydawg_UgaVII.jpg
    - https://www.nasa.gov/wp-content/uploads/2024/10/ksc-20240819-ph-jbs01-0022orig.jpg
    - https://www.nasa.gov/wp-content/uploads/2024/11/iss070e028324orig.jpg
    - https://filesampleshub.com/download/image/png/sample3.png
    - https://cobweb.cs.uga.edu/~perdisci/CSCI6760-F21/Project2-TestFiles/topnav-sport2_r1_c1.gif

Note: The application works perfectly with urls which have file extension at the end