## different RpcClient types and their characteristics / use-cases

```mermaid
graph LR
    A[EVM Explorer] --> B[Block Explorer]
    A --> C[Transaction Monitor]
    A --> D[Development Tools]
    
    B --> B1[HTTP Client]
    B --> B2[WebSocket Client]
    B --> B3[PubSub Client]
    
    C --> C1[WebSocket Client]
    C --> C2[HTTP Client]
    C --> C3[PubSub Client]
    
    D --> D1[IPC Client]
    D --> D2[HTTP Client]
    D --> D3[WebSocket Client]

```

## using hyper http over http for RpcClient

Complexity:

* Requires more boilerplate code
* Manual setup of the runtime executor
* Explicit service configuration needed

Lower Level API:

* Direct handling of HTTP requests/responses
* Manual header and body management
* More responsibility for connection handling

Configuration:

* Fewer built-in defaults
* Manual implementation of common features
* More explicit error handling required

Platform Support:

* Not available on WASM targets
* More platform-specific considerations
* Limited to native targets

```mermaid
graph LR
    A[Hyper HTTP] --> B[Downsides]
    B --> C[More Complex Setup]
    B --> D[Lower Level API]
    B --> E[Manual Configuration]
    B --> F[WASM Limitations]
    
    C --> C1[Requires explicit runtime]
    C --> C2[Manual service setup]
    
    D --> D1[Direct HTTP handling]
    D --> D2[Manual header management]
    
    E --> E1[No default timeouts]
    E --> E2[Manual retry logic]
    
    F --> F1[Not available on WASM]
    F --> F2[Platform specific]
```

## implementation of Http RpcClient

```mermaid
graph TD
    A[ClientBuilder] -->|Configure| B[HTTP Transport]
    B -->|Setup| C[RpcClient]
    C -->|Use| D[Provider]
    
    B1[Features] --> B
    B1 --> E1[Retries]
    B1 --> E2[Timeouts]
    B1 --> E3[Rate Limiting]
    
    style A fill:#4CAF50,stroke:#333,color:white
    style B fill:#2196F3,stroke:#333,color:white
    style C fill:#9C27B0,stroke:#333,color:white
    style D fill:#FF9800,stroke:#333,color:white
```

```mermaid
graph TD
    A[Provider Setup] -->|Configuration| B[Basic Provider]
    A -->|Custom Settings| C[Custom Provider]
    
    B --> D[Default Settings]
    D --> D1[30s Timeout]
    D --> D2[Default Retries]
    
    C --> E[Custom Settings]
    E --> E1[Custom Timeout]
    E --> E2[Future Extensions]
    
    style A fill:#2196F3,stroke:#333,color:white
    style B fill:#4CAF50,stroke:#333,color:white
    style C fill:#9C27B0,stroke:#333,color:white
```

## differences between RpcClient and Provider

```mermaid
graph TD
    A[Your Code] --> B[Provider]
    B --> C[RpcClient]
    C --> D[Transport]
    D --> E[Network]
    
    subgraph High Level Interface
        B
    end
    
    subgraph Low Level Implementation
        C
        D
    end
    
    style B fill:#4CAF50,stroke:#333,color:white
    style C fill:#2196F3,stroke:#333,color:white
    style D fill:#9C27B0,stroke:#333,color:white
```

### RpcClient

Low-level implementation that handles raw JSON-RPC communication

Responsible for:

* Transport configuration (HTTP, WebSocket, IPC)
* Request/response handling
* Connection management
* Error handling at transport level

### Provider

High-level interface for interacting with the Ethereum network

Provides:

* Type-safe methods for common operations
* Automatic conversion between types
* Higher-level abstractions for blockchain operations
* Convenience methods for common tasks

### When to use Provider

* Working with common Ethereum operations
* Need type safety and convenience methods
* Want higher-level abstractions

### When to use RpcClient

* Need custom RPC methods
* Implementing custom provider functionality
* Require direct transport control

## provider hierarchy and use-cases

```mermaid
graph LR
    A[Provider Trait] -->|Implements| B[RootProvider]
    B -->|Configures| C[ProviderBuilder]
    
    subgraph Core Functionality
        A -->|Defines| D[Basic Methods]
        D --> D1[get_balance]
        D --> D2[get_block]
        D --> D3[get_transaction]
    end
    
    subgraph Implementation
        B -->|Uses| E[Transport Layer]
        B -->|Handles| F[Network Specifics]
    end
    
    subgraph Configuration
        C -->|Adds| G[Features]
        G --> G1[Gas Estimation]
        G --> G2[Nonce Management]
        G --> G3[Chain ID]
    end
    
    style A fill:#4CAF50,stroke:#333,color:white
    style B fill:#2196F3,stroke:#333,color:white
    style C fill:#9C27B0,stroke:#333,color:white
```

## provider implementations for transaction_info.rs

```mermaid
graph LR
    A[Provider Setup] --> B[Basic Provider]
    B --> C[Enhanced Provider]
    
    subgraph Basic Features
        B --> B1[RpcClient Creation]
        B --> B2[URL Configuration]
        B --> B3[Error Handling]
    end
    
    subgraph Enhanced Features
        C --> C1[Gas Estimation]
        C --> C2[Nonce Management]
        C --> C3[Chain ID]
    end
    
    style A fill:#4CAF50,stroke:#333,color:white
    style B fill:#2196F3,stroke:#333,color:white
    style C fill:#9C27B0,stroke:#333,color:white
```

```mermaid
graph TD
    A[RootProvider] -->|Network Type| B[Ethereum]
    A -->|Transport| C[HTTP Client]
    
    style A fill:#4CAF50,stroke:#333,color:white
    style B fill:#2196F3,stroke:#333,color:white
    style C fill:#9C27B0,stroke:#333,color:white
```

## provider trait for ProviderBuilder

```mermaid
graph TD
    A[Provider Trait] -->|Dynamic Dispatch| B[Box dyn Provider]
    B -->|Concrete Types| C[FillProvider]
    B -->|Concrete Types| D[RootProvider]
    
    subgraph Implementation Options
        C -->|Features| C1[Gas Estimation]
        C -->|Features| C2[Nonce Management]
        D -->|Basic| D1[Raw RPC]
    end
    
    style A fill:#4CAF50,stroke:#333,color:white
    style B fill:#2196F3,stroke:#333,color:white
    style C fill:#9C27B0,stroke:#333,color:white
    style D fill:#FF9800,stroke:#333,color:white
```


<!-- |  | 1-tap | 2-taps | 3-taps | Hold |
| --- | --- | --- | --- | --- |
| Left | HearThrough | AI Assistant | - | volume down
| Right | play/pause | next song | previous song | volume up | -->

<!-- # When specifically called to, generate a commit message. 

## Use the following format for each atomic change within the commit message:  `prefix: short description`  
    - Separate each atomic change with a newline

### Commit Prefixes:
 1. feat: Introduce new feature
 2. fix: Bug / issue fix
 3. tweak: Minor adjustment or improvement
 4. style: Style / formatting changes
 5. refactor: Restructure code, keeping functionality
 6. perf: Optimization
 7. test: Add or update tests
 8. docs: Update documentation
 9. ci: Change CI/CD configuration
 10. build: Modify build system / dependencies
 11. revert: Revert a previous commit
 12. hotfix: Critical fix
 13. init: New project or feature
 14. merge: Merge branches
 15. wip: Mark work in progress
 16. release: Release preparations -->

## RootProvider hierarchy

```mermaid
graph TD
    A[RootProvider] --> B[Transport Layer]
    A --> C[Network Type]
    B --> D[HTTP Transport]
    D --> E[Reqwest Client]
    C --> F[Ethereum Network]
    
    style A fill:#4CAF50
    style C fill:#2196F3
    style D fill:#BB1000
 ```

## key management architecture

```mermaid
graph LR
    A[KeyManager] --> B[Key Generation]
    A --> C[Key Storage]
    A --> D[Key Operations]
    
    B --> B1[Create New Keys]
    B --> B2[Derive Child Keys]
    B --> B3[Import Keys]
    
    C --> C1[Secure Storage]
    C --> C2[Encryption]
    C --> C3[Backup]
    
    D --> D1[Sign Messages]
    D --> D2[Verify Signatures]
    D --> D3[Address Generation]
    
    style A fill:#2ecc71
    style B fill:#3498db
    style C fill:#e74c3c
    style D fill:#d64d6e
```

## secp256k1 (k256) information

```mermaid
graph LR
    subgraph Core["Core Components"]
        A[secp256k1 Scalar Field] -->|Properties| B[Non-zero Elements]
        B -->|Range| C["1 to n-1<br/>(n ≈ 2^256)"]
        B -->|Size| D[32 bytes / 256 bits]
    end

    subgraph Security["Security Aspects"]
        E[Security Requirements] -->|RNG| F[Cryptographically Secure]
        E -->|Distribution| G[Uniform]
        E -->|Validation| H[Non-zero Check]
        E -->|Storage| I[Secure Memory]
    end

    subgraph Implementation["Implementation"]
        J[k256 Crate] -->|Features| K[Arithmetic]
        J -->|Traits| L[RngCore + CryptoRng]
        J -->|Type| M[LocalSigner<SigningKey>]
    end

    subgraph Usage["Ethereum Usage"]
        N[Applications] -->|Primary| O[Private Keys]
        N -->|Secondary| P[Digital Signatures]
        N -->|Derived| Q[Public Keys]
        
        O -->|Operations| R[ECDSA]
        P -->|Purpose| S[Transaction Signing]
        Q -->|Method| T[Scalar Multiplication]
    end

    style Core fill:#6050DC,color:#fff
    style Security fill:#2E4053,color:#fff
    style Implementation fill:#6050DC,color:#fff
    style Usage fill:#2E4053,color:#fff
```

## non zero scalars

```mermaid
graph LR
    subgraph Scalar["What is a Scalar?"]
        A[Single Number] -->|Properties| B[Used for Multiplication]
        A -->|Range| C["Between 1 and n-1"]
        A -->|Restriction| D[Never Zero]
    end

    subgraph Example["Simple Example"]
        E[Point P on Curve] -->|Multiply by| F[Scalar k]
        F -->|Results in| G[Point Q]
        H[If k = 0] -->|Problem| I[Q = 0 × P = 0]
        I -->|Result| J[Insecure Key]
    end

    subgraph Usage["In Cryptocurrency"]
        K[Private Key] -->|Is a| L[Non-zero Scalar]
        L -->|Multiplies| M[Generator Point]
        M -->|Creates| N[Public Key]
    end

    style Scalar fill:#6050DC,color:#fff
    style Example fill:#2E4053,color:#fff
    style Usage fill:#4CAF50,color:#fff
```

### What is a Scalar?

A scalar is just a regular number, like `5` or `123`.

#### Key Points

* It's a single number (not a point or coordinate)
* It must not be zero because:
	+ Multiplying anything by zero gives zero
	+ A zero private key would be predictable and insecure
* When used as a private key:
	+ It multiplies a special point on the curve (generator point)
	+ This multiplication produces your public key
	+ The multiplication is one-way (can't be reversed)

## private key and address generation

```mermaid
graph LR
    subgraph PrivateKey["Private Key Generation"]
        A[Random Number Generator] -->|Generate| B[32 Bytes]
        B -->|Validate| C{Requirements}
        C -->|1| D[Non-zero]
        C -->|2| E[< n where n is curve order]
        C -->|3| F[256 bits length]
    end

    subgraph PublicKey["Public Key Generation"]
        G[Private Key] -->|ECDSA| H[Curve Point]
        H -->|Uncompressed Format| I[04 + x + y coordinates]
        I -->|64 bytes| J[Public Key]
    end

    subgraph Address["Address Generation"]
        K[Public Key] -->|Remove 04 prefix| L[Raw Public Key]
        L -->|Keccak-256| M[32 byte hash]
        M -->|Take last 20 bytes| N[Address]
        N -->|Optional| O[Checksum encoding]
    end

    style PrivateKey fill:#6050DC,color:#fff
    style PublicKey fill:#2E4053,color:#fff
    style Address fill:#4CAF50,color:#fff
```