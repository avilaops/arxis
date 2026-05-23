import React, { useEffect, useState } from 'react';
import { Typography, Box, CircularProgress } from '@mui/material';
import { PieChart, Pie, Cell, ResponsiveContainer, Tooltip, Legend } from 'recharts';
import { crmAnalyticsService, CrmLeadSource } from '../../services/analyticsService';

interface Props {
  dateRange: { start?: string; end?: string };
}

const COLORS = ['#1976d2', '#9c27b0', '#2e7d32', '#ed6c02', '#0288d1', '#d32f2f'];

export const LeadSourcesChart: React.FC<Props> = ({ dateRange }) => {
  const [data, setData] = useState<CrmLeadSource[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [dateRange]);

  const loadData = async () => {
    try {
      setLoading(true);
      const sourcesData = await crmAnalyticsService.getLeadSources(dateRange.start, dateRange.end);
      setData(sourcesData);
    } catch (error) {
      console.error('Erro ao carregar fontes de leads:', error);
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
        Fontes de Leads
      </Typography>
      <ResponsiveContainer width="100%" height="90%">
        <PieChart>
          <Pie
            data={data}
            dataKey="count"
            nameKey="source"
            cx="50%"
            cy="50%"
            outerRadius={80}
            label={({ percentage }) => `${percentage.toFixed(1)}%`}
          >
            {data.map((entry, index) => (
              <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
            ))}
          </Pie>
          <Tooltip formatter={(value: number) => [value, 'Quantidade']} />
          <Legend verticalAlign="bottom" height={36} />
        </PieChart>
      </ResponsiveContainer>
    </Box>
  );
};
