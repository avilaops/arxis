//! Mycelium - Rede de comunicação P2P

use crate::{SporeData, NodeState, PeerInfo};
use avila_error::{Error, ErrorKind, Result};
use avila_id::Id;
use avila_molecule::tcp::TcpServer;
use avila_molecule::NetworkAddress;
use avila_atom::{Map, DynamicList};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Mycelium - Rede micélica distribuída
pub struct Mycelium {
    /// ID deste nó
    pub id: String,
    /// Nome amigável
    pub name: String,
    /// Endereço onde este nó escuta
    pub address: String,
    /// Peers conectados
    peers: Arc<RwLock<Map<String, PeerInfo>>>,
    /// Esporos recebidos (cache para evitar duplicatas)
    spore_cache: Arc<RwLock<Map<String, i64>>>,
    /// Servidor TCP para aceitar conexões
    server: Option<TcpServer>,
}

impl Mycelium {
    /// Cria novo nó do micélio
    pub async fn new(name: impl Into<String>, address: impl AsRef<str>) -> Result<Self> {
        use avila_id::Id;
        let addr_str = address.as_ref();

        // Parse "host:port"
        let parts: Vec<&str> = addr_str.split(':').collect();
        if parts.len() != 2 {
            return Err(Error::new(ErrorKind::InvalidInput, "Address must be host:port"));
        }

        let host = parts[0];
        let port: u16 = parts[1].parse()
            .map_err(|_| Error::new(ErrorKind::InvalidInput, "Invalid port"))?;

        let net_addr = NetworkAddress::new(host, port);
        let server = TcpServer::bind(net_addr).await?;

        Ok(Self {
            id: Id::new().to_string(),
            name: name.into(),
            address: addr_str.to_string(),
            peers: Arc::new(RwLock::new(Map::new())),
            spore_cache: Arc::new(RwLock::new(Map::new())),
            server: Some(server),
        })
    }

    /// Inicia o servidor e aceita conexões
    pub async fn start(&mut self) -> Result<()> {
        println!("🍄 Mycelium {} iniciado em {}", self.name, self.address);

        // TODO: Loop de aceitação de conexões
        // let server = self.server.take().ok_or_else(|| {
        //     Error::new(ErrorKind::Internal, "Server already started")
        // })?;

        Ok(())
    }

    /// Conecta a um peer
    pub async fn connect_to_peer(&self, address: impl AsRef<str>) -> Result<()> {
        use avila_id::Id;
        use avila_time::DateTime;

        let peer_addr = address.as_ref().to_string();

        // Criar peer info (conexão será feita sob demanda)
        let peer_info = PeerInfo {
            state: NodeState {
                id: Id::new().to_string(),
                name: "unknown".to_string(),
                address: peer_addr.clone(),
                last_seen: DateTime::now().timestamp() as i64,
                metadata: Map::new(),
            },
            address: peer_addr.clone(),
        };

        let mut peers = self.peers.write().await;
        peers.insert(peer_addr.clone(), peer_info);

        println!("🔗 Conectado ao peer: {}", peer_addr);

        Ok(())
    }    /// Libera um esporo na rede (broadcast)
    pub async fn release_spore(&self, data_type: impl Into<String>, payload: Vec<u8>) -> Result<()> {
        let spore = SporeData::new(data_type, payload, 10); // TTL 10 hops

        // Adicionar ao cache
        {
            let mut cache = self.spore_cache.write().await;
            cache.insert(spore.id.to_string(), spore.created_at.clone());
        }

        // Propagar para todos os peers
        self.propagate_spore(&spore).await?;

        Ok(())
    }

    /// Propaga esporo para peers conectados
    async fn propagate_spore(&self, spore: &SporeData) -> Result<()> {
        let peers = self.peers.read().await;

        println!("🍄 Propagando esporo {} para {} peers", spore.id, peers.len());

        // TODO: Enviar via conexões TCP quando implementadas
        // let spore_json = serde_json::to_vec(spore)
        //     .map_err(|e| Error::new(ErrorKind::Serialization, e.to_string()))?;

        // for (addr, peer) in peers.iter() {
        //     // Conectar e enviar
        // }

        Ok(())
    }

    /// Retorna estado deste nó
    pub fn get_state(&self) -> NodeState {
        use avila_time::DateTime;

        NodeState {
            id: self.id.clone(),
            name: self.name.clone(),
            address: self.address.clone(),
            last_seen: DateTime::now().timestamp() as i64,
            metadata: Map::new(),
        }
    }

    /// Lista peers conectados
    pub async fn list_peers(&self) -> DynamicList<String> {
        let peers = self.peers.read().await;
        peers.keys().cloned().collect()
    }

    /// Número de peers conectados
    pub async fn peer_count(&self) -> usize {
        let peers = self.peers.read().await;
        peers.len()
    }
}
