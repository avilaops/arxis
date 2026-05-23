import React, { useEffect, useState } from 'react';
import { Typography, Box, CircularProgress } from '@mui/material';
import { BarChart, Bar, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Cell } from 'recharts';
import { crmAnalyticsService, CrmFunnelData } from '../../services/analyticsService';

interface Props {
  dateRange: { start?: string; end?: string };
}

const COLORS = ['#4F81BD', '#C0504D', '#9BBB59', '#8064A2', '#4BACC6'];

export const FunnelChart: React.FC<Props> = ({ dateRange }) => {
  const [data, setData] = useState<CrmFunnelData[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [dateRange]);

  const loadData = async () => {
    try {
      setLoading(true);
      const funnelData = await crmAnalyticsService.getFunnel(dateRange.start, dateRange.end);
      setData(funnelData);
    } catch (error) {
      console.error('Erro ao carregar funil:', error);
    } finally {
      setLoading(false);
    }
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
        Funil de Vendas
      </Typography>
      <ResponsiveContainer width="100%" height="90%">
        <BarChart
          data={data}
          layout="vertical"
          margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
        >
          <CartesianGrid strokeDasharray="3 3" />
          <XAxis type="number" />
          <YAxis dataKey="stage" type="category" width={150} />
          <Tooltip
            formatter={(value: number, name: string, props: any) => [
              `${value} (${props.payload.percentage.toFixed(1)}%)`,
              'Quantidade'
            ]}
          />
          <Bar dataKey="count" radius={[0, 8, 8, 0]}>
            {data.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
            ))}
          </Bar>
        </BarChart>
      </ResponsiveContainer>
    </Box>
  );
};
