import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Grid,
  Paper,
  Card,
  CardContent,
  List,
  ListItem,
  ListItemText,
  Chip,
  Avatar,
  IconButton,
  Button,
  Divider,
  LinearProgress,
  Alert,
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableRow,
} from '@mui/material';
import {
  Speed as SpeedIcon,
  Warning as WarningIcon,
  CheckCircle as CheckIcon,
  TrendingUp as TrendingUpIcon,
  Phone as PhoneIcon,
  WhatsApp as WhatsAppIcon,
  Engineering as EngineeringIcon,
  AccessTime as TimeIcon,
  CalendarToday as CalendarIcon,
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import { salesService } from '../services/salesService';
import {
  SalesMetricsDto,
  SalesInboxItemDto,
  LeadTemperature,
  LeadTemperatureColors,
  LeadTemperatureLabels,
} from '../models/Obra';

const SalesDashboardObra: React.FC = () => {
  const navigate = useNavigate();
  const [metrics, setMetrics] = useState<SalesMetricsDto | null>(null);
  const [inbox, setInbox] = useState<SalesInboxItemDto[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);
      // TODO: Implementar endpoints reais
      const mockMetrics: SalesMetricsDto = {
        speedToLeadMinutosMedia: 45,
        speedToLeadUltimas24h: 12,
        dealsTotal: 34,
        dealsSemProximoPasso: 3,
        percentualSemProximoPasso: 8.8,
        diasParadoPorEtapa: {
          'NovoLead': 1.2,
          'QualificacaoTecnica': 3.5,
          'DiagnosticoVisita': 5.2,
          'PropostaEnviada': 7.8,
        },
        winRatePorOrigem: {
          'Indicacao': { totalDeals: 15, dealsGanhos: 9, dealsPerdidos: 3, winRate: 0.75, valorMedioGanho: 4500 },
          'Trafego': { totalDeals: 12, dealsGanhos: 4, dealsPerdidos: 5, winRate: 0.44, valorMedioGanho: 3200 },
          'Portal': { totalDeals: 7, dealsGanhos: 1, dealsPerdidos: 4, winRate: 0.2, valorMedioGanho: 2800 },
        },
        motivosPerdaContagem: {
          'Timing': 5,
          'Preço': 3,
          'Sem Fit': 2,
          'Concorrente': 2,
        },
        dealsAtrasados: 8,
        dealsVenceHoje: 5,
        dealsQuentes: 12,
        valorTotalPipeline: 185000,
      };

      const mockInbox: SalesInboxItemDto[] = [
        {
          id: '1',
          tipo: 'opportunity',
          nome: 'Obra Residencial Alto Padrão',
          empresa: 'Construtora ABC',
          status: 'PropostaEnviada',
          temperatura: LeadTemperature.Quente,
          proximoPasso: 'Follow-up decisão',
          proximoPassoEm: new Date(Date.now() - 2 * 24 * 60 * 60 * 1000).toISOString(),
          diasParado: 2,
          atrasado: true,
          venceHoje: false,
          valorEstimado: 5500,
          fitScore: 85,
          prioridade: 'alta',
          ultimaAtividade: 'Demo técnica realizada',
          ultimaAtividadeEm: new Date(Date.now() - 5 * 24 * 60 * 60 * 1000).toISOString(),
        },
        {
          id: '2',
          tipo: 'lead',
          nome: 'Incorporadora XYZ - Condomínio',
          empresa: 'Incorporadora XYZ',
          status: 'QualificacaoTecnica',
          temperatura: LeadTemperature.Quente,
          proximoPasso: 'Visita técnica na obra',
          proximoPassoEm: new Date().toISOString(),
          diasParado: 0,
          atrasado: false,
          venceHoje: true,
          valorEstimado: 8900,
          fitScore: 92,
          prioridade: 'alta',
          ultimaAtividade: 'Ligação de qualificação',
          ultimaAtividadeEm: new Date(Date.now() - 1 * 24 * 60 * 60 * 1000).toISOString(),
        },
        {
          id: '3',
          tipo: 'opportunity',
          nome: 'Shopping Center - Fase 2',
          empresa: 'Mega Obras Ltda',
          status: 'NegociacaoJuridico',
          temperatura: LeadTemperature.Quente,
          proximoPasso: 'Ajustar contrato',
          proximoPassoEm: new Date(Date.now() + 1 * 24 * 60 * 60 * 1000).toISOString(),
          diasParado: 12,
          atrasado: false,
          venceHoje: false,
          valorEstimado: 12000,
          fitScore: 88,
          prioridade: 'media',
          ultimaAtividade: 'Negociação comercial',
          ultimaAtividadeEm: new Date(Date.now() - 12 * 24 * 60 * 60 * 1000).toISOString(),
        },
      ];

      setMetrics(mockMetrics);
      setInbox(mockInbox);
    } catch (error) {
      console.error('Erro ao carregar dashboard:', error);
    } finally {
      setLoading(false);
    }
  };

  const getPrioridadeColor = (prioridade: string) => {
    switch (prioridade) {
      case 'alta': return '#ff4444';
      case 'media': return '#ffaa00';
      case 'baixa': return '#4444ff';
      default: return '#666666';
    }
  };

  const handleRegistrarToque = (id: string) => {
    console.log('Registrar toque para:', id);
    // TODO: Implementar modal de registro rápido
  };

  if (loading) {
    return (
      <Box sx={{ p: 3 }}>
        <LinearProgress />
        <Typography sx={{ mt: 2 }}>Carregando dashboard...</Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ p: 3 }}>
      {/* Header */}
      <Box sx={{ mb: 4 }}>
        <Typography variant="h4" sx={{ fontWeight: 'bold', mb: 1 }}>
          Vendas | Gestão de Obra
        </Typography>
        <Typography variant="body2" color="text.secondary">
          Dashboard focado em engenharia: qualificação técnica + follow-up + proposta com escopo
        </Typography>
      </Box>

      {/* Alertas Críticos */}
      {metrics && (metrics.dealsAtrasados > 0 || metrics.percentualSemProximoPasso > 5) && (
        <Grid container spacing={2} sx={{ mb: 3 }}>
          {metrics.dealsAtrasados > 0 && (
            <Grid item xs={12} md={6}>
              <Alert severity="error" icon={<WarningIcon />}>
                <strong>{metrics.dealsAtrasados} deals atrasados</strong> - Precisam de ação imediata!
              </Alert>
            </Grid>
          )}
          {metrics.percentualSemProximoPasso > 5 && (
            <Grid item xs={12} md={6}>
              <Alert severity="warning" icon={<TimeIcon />}>
                <strong>{metrics.percentualSemProximoPasso.toFixed(1)}% dos deals sem próximo passo</strong> - Defina ações!
              </Alert>
            </Grid>
          )}
        </Grid>
      )}

      {/* Métricas Principais - Engenharia First */}
      <Grid container spacing={3} sx={{ mb: 4 }}>
        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <Box>
                  <Typography variant="body2" color="text.secondary">
                    Speed-to-Lead
                  </Typography>
                  <Typography variant="h4" sx={{ fontWeight: 'bold', color: metrics?.speedToLeadUltimas24h! < 30 ? '#4caf50' : '#ff9800' }}>
                    {metrics?.speedToLeadUltimas24h}min
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    Média: {metrics?.speedToLeadMinutosMedia}min
                  </Typography>
                </Box>
                <Avatar sx={{ bgcolor: '#e3f2fd', color: '#2196f3' }}>
                  <SpeedIcon />
                </Avatar>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <Box>
                  <Typography variant="body2" color="text.secondary">
                    Sem Próximo Passo
                  </Typography>
                  <Typography variant="h4" sx={{ fontWeight: 'bold', color: metrics?.percentualSemProximoPasso! > 10 ? '#f44336' : '#4caf50' }}>
                    {metrics?.percentualSemProximoPasso.toFixed(0)}%
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    {metrics?.dealsSemProximoPasso} de {metrics?.dealsTotal} deals
                  </Typography>
                </Box>
                <Avatar sx={{ bgcolor: '#ffebee', color: '#f44336' }}>
                  <WarningIcon />
                </Avatar>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <Box>
                  <Typography variant="body2" color="text.secondary">
                    Deals Quentes
                  </Typography>
                  <Typography variant="h4" sx={{ fontWeight: 'bold', color: '#ff5722' }}>
                    {metrics?.dealsQuentes}
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    Alta probabilidade
                  </Typography>
                </Box>
                <Avatar sx={{ bgcolor: '#fbe9e7', color: '#ff5722' }}>
                  <TrendingUpIcon />
                </Avatar>
              </Box>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} md={3}>
          <Card>
            <CardContent>
              <Box sx={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <Box>
                  <Typography variant="body2" color="text.secondary">
                    Pipeline Total
                  </Typography>
                  <Typography variant="h4" sx={{ fontWeight: 'bold' }}>
                    R$ {(metrics?.valorTotalPipeline! / 1000).toFixed(0)}k
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    {metrics?.dealsTotal} oportunidades
                  </Typography>
                </Box>
                <Avatar sx={{ bgcolor: '#e8f5e9', color: '#4caf50' }}>
                  <CheckIcon />
                </Avatar>
              </Box>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      <Grid container spacing={3}>
        {/* Sales Inbox (Hoje) - Priorizado */}
        <Grid item xs={12} md={8}>
          <Paper sx={{ p: 3 }}>
            <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 2 }}>
              <Typography variant="h6" sx={{ fontWeight: 'bold' }}>
                📥 Sales Inbox (Hoje)
              </Typography>
              <Chip
                label={`${inbox.filter(i => i.atrasado).length} atrasados`}
                color="error"
                size="small"
              />
            </Box>

            <Typography variant="caption" color="text.secondary" sx={{ mb: 2, display: 'block' }}>
              Ordenado por: Atrasados → Vence Hoje → Quentes → Alto Valor
            </Typography>

            <List>
              {inbox.map((item, index) => (
                <React.Fragment key={item.id}>
                  {index > 0 && <Divider />}
                  <ListItem
                    sx={{
                      bgcolor: item.atrasado ? '#ffebee' : item.venceHoje ? '#fff3e0' : 'transparent',
                      borderLeft: `4px solid ${getPrioridadeColor(item.prioridade)}`,
                      mb: 1,
                      borderRadius: 1,
                    }}
                  >
                    <Box sx={{ flex: 1 }}>
                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1, mb: 1 }}>
                        <Typography variant="subtitle1" sx={{ fontWeight: 'bold' }}>
                          {item.nome}
                        </Typography>
                        <Chip
                          label={item.temperatura}
                          size="small"
                          sx={{
                            bgcolor: LeadTemperatureColors[item.temperatura],
                            color: 'white',
                            height: 20,
                          }}
                        />
                        <Chip
                          label={`FitScore: ${item.fitScore}`}
                          size="small"
                          variant="outlined"
                        />
                        {item.atrasado && (
                          <Chip label="ATRASADO" color="error" size="small" />
                        )}
                        {item.venceHoje && (
                          <Chip label="VENCE HOJE" color="warning" size="small" />
                        )}
                      </Box>

                      <Typography variant="body2" color="text.secondary" sx={{ mb: 0.5 }}>
                        {item.empresa} • {item.status}
                      </Typography>

                      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
                        <Typography variant="body2" sx={{ fontWeight: 'bold', color: '#1976d2' }}>
                          ▶ {item.proximoPasso}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          {new Date(item.proximoPassoEm!).toLocaleDateString('pt-BR')}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          • Parado há {item.diasParado} dias
                        </Typography>
                        {item.valorEstimado && (
                          <Typography variant="caption" sx={{ fontWeight: 'bold', color: '#4caf50' }}>
                            • R$ {item.valorEstimado.toLocaleString()}/mês
                          </Typography>
                        )}
                      </Box>
                    </Box>

                    <Box sx={{ display: 'flex', gap: 1 }}>
                      <Button
                        variant="contained"
                        size="small"
                        startIcon={<PhoneIcon />}
                        onClick={() => handleRegistrarToque(item.id)}
                        sx={{ textTransform: 'none' }}
                      >
                        Registrar Toque
                      </Button>
                      <IconButton size="small" color="success">
                        <WhatsAppIcon />
                      </IconButton>
                    </Box>
                  </ListItem>
                </React.Fragment>
              ))}
            </List>

            {inbox.length === 0 && (
              <Box sx={{ textAlign: 'center', py: 4 }}>
                <CheckIcon sx={{ fontSize: 48, color: '#4caf50', mb: 2 }} />
                <Typography variant="body1" color="text.secondary">
                  Nenhuma ação pendente no momento!
                </Typography>
              </Box>
            )}
          </Paper>
        </Grid>

        {/* Win Rate por Origem */}
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 3, mb: 3 }}>
            <Typography variant="h6" sx={{ fontWeight: 'bold', mb: 2 }}>
              🎯 Win Rate por Origem
            </Typography>

            {metrics && Object.entries(metrics.winRatePorOrigem).map(([origem, data]) => (
              <Box key={origem} sx={{ mb: 2 }}>
                <Box sx={{ display: 'flex', justifyContent: 'space-between', mb: 0.5 }}>
                  <Typography variant="body2">{origem}</Typography>
                  <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                    {(data.winRate * 100).toFixed(0)}%
                  </Typography>
                </Box>
                <LinearProgress
                  variant="determinate"
                  value={data.winRate * 100}
                  sx={{ height: 8, borderRadius: 4 }}
                />
                <Typography variant="caption" color="text.secondary">
                  {data.dealsGanhos} ganhos • Ticket: R$ {data.valorMedioGanho.toLocaleString()}
                </Typography>
              </Box>
            ))}
          </Paper>

          {/* Motivos de Perda */}
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" sx={{ fontWeight: 'bold', mb: 2 }}>
              ⚠️ Motivos de Perda
            </Typography>

            {metrics && Object.entries(metrics.motivosPerdaContagem).map(([motivo, count]) => (
              <Box key={motivo} sx={{ display: 'flex', justifyContent: 'space-between', mb: 1 }}>
                <Typography variant="body2">{motivo}</Typography>
                <Chip label={count} size="small" />
              </Box>
            ))}
          </Paper>
        </Grid>

        {/* Dias Parado por Etapa */}
        <Grid item xs={12}>
          <Paper sx={{ p: 3 }}>
            <Typography variant="h6" sx={{ fontWeight: 'bold', mb: 2 }}>
              ⏱️ Dias Parado por Etapa (Média)
            </Typography>

            <Table size="small">
              <TableHead>
                <TableRow>
                  <TableCell>Etapa</TableCell>
                  <TableCell align="right">Dias Médios</TableCell>
                  <TableCell align="right">Status</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {metrics && Object.entries(metrics.diasParadoPorEtapa).map(([etapa, dias]) => (
                  <TableRow key={etapa}>
                    <TableCell>{etapa}</TableCell>
                    <TableCell align="right">
                      <Typography
                        variant="body2"
                        sx={{
                          fontWeight: 'bold',
                          color: dias > 7 ? '#f44336' : dias > 3 ? '#ff9800' : '#4caf50'
                        }}
                      >
                        {dias.toFixed(1)} dias
                      </Typography>
                    </TableCell>
                    <TableCell align="right">
                      {dias > 7 && <Chip label="CRÍTICO" color="error" size="small" />}
                      {dias > 3 && dias <= 7 && <Chip label="ATENÇÃO" color="warning" size="small" />}
                      {dias <= 3 && <Chip label="OK" color="success" size="small" />}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </Paper>
        </Grid>
      </Grid>

      {/* Quick Actions */}
      <Box sx={{ mt: 3, display: 'flex', gap: 2 }}>
        <Button
          variant="outlined"
          startIcon={<EngineeringIcon />}
          onClick={() => navigate('/crm/leads')}
        >
          Ver Todos os Leads
        </Button>
        <Button
          variant="outlined"
          startIcon={<CalendarIcon />}
          onClick={() => navigate('/crm/cadences')}
        >
          Gerenciar Cadências
        </Button>
        <Button
          variant="outlined"
          onClick={() => navigate('/crm/opportunities')}
        >
          Pipeline Kanban
        </Button>
      </Box>
    </Box>
  );
};

export default SalesDashboardObra;
