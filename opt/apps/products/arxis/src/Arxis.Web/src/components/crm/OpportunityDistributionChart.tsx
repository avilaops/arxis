import React, { useEffect, useState } from 'react';
import { Typography, Box, CircularProgress } from '@mui/material';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Cell, Legend } from 'recharts';
import { crmAnalyticsService, CrmOpportunityDistribution } from '../../services/analyticsService';

interface Props {
  dateRange: { start?: string; end?: string };
}

const COLORS = ['#8064A2', '#4BACC6', '#4F81BD', '#C0504D', '#9BBB59'];

export const OpportunityDistributionChart: React.FC<Props> = ({ dateRange }) => {
  const [data, setData] = useState<CrmOpportunityDistribution[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [dateRange]);

  const loadData = async () => {
    try {
      setLoading(true);
      const distributionData = await crmAnalyticsService.getOpportunityDistribution(
        dateRange.start,
        dateRange.end
      );
      setData(distributionData);
    } catch (error) {
      console.error('Erro ao carregar distribuição de oportunidades:', error);
    } finally {
      setLoading(false);
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
      <Typography variant="h6" gutterBottom>
        Distribuição de Oportunidades
      </Typography>
      <ResponsiveContainer width="100%" height="90%">
        <BarChart data={data} margin={{ top: 5, right: 30, left: 20, bottom: 5 }}>
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis dataKey="stage" />
          <YAxis yAxisId="left" orientation="left" />
          <YAxis yAxisId="right" orientation="right" tickFormatter={formatCurrency} />
          <Tooltip
            formatter={(value: number, name: string) => {
              if (name === 'Valor Total') {
                return [formatCurrency(value), name];
              }
              return [value, name];
            }}
          />
          <Legend />
          <Bar yAxisId="left" dataKey="count" name="Quantidade" radius={[8, 8, 0, 0]}>
            {data.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
            ))}
          </Bar>
          <Bar yAxisId="right" dataKey="totalValue" name="Valor Total" fill="#ed6c02" opacity={0.7} />
        </BarChart>
      </ResponsiveContainer>
    </Box>
  );
};
