# NNTP Response Codes

## First Digit

- 1xx - Informative message
- 2xx - Command completed OK
- 3xx - Command OK so far; send the rest of it
- 4xx - Command was syntactically correct but failed for some reason
- 5xx - Command unknown, unsupported, unavailable, or syntax error

## Second Digit

- x0x - Connection, setup, and miscellaneous messages
- x1x - Newsgroup selection
- x2x - Article selection
- x3x - Distribution functions
- x4x - Posting
- x8x - Reserved for authentication and privacy extensions
- x9x - Reserved for private use (non-standard extensions)

## Generic Responses

See section 3.2.1.1 of RFC3977 for some examples of generic responses.

### Response Code 400

If the server has to terminate the connection for some reason, it MUST give a 400 response code to the next command and then immediately close the connection.  Following a 400 response, clients SHOULD NOT simply reconnect immediately and retry the same actions. Rather, a client SHOULD either use an exponentially increasing delay between retries (e.g., double the waiting time after each 400 response) or present any associated text to the user for them to decide whether and when to retry.

### Response Code 403

If the server experiences an internal fault or problem that means it is unable to carry out the command (for example, a necessary file is missing or a necessary service could not be contacted), the response code 403 MUST be returned.

### Response Code 500

If the command is not recognized, or if it is an optional command that is not implemented by the server, the response code 500 MUST be returned.

### Response Code 501

If there is a syntax error in the arguments of a recognized command, including the case where more arguments are provided than the command specifies or the command line is longer than the server accepts, the response code 501 MUST be returned.

### Response Code 503

If the server recognizes the command but does not provide an optional feature (for example, because it does not store the required information), or if it only handles a subset of legitimate cases (see the HDR command, Section 8.5, for an example), the response code 503 MUST be returned.

### Non Authorized clients

#### Response Code 502

It is necessary to terminate the connection and to start a new one with the appropriate authority before the command can be used. Historically, some mode-switching servers (see Section 3.4.1) used this response to indicate that this command will become available after the MODE READER command (Section 5.3) is used, but this usage does not conform to this specification and MUST NOT be used. Note that the server MUST NOT close the connection immediately after a 502 response except at the initial connection (Section 5.1) and with the MODE READER command.

#### Response Code 480

The client must authenticate itself to the server (that is, it must provide information as to the identity of the client) before the facility can be used on this connection.  This will involve the use of an authentication extension such as [NNTP-AUTH].

#### Response Code 483
The client must negotiate appropriate privacy protection on the connection.  This will involve the use of a privacy extension such as [NNTP-TLS].

#### Response Code 401
The client must change the state of the connection in some other manner.  The first argument of the response MUST be the capability label (see Section 5.2) of the facility that provides the necessary mechanism (usually an extension, which may be a private extension).  The server MUST NOT use this response code except as specified by the definition of the capability in question.
