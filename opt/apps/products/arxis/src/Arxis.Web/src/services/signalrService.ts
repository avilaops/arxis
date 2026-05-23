import * as signalR from '@microsoft/signalr';

const HUB_URL = import.meta.env.VITE_API_URL 
  ? `${import.meta.env.VITE_API_URL.replace('/api', '')}/hubs/crm-notifications`
  : 'http://localhost:5136/hubs/crm-notifications';

export interface CrmNotification {
  type: string;
  message: string;
  timestamp: string;
  icon: string;
  color: string;
  leadId?: number;
  leadName?: string;
  opportunityId?: number;
  opportunityName?: string;
  activityId?: number;
  activityTitle?: string;
  [key: string]: any;
}

type NotificationCallback = (notification: CrmNotification) => void;

class SignalRService {
  private connection: signalR.HubConnection | null = null;
  private listeners: Map<string, NotificationCallback[]> = new Map();
  private reconnectAttempts = 0;
  private maxReconnectAttempts = 10;

  async connect(token: string): Promise<void> {
    if (this.connection?.state === signalR.HubConnectionState.Connected) {
      console.log('🔌 Já conectado ao SignalR');
      return;
    }

    this.connection = new signalR.HubConnectionBuilder()
      .withUrl(HUB_URL, {
        accessTokenFactory: () => token,
        skipNegotiation: false,
        transport: signalR.HttpTransportType.WebSockets | signalR.HttpTransportType.ServerSentEvents
      })
      .withAutomaticReconnect({
        nextRetryDelayInMilliseconds: () => {
          this.reconnectAttempts++;
          if (this.reconnectAttempts > this.maxReconnectAttempts) {
            return null; // Stop reconnecting
          }
          return Math.min(1000 * Math.pow(2, this.reconnectAttempts), 30000);
        }
      })
      .configureLogging(signalR.LogLevel.Information)
      .build();

    // Main notification handler
    this.connection.on('ReceiveNotification', (notification: CrmNotification) => {
      console.log('📢 Notificação recebida:', notification);
      this.notifyListeners('notification', notification);
      this.notifyListeners('all', notification);
    });

    // Specific handlers for different notification types
    this.connection.on('ReceiveLeadNotification', (notification: CrmNotification) => {
      console.log('👤 Novo lead:', notification);
      this.notifyListeners('lead', notification);
      this.notifyListeners('all', notification);
    });

    this.connection.on('ReceiveOpportunityNotification', (notification: CrmNotification) => {
      console.log('💼 Oportunidade:', notification);
      this.notifyListeners('opportunity', notification);
      this.notifyListeners('all', notification);
    });

    // Connection lifecycle handlers
    this.connection.onreconnecting(() => {
      console.log('🔄 Reconectando ao SignalR...');
    });

    this.connection.onreconnected(() => {
      console.log('✅ Reconectado ao SignalR');
      this.reconnectAttempts = 0;
    });

    this.connection.onclose((error) => {
      console.log('❌ Conexão SignalR fechada', error);
      if (this.reconnectAttempts < this.maxReconnectAttempts) {
        setTimeout(() => this.connect(token), 5000);
      }
    });

    try {
      await this.connection.start();
      console.log('✅ Conectado ao SignalR Hub:', HUB_URL);
      this.reconnectAttempts = 0;
    } catch (err) {
      console.error('❌ Erro ao conectar ao SignalR:', err);
      if (this.reconnectAttempts < this.maxReconnectAttempts) {
        setTimeout(() => this.connect(token), 5000);
      }
    }
  }

  async disconnect(): Promise<void> {
    if (this.connection) {
      await this.connection.stop();
      this.connection = null;
      console.log('🔌 Desconectado do SignalR');
    }
  }

  async joinChannel(channelName: string): Promise<void> {
    if (this.connection?.state === signalR.HubConnectionState.Connected) {
      try {
        await this.connection.invoke('JoinChannel', channelName);
        console.log(`📡 Entrou no canal: ${channelName}`);
      } catch (err) {
        console.error(`❌ Erro ao entrar no canal ${channelName}:`, err);
      }
    }
  }

  async leaveChannel(channelName: string): Promise<void> {
    if (this.connection?.state === signalR.HubConnectionState.Connected) {
      try {
        await this.connection.invoke('LeaveChannel', channelName);
        console.log(`📡 Saiu do canal: ${channelName}`);
      } catch (err) {
        console.error(`❌ Erro ao sair do canal ${channelName}:`, err);
      }
    }
  }

  onNotification(type: string, callback: NotificationCallback): void {
    if (!this.listeners.has(type)) {
      this.listeners.set(type, []);
    }
    this.listeners.get(type)!.push(callback);
  }

  offNotification(type: string, callback: NotificationCallback): void {
    const listeners = this.listeners.get(type);
    if (listeners) {
      const index = listeners.indexOf(callback);
      if (index > -1) {
        listeners.splice(index, 1);
      }
    }
  }

  private notifyListeners(type: string, notification: CrmNotification): void {
    const listeners = this.listeners.get(type);
    if (listeners) {
      listeners.forEach(callback => {
        try {
          callback(notification);
        } catch (err) {
          console.error('❌ Erro no callback de notificação:', err);
        }
      });
    }
  }

  isConnected(): boolean {
    return this.connection?.state === signalR.HubConnectionState.Connected;
  }

  getConnectionState(): signalR.HubConnectionState | null {
    return this.connection?.state ?? null;
  }
}

export const signalrService = new SignalRService();
