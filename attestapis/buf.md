# AttestAPIs

The APIs required to interact with the `AttestService`.

There are two ways to attest with an encalve:

1. Client to enclave. Used when a (non enclave) client wants to attest that the
   enclave is a predetermined software image.
2. Enclave to enclave. This is for attesting between two trusted enclaves
   running the same software image.

## Client to Enclave

```mermaid
sequenceDiagram
    autonumber
    Client->>+Enclave: AttestRequest
    Enclave->>+Client: AttestResponse
    Client->>Client: Verify AttestResponse contents
    loop Ongoing Session
        Client-->>Enclave: AttestedMessage
        Enclave-->>Client: AttestedMessge
    end
```

## Enclave to Enclave

```mermaid
sequenceDiagram
    autonumber
    Enclave_1->>+Enclave_2: MutualAttestRequest
    Enclave_2->>+Enclave_1: MutualAttestResponse
    Enclave_1->>Enclave_1: Verify MutualAttestResponse contents
    loop Ongoing Session
        Enclave_1-->>Enclave_2: AttestedMessage
        Enclave_2-->>Enclave_1: AttestedMessge
    end
```
