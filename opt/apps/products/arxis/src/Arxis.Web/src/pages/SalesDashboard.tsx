import React, { useState, useEffect } from 'react';
import {
  Box,
  Grid,
  Card,
  CardContent,
  Typography,
  Chip,
  List,
  ListItem,
  ListItemText,
  ListItemAvatar,
  Avatar,
  IconButton,
  Button,
  Tabs,
  Tab,
  Paper,
  Divider,
  Badge,
} from '@mui/material';
import {
  TrendingUp,
  PersonAdd,
  Assignment,
  Schedule,
  Email,
  Phone,
  WhatsApp,
  PriorityHigh,
  Today,
  LocalFireDepartment,
  AttachMoney,
} from '@mui/icons-material';
import { useNavigate } from 'react-router-dom';
import { salesService } from '../services/salesService';

interface SalesDashboardDto {
  totalLeads: number;
  newLeadsToday: number;
  hotLeads: number;
  totalOpportunities: number;
  totalPipelineValue: number;
  activitiesToday: number;
  overdueActivities: number;
  inboxItems: InboxItemDto[];
  leadsByStatus: Record<string, number>;
  opportunitiesByStage: Record<string, number>;
}

interface InboxItemDto {
  id: string;
  type: string;
  title: string;
  description: string;
  priority: string;
  dueDate?: string;
  ownerName?: string;
  temperature?: string;
  value?: number;
}

const SalesDashboard: React.FC = () => {
  const [dashboardData, setDashboardData] = useState<SalesDashboardDto | null>(null);
  const [loading, setLoading] = useState(true);
  const [activeTab, setActiveTab] = useState(0);
  const navigate = useNavigate();

  useEffect(() => {
    fetchDashboardData();
  }, []);

  const fetchDashboardData = async () => {
    try {
      const data = await salesService.getDashboard();
      setDashboardData(data);
    } catch (error) {
      console.error('Error fetching dashboard data:', error);
    } finally {
      setLoading(false);
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'overdue': return 'error';
      case 'today': return 'warning';
      case 'hot': return 'error';
      case 'high_value': return 'success';
      default: return 'default';
    }
  };

  const getPriorityIcon = (priority: string) => {
    switch (priority) {
      case 'overdue': return <PriorityHigh />;
      case 'today': return <Today />;
      case 'hot': return <LocalFireDepartment />;
      case 'high_value': return <AttachMoney />;
      default: return <Assignment />;
    }
  };

  const getTemperatureColor = (temperature?: string) => {
    switch (temperature) {
      case 'hot': return 'error';
      case 'warm': return 'warning';
      case 'cold': return 'info';
      default: return 'default';
    }
  };

  const handleInboxItemClick = (item: InboxItemDto) => {
    switch (item.type) {
      case 'lead':
        navigate('/crm/leads');
        break;
      case 'opportunity':
        navigate('/crm/opportunities');
        break;
      case 'activity':
        navigate('/crm/activities');
        break;
      default:
        break;
    }
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <Typography>Carregando dashboard...</Typography>
      </Box>
    );
  }

  if (!dashboardData) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <Typography>Erro ao carregar dados do dashboard</Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ flexGrow: 1, p: 3 }}>
      <Typography variant="h4" gutterBottom sx={{ display: 'flex', alignItems: 'center', mb: 3 }}>
        <TrendingUp sx={{ mr: 1 }} />
        CRM & Vendas
      </Typography>

      {/* Métricas Principais */}
      <Grid container spacing={3} sx={{ mb: 3 }}>
        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>
                Total de Leads
              </Typography>
              <Typography variant="h4">
                {dashboardData.totalLeads}
              </Typography>
              <Typography variant="body2" color="success.main">
                +{dashboardData.newLeadsToday} hoje
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>
                Leads Quentes
              </Typography>
              <Typography variant="h4" color="error.main">
                {dashboardData.hotLeads}
              </Typography>
              <LocalFireDepartment color="error" />
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>
                Oportunidades
              </Typography>
              <Typography variant="h4">
                {dashboardData.totalOpportunities}
              </Typography>
              <Typography variant="body2" color="textSecondary">
                R$ {dashboardData.totalPipelineValue.toLocaleString('pt-BR')}
              </Typography>
            </CardContent>
          </Card>
        </Grid>

        <Grid item xs={12} sm={6} md={3}>
          <Card>
            <CardContent>
              <Typography color="textSecondary" gutterBottom>
                Atividades Hoje
              </Typography>
              <Typography variant="h4">
                {dashboardData.activitiesToday}
              </Typography>
              <Typography variant="body2" color={dashboardData.overdueActivities > 0 ? 'error.main' : 'success.main'}>
                {dashboardData.overdueActivities} atrasadas
              </Typography>
            </CardContent>
          </Card>
        </Grid>
      </Grid>

      {/* Inbox e Pipeline */}
      <Grid container spacing={3}>
        {/* Inbox Prioritário */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Inbox Prioritário
            </Typography>
            <List>
              {dashboardData.inboxItems.slice(0, 10).map((item) => (
                <ListItem
                  key={item.id}
                  onClick={() => handleInboxItemClick(item)}
                  sx={{ borderRadius: 1, mb: 1, bgcolor: 'background.paper', cursor: 'pointer' }}
                >
                  <ListItemAvatar>
                    <Avatar sx={{ bgcolor: `${getPriorityColor(item.priority)}.main` }}>
                      {getPriorityIcon(item.priority)}
                    </Avatar>
                  </ListItemAvatar>
                  <ListItemText
                    primary={
                      <Box display="flex" alignItems="center" gap={1}>
                        <Typography variant="subtitle2">{item.title}</Typography>
                        {item.temperature && (
                          <Chip
                            label={item.temperature}
                            size="small"
                            color={getTemperatureColor(item.temperature) as any}
                          />
                        )}
                        {item.value && (
                          <Chip
                            label={`R$ ${item.value.toLocaleString('pt-BR')}`}
                            size="small"
                            variant="outlined"
                          />
                        )}
                      </Box>
                    }
                    secondary={
                      <Box>
                        <Typography variant="body2" color="textSecondary">
                          {item.description}
                        </Typography>
                        {item.ownerName && (
                          <Typography variant="caption" color="textSecondary">
                            Responsável: {item.ownerName}
                          </Typography>
                        )}
                      </Box>
                    }
                  />
                </ListItem>
              ))}
            </List>
          </Paper>
        </Grid>

        {/* Pipeline por Estágio */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 2 }}>
            <Typography variant="h6" gutterBottom>
              Pipeline por Estágio
            </Typography>
            <Box sx={{ mb: 2 }}>
              {Object.entries(dashboardData.opportunitiesByStage).map(([stage, count]) => (
                <Box key={stage} sx={{ mb: 1 }}>
                  <Box display="flex" justifyContent="space-between" alignItems="center">
                    <Typography variant="body2">{stage}</Typography>
                    <Typography variant="body2" fontWeight="bold">{count}</Typography>
                  </Box>
                  <Box
                    sx={{
                      height: 8,
                      bgcolor: 'grey.200',
                      borderRadius: 1,
                      overflow: 'hidden',
                    }}
                  >
                    <Box
                      sx={{
                        height: '100%',
                        width: `${(count / Math.max(...Object.values(dashboardData.opportunitiesByStage))) * 100}%`,
                        bgcolor: 'primary.main',
                        borderRadius: 1,
                      }}
                    />
                  </Box>
                </Box>
              ))}
            </Box>

            <Divider sx={{ my: 2 }} />

            <Typography variant="h6" gutterBottom>
              Leads por Status
            </Typography>
            {Object.entries(dashboardData.leadsByStatus).map(([status, count]) => (
              <Box key={status} display="flex" justifyContent="space-between" sx={{ mb: 1 }}>
                <Typography variant="body2">{status}</Typography>
                <Typography variant="body2" fontWeight="bold">{count}</Typography>
              </Box>
            ))}
          </Paper>
        </Grid>
      </Grid>

      {/* Ações Rápidas */}
      <Box sx={{ mt: 3, display: 'flex', gap: 2, flexWrap: 'wrap' }}>
        <Button
          variant="contained"
          startIcon={<PersonAdd />}
          onClick={() => navigate('/crm/leads')}
        >
          Novo Lead
        </Button>
        <Button
          variant="outlined"
          startIcon={<Assignment />}
          onClick={() => navigate('/crm/opportunities')}
        >
          Ver Oportunidades
        </Button>
        <Button
          variant="outlined"
          startIcon={<Schedule />}
          onClick={() => navigate('/crm/activities')}
        >
          Agendar Atividade
        </Button>
      </Box>
    </Box>
  );
};

export default SalesDashboard;