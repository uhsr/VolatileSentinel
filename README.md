# VolatileSentinel: Decentralized Crypto Price Alerts

A robust, decentralized system for real-time crypto price alerts leveraging smart contracts, serverless functions, and encrypted push notifications.

VolatileSentinel provides a reliable and secure mechanism for users to receive immediate notifications when cryptocurrency prices breach predefined thresholds. Unlike centralized alert services, VolatileSentinel utilizes smart contracts to store alert configurations and serverless functions to monitor price feeds, ensuring transparency and minimizing single points of failure. This decentralized architecture makes the system resistant to censorship and manipulation, giving users greater control over their alert parameters. The platform offers highly customizable alerts, allowing users to specify the cryptocurrency, exchange, threshold price (both upper and lower bounds), and notification frequency. Furthermore, all push notifications are encrypted end-to-end, safeguarding user privacy and preventing unauthorized access to sensitive price information.

The core benefit of VolatileSentinel is its ability to provide timely and trustworthy price alerts in a volatile market. Traditional alert systems can be unreliable, experiencing delays or even outages during periods of high market activity. By leveraging a decentralized architecture and serverless computing, VolatileSentinel offers superior scalability and availability. Moreover, the smart contract integration ensures that alerts are triggered automatically and verifiably, eliminating the risk of human error or manipulation. The encrypted push notification system adds an extra layer of security, guaranteeing that only the intended recipient can access the alert information. This combination of features makes VolatileSentinel an ideal solution for traders, investors, and anyone who needs to stay informed about crypto market fluctuations.

Ultimately, VolatileSentinel empowers users with a powerful tool for managing their crypto investments. The platform's decentralized nature, coupled with its customizable alert parameters and encrypted push notifications, provides a secure, reliable, and user-friendly experience. Whether you're a seasoned trader or a novice investor, VolatileSentinel can help you stay ahead of the curve and make informed decisions in the dynamic world of cryptocurrency.

## Key Features

*   **Decentralized Alert Configuration:** Alert parameters (cryptocurrency, exchange, threshold price, notification frequency) are stored in immutable smart contracts deployed on a blockchain. This guarantees transparency and prevents unauthorized modifications.
*   **Real-time Price Monitoring:** Serverless functions constantly monitor price feeds from various cryptocurrency exchanges using WebSocket connections. These functions are triggered by real-time data streams and execute independently, ensuring high availability.
*   **Threshold Breach Detection:** Smart contract logic is triggered by the serverless functions when a price breach is detected. This ensures that alerts are triggered automatically and verifiably without relying on centralized intermediaries.
*   **Encrypted Push Notifications:** When a threshold is breached, an encrypted push notification is sent to the user's registered device. The encryption uses a public-key cryptography scheme, guaranteeing end-to-end security.
*   **Customizable Alert Frequency:** Users can configure the minimum time interval between notifications to avoid being bombarded with alerts during periods of high volatility.
*   **Multi-Exchange Support:** The platform supports multiple cryptocurrency exchanges, allowing users to track prices across different markets. New exchanges can be added easily through configuration updates to the serverless functions.
*   **Gas Optimization:** Smart contracts are designed with gas efficiency in mind to minimize transaction costs associated with alert creation and modification.

## Technology Stack

*   **Rust:** The backend is written in Rust for its performance, safety, and concurrency features. Rust ensures memory safety and prevents common programming errors, making the system more robust.
*   **Solidity:** Smart contracts are written in Solidity and deployed on an EVM-compatible blockchain (e.g., Ethereum, Polygon). Solidity provides the necessary tools for creating and managing decentralized alert configurations.
*   **WebSockets:** Used for real-time price feed ingestion from cryptocurrency exchanges. WebSockets provide a persistent connection for efficient and low-latency data transfer.
*   **Serverless Functions (AWS Lambda/Google Cloud Functions/Azure Functions):** The price monitoring and alert triggering logic are implemented using serverless functions. This allows for scalable and cost-effective execution of these tasks.
*   **Push Notification Service (Firebase Cloud Messaging/Apple Push Notification Service):** Used to deliver encrypted push notifications to user devices. These services provide reliable and secure push notification delivery.
*   **Encryption Library (libsodium):** Used for end-to-end encryption of push notifications. Libsodium is a modern and easy-to-use cryptography library.

## Installation

1.  Clone the repository:

    git clone https://github.com/uhsr/VolatileSentinel.git
    cd VolatileSentinel
2.  Install Rust and Cargo:

    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
3.  Install dependencies for the smart contracts:

    Install Foundry: curl -L https://foundry.paradigm.xyz | bash
    Then: foundryup
4.  Install dependencies for the backend:

    cargo build --release
5.  Deploy the smart contracts to your desired EVM-compatible blockchain. The deployment script is located in the contracts directory.
6.  Configure the serverless functions to connect to the deployed smart contracts and the cryptocurrency exchange APIs.
7.  Set up the push notification service and configure the backend to send encrypted push notifications.

## Configuration

The following environment variables need to be configured for the backend and serverless functions:

*   `SMART_CONTRACT_ADDRESS`: The address of the deployed alert contract.
*   `RPC_ENDPOINT`: The RPC endpoint of the blockchain.
*   `EXCHANGE_API_KEY`: The API key for the cryptocurrency exchange.
*   `PUSH_NOTIFICATION_SERVICE_KEY`: The API key for the push notification service.
*   `ENCRYPTION_PRIVATE_KEY`: The private key used for encrypting push notifications. This should be securely stored and never exposed.
*   `ENCRYPTION_PUBLIC_KEY`: The public key corresponding to the private key. Stored in the smart contract for verification purposes.
*   `ALERT_CHECK_INTERVAL`: The interval (in seconds) at which the serverless functions check for price breaches.
*   `COIN_PAIR`: Coin pair to watch ex: BTC/USD.

## Usage

(Example smart contract interactions and API calls for alert creation and modification will be located here. Detailed API documentation to be included as well.)

The frontend can interact with the smart contract to create and manage alerts. Users must provide the smart contract address, alert parameters (cryptocurrency, exchange, threshold price), and their public encryption key. The frontend then calls the `createAlert` function on the smart contract, passing in these parameters. Serverless functions automatically query the smart contract for all active alerts, compare the current price with the configured threshold, and trigger encrypted push notifications when a breach is detected.

## Contributing

We welcome contributions to VolatileSentinel! Please follow these guidelines:

1.  Fork the repository.
2.  Create a new branch for your feature or bug fix.
3.  Write clear and concise code with thorough comments.
4.  Test your changes thoroughly.
5.  Submit a pull request with a detailed description of your changes.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/uhsr/VolatileSentinel/blob/main/LICENSE) file for details.

## Acknowledgements

(Optional: Acknowledge any third-party libraries or contributors.)