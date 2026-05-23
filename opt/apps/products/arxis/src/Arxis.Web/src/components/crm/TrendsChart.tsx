import React, { useEffect, useState } from 'react';
import { Typography, Box, CircularProgress, ToggleButtonGroup, ToggleButton } from '@mui/material';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, Legend, ResponsiveContainer } from 'recharts';
import { crmAnalyticsService, CrmTrendsData } from '../../services/analyticsService';

interface Props {
  groupBy?: 'week' | 'month';
}

export const TrendsChart: React.FC<Props> = ({ groupBy: initialGroupBy = 'week' }) => {
  const [data, setData] = useState<CrmTrendsData[]>([]);
  const [loading, setLoading] = useState(true);
  const [groupBy, setGroupBy] = useState<'week' | 'month'>(initialGroupBy);

  useEffect(() => {
    loadData();
  }, [groupBy]);

  const loadData = async () => {
    try {
      setLoading(true);
      const trendsData = await crmAnalyticsService.getTrends(groupBy);
      setData(trendsData);
    } catch (error) {
      console.error('Erro ao carregar tendências:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleGroupByChange = (_: React.MouseEvent<HTMLElement>, newGroupBy: 'week' | 'month' | null) => {
    if (newGroupBy !== null) {
      setGroupBy(newGroupBy);
    }
  };

  const formatCurrency = (value: number): string => {
    return new Intl.NumberFormat('pt-BR', {
      style: 'currency',
      currency: 'BRL',
      minimumFractionDigits: 0,
      maximumFractionDigits: 0
    }).format(value);
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" height="100%">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box height="100%">
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={2}>
        <Typography variant="h6">Tendências</Typography>
        <ToggleButtonGroup
          value={groupBy}
          exclusive
          onChange={handleGroupByChange}
          size="small"
        >
          <ToggleButton value="week">Semanal</ToggleButton>
          <ToggleButton value="month">Mensal</ToggleButton>
        </ToggleButtonGroup>
      </Box>

      <ResponsiveContainer width="100%" height="85%">
        <LineChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="period" />
          <YAxis yAxisId="left" />
          <YAxis yAxisId="right" orientation="right" tickFormatter={formatCurrency} />
          <Tooltip
            formatter={(value: number, name: string) => {
              if (name === 'Receita') {
                return [formatCurrency(value), name];
              }
              return [value, name];
            }}
          />
          <Legend />
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="newLeads"
            name="Novos Leads"
            stroke="#1976d2"
            strokeWidth={2}
            dot={{ r: 4 }}
            activeDot={{ r: 6 }}
          />
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="newOpportunities"
            name="Novas Oportunidades"
            stroke="#9c27b0"
            strokeWidth={2}
            dot={{ r: 4 }}
            activeDot={{ r: 6 }}
          />
          <Line
            yAxisId="left"
            type="monotone"
            dataKey="wonOpportunities"
            name="Oportunidades Ganhas"
            stroke="#2e7d32"
            strokeWidth={2}
            dot={{ r: 4 }}
            activeDot={{ r: 6 }}
          />
          <Line
            yAxisId="right"
            type="monotone"
            dataKey="revenue"
            name="Receita"
            stroke="#ed6c02"
            strokeWidth={2}
            dot={{ r: 4 }}
            activeDot={{ r: 6 }}
          />
        </LineChart>
      </ResponsiveContainer>
    </Box>
  );
};
