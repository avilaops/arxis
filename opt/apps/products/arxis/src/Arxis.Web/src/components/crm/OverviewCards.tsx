import React from 'react';
import { Grid, Paper, Typography, Box } from '@mui/material';
import {
  TrendingUp,
  People,
  BusinessCenter,
  AttachMoney,
  ShowChart
} from '@mui/icons-material';
import { CrmAnalyticsOverview } from '../../services/analyticsService';

interface Props {
  overview: CrmAnalyticsOverview;
}

interface CardData {
  title: string;
  value: string | number;
  icon: React.ReactElement;
  color: string;
  bgColor: string;
}

export const OverviewCards: React.FC<Props> = ({ overview }) => {
  const formatCurrency = (value: number): string => {
    return new Intl.NumberFormat('pt-BR', {
      style: 'currency',
      currency: 'BRL'
    }).format(value);
  };

  const formatPercentage = (value: number): string => {
    return `${value.toFixed(1)}%`;
  };

  const cards: CardData[] = [
    {
      title: 'Total de Leads',
      value: overview.totalLeads,
      icon: <People sx={{ fontSize: 40 }} />,
      color: '#1976d2',
      bgColor: '#e3f2fd'
    },
    {
      title: 'Oportunidades',
      value: overview.totalOpportunities,
      icon: <BusinessCenter sx={{ fontSize: 40 }} />,
      color: '#9c27b0',
      bgColor: '#f3e5f5'
    },
    {
      title: 'Taxa de Conversão',
      value: formatPercentage(overview.conversionRate),
      icon: <TrendingUp sx={{ fontSize: 40 }} />,
      color: '#2e7d32',
      bgColor: '#e8f5e9'
    },
    {
      title: 'Receita Total',
      value: formatCurrency(overview.totalRevenue),
      icon: <AttachMoney sx={{ fontSize: 40 }} />,
      color: '#ed6c02',
      bgColor: '#fff3e0'
    },
    {
      title: 'Ticket Médio',
      value: formatCurrency(overview.avgTicket),
      icon: <ShowChart sx={{ fontSize: 40 }} />,
      color: '#0288d1',
      bgColor: '#e1f5fe'
    }
  ];

  return (
    <Grid container spacing={3}>
      {cards.map((card, index) => (
        <Grid item xs={12} sm={6} md={4} lg={2.4} key={index}>
          <Paper
            elevation={2}
            sx={{
              p: 2.5,
              display: 'flex',
              flexDirection: 'column',
              height: '140px',
              position: 'relative',
              overflow: 'hidden',
              '&:hover': {
                boxShadow: 6,
                transform: 'translateY(-4px)',
                transition: 'all 0.3s ease'
              }
            }}
          >
            {/* Background Icon */}
            <Box
              sx={{
                position: 'absolute',
                right: -10,
                bottom: -10,
                opacity: 0.1,
                color: card.color
              }}
            >
              {React.cloneElement(card.icon, { sx: { fontSize: 100 } })}
            </Box>

            {/* Icon Circle */}
            <Box
              sx={{
                width: 56,
                height: 56,
                borderRadius: '50%',
                backgroundColor: card.bgColor,
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                mb: 1.5,
                color: card.color
              }}
            >
              {card.icon}
            </Box>

            {/* Title */}
            <Typography
              variant="body2"
              color="text.secondary"
              sx={{ mb: 0.5, fontWeight: 500 }}
            >
              {card.title}
            </Typography>

            {/* Value */}
            <Typography
              variant="h5"
              component="div"
              sx={{
                fontWeight: 700,
                color: card.color,
                position: 'relative',
                zIndex: 1
              }}
            >
              {card.value}
            </Typography>
          </Paper>
        </Grid>
      ))}
    </Grid>
  );
};
