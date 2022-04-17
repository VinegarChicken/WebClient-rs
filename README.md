```
WebClient-rs
A Http Client

USAGE:
    WebCLient-rs [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -d, --dump-headers <DUMP_HEADERS>
            Write headers to json file

    -f, --file-path <FILE_PATH>
            File to upload, if any. Use with correct type of request.

    -h, --help
            Print help information

    -i, --info
            Only print response information instead of response content

    -j, --json-header-path <JSON_HEADER_PATH>


        --no-redirect
            Disables default behavior of following requests. When downloading a website, requests
            are not followed regardless.

SUBCOMMANDS:
    CONNECT
            Send CONNECT request
    DELETE
            Send DELETE request
    Download
            Download file
    GET
            Send GET request
    HEAD
            Send HEAD request
    OPTIONS
            Send OPTIONS request
    PATCH
            Send PATCH request
    POST
            Send POST request
    PUT
            Send PUT request
    TRACE
            Send TRACE request
    help
            Print this message or the help of the given subcommand(s)
    site-download
            Download website
```
## License
[ GNU GPLv3 ](https://choosealicense.com/licenses/gpl-3.0/)
