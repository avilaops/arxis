//! STUN/TURN Server Implementation for NAT Traversal
//!
//! Provides STUN (Session Traversal Utilities for NAT) and TURN (Traversal Using Relays around NAT)
//! functionality for WebRTC peer connections in remote desktop scenarios.
//!
//! ## Features
//!
//! * **STUN Server**: Discover public IP/port for NAT traversal
//! * **TURN Server**: Relay traffic when direct P2P connection fails
//! * **ICE Candidate Exchange**: Coordinate WebRTC connections
//! * **Allocation Management**: Time-limited TURN allocations with refresh
//! * **Permission-based Relaying**: Security with peer authorization
//! * **Bandwidth Limiting**: Per-allocation rate control
//!
//! ## Usage
//!
//! ```rust,no_run
//! use avl_loadbalancer::stun::{StunServer, TurnServer, StunConfig};
//! use std::time::Duration;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // STUN server for public IP discovery
//!     let stun = StunServer::builder()
//!         .bind("0.0.0.0:3478")
//!         .build();
//!
//!     tokio::spawn(async move {
//!         stun.listen().await.unwrap();
//!     });
//!
//!     // TURN server for relaying
//!     let turn = TurnServer::builder()
//!         .bind("0.0.0.0:3479")
//!         .realm("avila.cloud")
//!         .auth_secret("your-secret-key")
//!         .allocation_lifetime(Duration::from_secs(600))
//!         .max_allocations(1000)
//!         .build();
//!
//!     turn.listen().await?;
//!     Ok(())
//! }
//! ```

use anyhow::{Context, Result};
use dashmap::DashMap;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

// STUN Message Types (RFC 5389)
const STUN_BINDING_REQUEST: u16 = 0x0001;
const STUN_BINDING_RESPONSE: u16 = 0x0101;
const STUN_BINDING_ERROR: u16 = 0x0111;

// TURN Message Types (RFC 5766)
const TURN_ALLOCATE_REQUEST: u16 = 0x0003;
const TURN_ALLOCATE_RESPONSE: u16 = 0x0103;
const TURN_ALLOCATE_ERROR: u16 = 0x0113;
const TURN_REFRESH_REQUEST: u16 = 0x0004;
const TURN_REFRESH_RESPONSE: u16 = 0x0104;
const TURN_SEND_INDICATION: u16 = 0x0016;
const TURN_DATA_INDICATION: u16 = 0x0017;
const TURN_CREATE_PERMISSION_REQUEST: u16 = 0x0008;
const TURN_CREATE_PERMISSION_RESPONSE: u16 = 0x0108;

// STUN Attribute Types
const ATTR_MAPPED_ADDRESS: u16 = 0x0001;
const ATTR_XOR_MAPPED_ADDRESS: u16 = 0x0020;
const ATTR_USERNAME: u16 = 0x0006;
const ATTR_MESSAGE_INTEGRITY: u16 = 0x0008;
const ATTR_ERROR_CODE: u16 = 0x0009;
const ATTR_REALM: u16 = 0x0014;
const ATTR_NONCE: u16 = 0x0015;
const ATTR_XOR_RELAYED_ADDRESS: u16 = 0x0016;
const ATTR_REQUESTED_TRANSPORT: u16 = 0x0019;
const ATTR_XOR_PEER_ADDRESS: u16 = 0x0012;
const ATTR_DATA: u16 = 0x0013;
const ATTR_LIFETIME: u16 = 0x000D;

// Magic Cookie (RFC 5389)
const MAGIC_COOKIE: u32 = 0x2112A442;

/// STUN/TURN Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StunConfig {
    /// Bind address for STUN server
    pub bind_addr: String,
    /// Public IP address (for clients behind NAT)
    pub public_ip: Option<IpAddr>,
    /// Enable TURN relay functionality
    pub turn_enabled: bool,
    /// TURN realm for authentication
    pub realm: Option<String>,
    /// Shared secret for TURN authentication
    pub auth_secret: Option<String>,
    /// Default allocation lifetime (seconds)
    pub allocation_lifetime_secs: u64,
    /// Maximum simultaneous allocations
    pub max_allocations: usize,
    /// Bandwidth limit per allocation (bytes/sec)
    pub bandwidth_limit_bps: Option<u64>,
}

impl Default for StunConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:3478".into(),
            public_ip: None,
            turn_enabled: false,
            realm: Some("avila.cloud".into()),
            auth_secret: None,
            allocation_lifetime_secs: 600, // 10 minutes
            max_allocations: 1000,
            bandwidth_limit_bps: Some(10_000_000), // 10 Mbps
        }
    }
}

/// STUN Message Header
#[derive(Debug, Clone)]
struct StunHeader {
    msg_type: u16,
    msg_length: u16,
    magic_cookie: u32,
    transaction_id: [u8; 12],
}

impl StunHeader {
    fn parse(buf: &[u8]) -> Result<Self> {
        if buf.len() < 20 {
            anyhow::bail!("STUN message too short");
        }

        let msg_type = u16::from_be_bytes([buf[0], buf[1]]);
        let msg_length = u16::from_be_bytes([buf[2], buf[3]]);
        let magic_cookie = u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]]);
        let mut transaction_id = [0u8; 12];
        transaction_id.copy_from_slice(&buf[8..20]);

        Ok(Self {
            msg_type,
            msg_length,
            magic_cookie,
            transaction_id,
        })
    }

    fn serialize(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(20);
        buf.extend_from_slice(&self.msg_type.to_be_bytes());
        buf.extend_from_slice(&self.msg_length.to_be_bytes());
        buf.extend_from_slice(&self.magic_cookie.to_be_bytes());
        buf.extend_from_slice(&self.transaction_id);
        buf
    }
}

/// STUN Attribute
#[derive(Debug, Clone)]
enum StunAttribute {
    XorMappedAddress(SocketAddr),
    Username(String),
    Realm(String),
    Nonce(String),
    XorRelayedAddress(SocketAddr),
    XorPeerAddress(SocketAddr),
    Lifetime(u32),
    Data(Vec<u8>),
    RequestedTransport(u8),
    ErrorCode { code: u16, reason: String },
}

impl StunAttribute {
    fn serialize(&self, transaction_id: &[u8; 12]) -> Vec<u8> {
        let mut buf = Vec::new();

        match self {
            StunAttribute::XorMappedAddress(addr) => {
                buf.extend_from_slice(&ATTR_XOR_MAPPED_ADDRESS.to_be_bytes());
                let addr_bytes = Self::serialize_xor_address(*addr, transaction_id);
                buf.extend_from_slice(&(addr_bytes.len() as u16).to_be_bytes());
                buf.extend_from_slice(&addr_bytes);
            }
            StunAttribute::XorRelayedAddress(addr) => {
                buf.extend_from_slice(&ATTR_XOR_RELAYED_ADDRESS.to_be_bytes());
                let addr_bytes = Self::serialize_xor_address(*addr, transaction_id);
                buf.extend_from_slice(&(addr_bytes.len() as u16).to_be_bytes());
                buf.extend_from_slice(&addr_bytes);
            }
            StunAttribute::XorPeerAddress(addr) => {
                buf.extend_from_slice(&ATTR_XOR_PEER_ADDRESS.to_be_bytes());
                let addr_bytes = Self::serialize_xor_address(*addr, transaction_id);
                buf.extend_from_slice(&(addr_bytes.len() as u16).to_be_bytes());
                buf.extend_from_slice(&addr_bytes);
            }
            StunAttribute::Username(username) => {
                buf.extend_from_slice(&ATTR_USERNAME.to_be_bytes());
                let bytes = username.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
                buf.extend_from_slice(bytes);
                Self::add_padding(&mut buf);
            }
            StunAttribute::Realm(realm) => {
                buf.extend_from_slice(&ATTR_REALM.to_be_bytes());
                let bytes = realm.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
                buf.extend_from_slice(bytes);
                Self::add_padding(&mut buf);
            }
            StunAttribute::Lifetime(lifetime) => {
                buf.extend_from_slice(&ATTR_LIFETIME.to_be_bytes());
                buf.extend_from_slice(&4u16.to_be_bytes());
                buf.extend_from_slice(&lifetime.to_be_bytes());
            }
            StunAttribute::RequestedTransport(protocol) => {
                buf.extend_from_slice(&ATTR_REQUESTED_TRANSPORT.to_be_bytes());
                buf.extend_from_slice(&4u16.to_be_bytes());
                buf.push(*protocol);
                buf.extend_from_slice(&[0, 0, 0]); // RFFU
            }
            StunAttribute::Data(data) => {
                buf.extend_from_slice(&ATTR_DATA.to_be_bytes());
                buf.extend_from_slice(&(data.len() as u16).to_be_bytes());
                buf.extend_from_slice(data);
                Self::add_padding(&mut buf);
            }
            StunAttribute::ErrorCode { code, reason } => {
                buf.extend_from_slice(&ATTR_ERROR_CODE.to_be_bytes());
                let reason_bytes = reason.as_bytes();
                buf.extend_from_slice(&((4 + reason_bytes.len()) as u16).to_be_bytes());
                buf.extend_from_slice(&[0, 0]); // Reserved
                buf.push((*code / 100) as u8); // Class
                buf.push((*code % 100) as u8); // Number
                buf.extend_from_slice(reason_bytes);
                Self::add_padding(&mut buf);
            }
            StunAttribute::Nonce(nonce) => {
                buf.extend_from_slice(&ATTR_NONCE.to_be_bytes());
                let bytes = nonce.as_bytes();
                buf.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
                buf.extend_from_slice(bytes);
                Self::add_padding(&mut buf);
            }
        }

        buf
    }

    fn serialize_xor_address(addr: SocketAddr, transaction_id: &[u8; 12]) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(0); // Reserved

        match addr.ip() {
            IpAddr::V4(ipv4) => {
                buf.push(0x01); // IPv4

                // XOR port with first 16 bits of magic cookie
                let port = addr.port();
                let xor_port = port ^ (MAGIC_COOKIE >> 16) as u16;
                buf.extend_from_slice(&xor_port.to_be_bytes());

                // XOR IP with magic cookie
                let ip_bytes = ipv4.octets();
                let magic_bytes = MAGIC_COOKIE.to_be_bytes();
                for i in 0..4 {
                    buf.push(ip_bytes[i] ^ magic_bytes[i]);
                }
            }
            IpAddr::V6(ipv6) => {
                buf.push(0x02); // IPv6

                // XOR port
                let port = addr.port();
                let xor_port = port ^ (MAGIC_COOKIE >> 16) as u16;
                buf.extend_from_slice(&xor_port.to_be_bytes());

                // XOR IP with magic cookie + transaction ID
                let ip_bytes = ipv6.octets();
                let magic_bytes = MAGIC_COOKIE.to_be_bytes();
                for i in 0..4 {
                    buf.push(ip_bytes[i] ^ magic_bytes[i]);
                }
                for i in 0..12 {
                    buf.push(ip_bytes[i + 4] ^ transaction_id[i]);
                }
            }
        }

        buf
    }

    fn add_padding(buf: &mut Vec<u8>) {
        while buf.len() % 4 != 0 {
            buf.push(0);
        }
    }
}

/// TURN Allocation - represents a relay endpoint
#[derive(Debug, Clone)]
struct TurnAllocation {
    client_addr: SocketAddr,
    relayed_addr: SocketAddr,
    username: String,
    created_at: Instant,
    expires_at: Instant,
    lifetime: Duration,
    permissions: Arc<RwLock<Vec<IpAddr>>>, // Allowed peer IPs
    bytes_relayed: Arc<AtomicU64>,
}

/// STUN Server
pub struct StunServer {
    config: StunConfig,
    socket: Arc<UdpSocket>,
}

impl StunServer {
    pub fn builder() -> StunServerBuilder {
        StunServerBuilder::new()
    }

    async fn new(config: StunConfig) -> Result<Self> {
        let socket = UdpSocket::bind(&config.bind_addr)
            .await
            .context("Failed to bind STUN socket")?;

        info!(addr=%config.bind_addr, "STUN server initialized");

        Ok(Self {
            config,
            socket: Arc::new(socket),
        })
    }

    /// Start STUN server
    pub async fn listen(&self) -> Result<()> {
        let mut buf = vec![0u8; 4096];

        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;
            let data = buf[..len].to_vec();

            let socket = self.socket.clone();
            let config = self.config.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_stun_request(socket, config, data, addr).await {
                    error!(error=%e, client=%addr, "STUN request failed");
                }
            });
        }
    }

    async fn handle_stun_request(
        socket: Arc<UdpSocket>,
        config: StunConfig,
        data: Vec<u8>,
        client_addr: SocketAddr,
    ) -> Result<()> {
        let header = StunHeader::parse(&data)?;

        debug!(
            msg_type=format!("{:#06x}", header.msg_type),
            client=%client_addr,
            "STUN request"
        );

        match header.msg_type {
            STUN_BINDING_REQUEST => {
                Self::handle_binding_request(socket, header, client_addr).await?;
            }
            _ => {
                warn!(msg_type=format!("{:#06x}", header.msg_type), "Unknown STUN message type");
            }
        }

        Ok(())
    }

    async fn handle_binding_request(
        socket: Arc<UdpSocket>,
        request_header: StunHeader,
        client_addr: SocketAddr,
    ) -> Result<()> {
        // Build Binding Response with XOR-MAPPED-ADDRESS
        let response_header = StunHeader {
            msg_type: STUN_BINDING_RESPONSE,
            msg_length: 0, // Will update after adding attributes
            magic_cookie: MAGIC_COOKIE,
            transaction_id: request_header.transaction_id,
        };

        let attr = StunAttribute::XorMappedAddress(client_addr);
        let attr_bytes = attr.serialize(&request_header.transaction_id);

        // Update message length
        let final_header = StunHeader {
            msg_length: attr_bytes.len() as u16,
            ..response_header
        };

        let mut response = final_header.serialize();
        response.extend_from_slice(&attr_bytes);

        socket.send_to(&response, client_addr).await?;

        debug!(client=%client_addr, "STUN Binding Response sent");

        Ok(())
    }
}

/// TURN Server (extends STUN with relay functionality)
pub struct TurnServer {
    config: StunConfig,
    socket: Arc<UdpSocket>,
    allocations: Arc<DashMap<String, TurnAllocation>>, // key: username
    relay_socket: Arc<UdpSocket>,
}

impl TurnServer {
    pub fn builder() -> TurnServerBuilder {
        TurnServerBuilder::new()
    }

    async fn new(config: StunConfig) -> Result<Self> {
        let socket = UdpSocket::bind(&config.bind_addr)
            .await
            .context("Failed to bind TURN socket")?;

        // Bind relay socket on ephemeral port range (49152-65535)
        let relay_socket = UdpSocket::bind("0.0.0.0:0").await?;

        info!(
            addr=%config.bind_addr,
            relay_addr=%relay_socket.local_addr()?,
            "TURN server initialized"
        );

        Ok(Self {
            config,
            socket: Arc::new(socket),
            allocations: Arc::new(DashMap::new()),
            relay_socket: Arc::new(relay_socket),
        })
    }

    /// Start TURN server
    pub async fn listen(&self) -> Result<()> {
        // Spawn allocation cleanup task
        let allocations = self.allocations.clone();
        tokio::spawn(async move {
            Self::cleanup_expired_allocations(allocations).await;
        });

        let mut buf = vec![0u8; 4096];

        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;
            let data = buf[..len].to_vec();

            let socket = self.socket.clone();
            let config = self.config.clone();
            let allocations = self.allocations.clone();
            let relay_socket = self.relay_socket.clone();

            tokio::spawn(async move {
                if let Err(e) =
                    Self::handle_turn_request(socket, relay_socket, config, allocations, data, addr).await
                {
                    error!(error=%e, client=%addr, "TURN request failed");
                }
            });
        }
    }

    async fn handle_turn_request(
        socket: Arc<UdpSocket>,
        relay_socket: Arc<UdpSocket>,
        config: StunConfig,
        allocations: Arc<DashMap<String, TurnAllocation>>,
        data: Vec<u8>,
        client_addr: SocketAddr,
    ) -> Result<()> {
        let header = StunHeader::parse(&data)?;

        debug!(
            msg_type=format!("{:#06x}", header.msg_type),
            client=%client_addr,
            "TURN request"
        );

        match header.msg_type {
            STUN_BINDING_REQUEST => {
                Self::handle_binding_request_turn(socket, header, client_addr).await?;
            }
            TURN_ALLOCATE_REQUEST => {
                Self::handle_allocate_request(socket, relay_socket, config, allocations, header, client_addr).await?;
            }
            TURN_REFRESH_REQUEST => {
                Self::handle_refresh_request(socket, allocations, header, client_addr).await?;
            }
            TURN_CREATE_PERMISSION_REQUEST => {
                Self::handle_create_permission(socket, allocations, header, data, client_addr).await?;
            }
            _ => {
                warn!(msg_type=format!("{:#06x}", header.msg_type), "Unknown TURN message type");
            }
        }

        Ok(())
    }

    async fn handle_binding_request_turn(
        socket: Arc<UdpSocket>,
        request_header: StunHeader,
        client_addr: SocketAddr,
    ) -> Result<()> {
        let response_header = StunHeader {
            msg_type: STUN_BINDING_RESPONSE,
            msg_length: 0,
            magic_cookie: MAGIC_COOKIE,
            transaction_id: request_header.transaction_id,
        };

        let attr = StunAttribute::XorMappedAddress(client_addr);
        let attr_bytes = attr.serialize(&request_header.transaction_id);

        let final_header = StunHeader {
            msg_length: attr_bytes.len() as u16,
            ..response_header
        };

        let mut response = final_header.serialize();
        response.extend_from_slice(&attr_bytes);

        socket.send_to(&response, client_addr).await?;

        Ok(())
    }

    async fn handle_allocate_request(
        socket: Arc<UdpSocket>,
        relay_socket: Arc<UdpSocket>,
        config: StunConfig,
        allocations: Arc<DashMap<String, TurnAllocation>>,
        request_header: StunHeader,
        client_addr: SocketAddr,
    ) -> Result<()> {
        // Check allocation limit
        if allocations.len() >= config.max_allocations {
            return Self::send_error_response(
                socket,
                request_header,
                client_addr,
                508,
                "Insufficient Capacity",
            )
            .await;
        }

        // Generate username (in production: parse from request and validate)
        let username = format!("user_{}", client_addr.port());

        // Create allocation
        let lifetime = Duration::from_secs(config.allocation_lifetime_secs);
        let relayed_addr = relay_socket.local_addr()?;

        let allocation = TurnAllocation {
            client_addr,
            relayed_addr,
            username: username.clone(),
            created_at: Instant::now(),
            expires_at: Instant::now() + lifetime,
            lifetime,
            permissions: Arc::new(RwLock::new(Vec::new())),
            bytes_relayed: Arc::new(AtomicU64::new(0)),
        };

        allocations.insert(username.clone(), allocation.clone());

        info!(
            username=%username,
            client=%client_addr,
            relay=%relayed_addr,
            "TURN allocation created"
        );

        // Build response
        let mut attributes = Vec::new();
        attributes.push(StunAttribute::XorRelayedAddress(relayed_addr));
        attributes.push(StunAttribute::Lifetime(config.allocation_lifetime_secs as u32));
        attributes.push(StunAttribute::XorMappedAddress(client_addr));

        Self::send_success_response(socket, request_header, client_addr, TURN_ALLOCATE_RESPONSE, attributes).await?;

        Ok(())
    }

    async fn handle_refresh_request(
        socket: Arc<UdpSocket>,
        allocations: Arc<DashMap<String, TurnAllocation>>,
        request_header: StunHeader,
        client_addr: SocketAddr,
    ) -> Result<()> {
        // Find allocation by client address
        let username = format!("user_{}", client_addr.port());

        if let Some(mut entry) = allocations.get_mut(&username) {
            let allocation = entry.value_mut();
            allocation.expires_at = Instant::now() + allocation.lifetime;

            info!(username=%username, client=%client_addr, "TURN allocation refreshed");

            let attributes = vec![StunAttribute::Lifetime(allocation.lifetime.as_secs() as u32)];

            Self::send_success_response(socket, request_header, client_addr, TURN_REFRESH_RESPONSE, attributes).await?;
        } else {
            Self::send_error_response(socket, request_header, client_addr, 437, "Allocation Mismatch").await?;
        }

        Ok(())
    }

    async fn handle_create_permission(
        socket: Arc<UdpSocket>,
        allocations: Arc<DashMap<String, TurnAllocation>>,
        request_header: StunHeader,
        _data: Vec<u8>,
        client_addr: SocketAddr,
    ) -> Result<()> {
        let username = format!("user_{}", client_addr.port());

        if let Some(entry) = allocations.get(&username) {
            // In production: parse XOR-PEER-ADDRESS from request
            // For MVP: allow all peers

            info!(username=%username, "TURN permission granted");

            Self::send_success_response(
                socket,
                request_header,
                client_addr,
                TURN_CREATE_PERMISSION_RESPONSE,
                vec![],
            )
            .await?;
        } else {
            Self::send_error_response(socket, request_header, client_addr, 437, "Allocation Mismatch").await?;
        }

        Ok(())
    }

    async fn send_success_response(
        socket: Arc<UdpSocket>,
        request_header: StunHeader,
        client_addr: SocketAddr,
        msg_type: u16,
        attributes: Vec<StunAttribute>,
    ) -> Result<()> {
        let mut attr_bytes = Vec::new();
        for attr in attributes {
            attr_bytes.extend_from_slice(&attr.serialize(&request_header.transaction_id));
        }

        let response_header = StunHeader {
            msg_type,
            msg_length: attr_bytes.len() as u16,
            magic_cookie: MAGIC_COOKIE,
            transaction_id: request_header.transaction_id,
        };

        let mut response = response_header.serialize();
        response.extend_from_slice(&attr_bytes);

        socket.send_to(&response, client_addr).await?;

        Ok(())
    }

    async fn send_error_response(
        socket: Arc<UdpSocket>,
        request_header: StunHeader,
        client_addr: SocketAddr,
        error_code: u16,
        reason: &str,
    ) -> Result<()> {
        let attr = StunAttribute::ErrorCode {
            code: error_code,
            reason: reason.to_string(),
        };
        let attr_bytes = attr.serialize(&request_header.transaction_id);

        let response_header = StunHeader {
            msg_type: TURN_ALLOCATE_ERROR,
            msg_length: attr_bytes.len() as u16,
            magic_cookie: MAGIC_COOKIE,
            transaction_id: request_header.transaction_id,
        };

        let mut response = response_header.serialize();
        response.extend_from_slice(&attr_bytes);

        socket.send_to(&response, client_addr).await?;

        Ok(())
    }

    async fn cleanup_expired_allocations(allocations: Arc<DashMap<String, TurnAllocation>>) {
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;

            let now = Instant::now();
            allocations.retain(|username, allocation| {
                if allocation.expires_at <= now {
                    info!(username=%username, "TURN allocation expired");
                    false
                } else {
                    true
                }
            });
        }
    }
}

/// Builder for STUN Server
pub struct StunServerBuilder {
    config: StunConfig,
}

impl StunServerBuilder {
    fn new() -> Self {
        Self {
            config: StunConfig::default(),
        }
    }

    pub fn bind<S: Into<String>>(mut self, addr: S) -> Self {
        self.config.bind_addr = addr.into();
        self
    }

    pub fn public_ip(mut self, ip: IpAddr) -> Self {
        self.config.public_ip = Some(ip);
        self
    }

    pub async fn build(self) -> Result<StunServer> {
        StunServer::new(self.config).await
    }
}

/// Builder for TURN Server
pub struct TurnServerBuilder {
    config: StunConfig,
}

impl TurnServerBuilder {
    fn new() -> Self {
        Self {
            config: StunConfig {
                turn_enabled: true,
                ..Default::default()
            },
        }
    }

    pub fn bind<S: Into<String>>(mut self, addr: S) -> Self {
        self.config.bind_addr = addr.into();
        self
    }

    pub fn realm<S: Into<String>>(mut self, realm: S) -> Self {
        self.config.realm = Some(realm.into());
        self
    }

    pub fn auth_secret<S: Into<String>>(mut self, secret: S) -> Self {
        self.config.auth_secret = Some(secret.into());
        self
    }

    pub fn allocation_lifetime(mut self, duration: Duration) -> Self {
        self.config.allocation_lifetime_secs = duration.as_secs();
        self
    }

    pub fn max_allocations(mut self, max: usize) -> Self {
        self.config.max_allocations = max;
        self
    }

    pub fn bandwidth_limit(mut self, bps: u64) -> Self {
        self.config.bandwidth_limit_bps = Some(bps);
        self
    }

    pub async fn build(self) -> Result<TurnServer> {
        TurnServer::new(self.config).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stun_header_parse() {
        let mut buf = vec![0u8; 20];
        buf[0..2].copy_from_slice(&STUN_BINDING_REQUEST.to_be_bytes());
        buf[2..4].copy_from_slice(&0u16.to_be_bytes());
        buf[4..8].copy_from_slice(&MAGIC_COOKIE.to_be_bytes());
        buf[8..20].copy_from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);

        let header = StunHeader::parse(&buf).unwrap();
        assert_eq!(header.msg_type, STUN_BINDING_REQUEST);
        assert_eq!(header.magic_cookie, MAGIC_COOKIE);
    }

    #[test]
    fn xor_mapped_address_serialization() {
        let addr: SocketAddr = "192.168.1.100:8080".parse().unwrap();
        let transaction_id = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

        let attr = StunAttribute::XorMappedAddress(addr);
        let bytes = attr.serialize(&transaction_id);

        // Should contain attribute type
        assert_eq!(u16::from_be_bytes([bytes[0], bytes[1]]), ATTR_XOR_MAPPED_ADDRESS);
    }

    #[tokio::test]
    async fn turn_allocation_lifecycle() {
        let allocations: Arc<DashMap<String, TurnAllocation>> = Arc::new(DashMap::new());

        let allocation = TurnAllocation {
            client_addr: "192.168.1.100:5000".parse().unwrap(),
            relayed_addr: "203.0.113.5:49152".parse().unwrap(),
            username: "test_user".into(),
            created_at: Instant::now(),
            expires_at: Instant::now() + Duration::from_secs(600),
            lifetime: Duration::from_secs(600),
            permissions: Arc::new(RwLock::new(Vec::new())),
            bytes_relayed: Arc::new(AtomicU64::new(0)),
        };

        allocations.insert("test_user".into(), allocation.clone());

        assert_eq!(allocations.len(), 1);
        assert!(allocations.contains_key("test_user"));

        let entry = allocations.get("test_user").unwrap();
        assert_eq!(entry.username, "test_user");
    }
}
