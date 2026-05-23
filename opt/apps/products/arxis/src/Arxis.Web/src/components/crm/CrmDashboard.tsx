import React, { useEffect, useState } from 'react';
import {
  Box,
  Container,
  Grid,
  Paper,
  Typography,
  CircularProgress,
  Alert,
  Button,
  ButtonGroup
} from '@mui/material';
import {
  TrendingUp,
  People,
  AttachMoney,
  Assessment,
  Download
} from '@mui/icons-material';
import { crmAnalyticsService, CrmAnalyticsOverview } from '../../services/analyticsService';
import { exportService } from '../../services/exportService';
import { OverviewCards } from './OverviewCards';
import { FunnelChart } from './FunnelChart';
import { TrendsChart } from './TrendsChart';
import { PerformanceTable } from './PerformanceTable';
import { LeadSourcesChart } from './LeadSourcesChart';
import { OpportunityDistributionChart } from './OpportunityDistributionChart';

export const CrmDashboard: React.FC = () => {
  const [overview, setOverview] = useState<CrmAnalyticsOverview | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState<{ start?: string; end?: string }>({});
  const [exporting, setExporting] = useState(false);

  useEffect(() => {
    loadData();
  }, [dateRange]);

  const loadData = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await crmAnalyticsService.getOverview(dateRange.start, dateRange.end);
      setOverview(data);
    } catch (err: any) {
      console.error('Erro ao carregar dashboard:', err);
      setError(err.response?.data?.message || 'Erro ao carregar dados do dashboard');
    } finally {
      setLoading(false);
    }
  };

  const handleExportDashboard = async () => {
    try {
      setExporting(true);
      await exportService.downloadDashboard(dateRange.start, dateRange.end);
    } catch (err) {
      console.error('Erro ao exportar dashboard:', err);
      setError('Erro ao exportar dashboard');
    } finally {
      setExporting(false);
    }
  };

  const handleExportLeads = async () => {
    try {
      setExporting(true);
      await exportService.downloadLeads(dateRange.start, dateRange.end);
    } catch (err) {
      console.error('Erro ao exportar leads:', err);
      setError('Erro ao exportar leads');
    } finally {
      setExporting(false);
    }
  };

  const handleExportOpportunities = async () => {
    try {
      setExporting(true);
      await exportService.downloadOpportunities(dateRange.start, dateRange.end);
    } catch (err) {
      console.error('Erro ao exportar oportunidades:', err);
      setError('Erro ao exportar oportunidades');
    } finally {
      setExporting(false);
    }
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <CircularProgress />
      </Box>
    );
  }

  if (error) {
    return (
      <Container maxWidth="lg" sx={{ mt: 4 }}>
        <Alert severity="error" onClose={() => setError(null)}>
          {error}
        </Alert>
      </Container>
    );
  }

  if (!overview) {
    return (
      <Container maxWidth="lg" sx={{ mt: 4 }}>
        <Alert severity="warning">Nenhum dado disponível</Alert>
      </Container>
    );
  }

  return (
    <Container maxWidth="xl" sx={{ mt: 4, mb: 4 }}>
      {/* Header */}
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1" gutterBottom>
          📊 Dashboard CRM
        </Typography>
        
        <ButtonGroup variant="outlined" disabled={exporting}>
          <Button
            startIcon={<Download />}
            onClick={handleExportDashboard}
          >
            Dashboard
          </Button>
          <Button
            startIcon={<People />}
            onClick={handleExportLeads}
          >
            Leads
          </Button>
          <Button
            startIcon={<AttachMoney />}
            onClick={handleExportOpportunities}
          >
            Oportunidades
          </Button>
        </ButtonGroup>
      </Box>

      {/* Overview Cards */}
      <OverviewCards overview={overview} />

      {/* Charts Grid */}
      <Grid container spacing={3} sx={{ mt: 2 }}>
        {/* Funil de Vendas */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3, height: '400px' }}>
            <FunnelChart dateRange={dateRange} />
          </Paper>
        </Grid>

        {/* Distribuição de Oportunidades */}
        <Grid item xs={12} md={6}>
          <Paper sx={{ p: 3, height: '400px' }}>
            <OpportunityDistributionChart dateRange={dateRange} />
          </Paper>
        </Grid>

        {/* Tendências */}
        <Grid item xs={12} md={8}>
          <Paper sx={{ p: 3, height: '400px' }}>
            <TrendsChart groupBy="week" />
          </Paper>
        </Grid>

        {/* Fontes de Leads */}
        <Grid item xs={12} md={4}>
          <Paper sx={{ p: 3, height: '400px' }}>
            <LeadSourcesChart dateRange={dateRange} />
          </Paper>
        </Grid>

        {/* Tabela de Performance por Usuário */}
        <Grid item xs={12}>
          <Paper sx={{ p: 3 }}>
            <PerformanceTable dateRange={dateRange} />
          </Paper>
        </Grid>
      </Grid>
    </Container>
  );
};
