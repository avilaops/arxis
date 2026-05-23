import React, { useEffect, useState } from 'react';
import {
  Typography,
  Box,
  CircularProgress,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow
} from '@mui/material';
import { crmAnalyticsService, CrmPerformanceByUser } from '../../services/analyticsService';

interface Props {
  dateRange: { start?: string; end?: string };
}

export const PerformanceTable: React.FC<Props> = ({ dateRange }) => {
  const [data, setData] = useState<CrmPerformanceByUser[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadData();
  }, [dateRange]);

  const loadData = async () => {
    try {
      setLoading(true);
      const performanceData = await crmAnalyticsService.getPerformanceByUser(
        dateRange.start,
        dateRange.end
      );
      setData(performanceData);
    } catch (error) {
      console.error('Erro ao carregar performance:', error);
    } finally {
      setLoading(false);
    }
  };

  const formatCurrency = (value: number): string => {
    return new Intl.NumberFormat('pt-BR', {
      style: 'currency',
      currency: 'BRL'
    }).format(value);
  };

  const formatPercentage = (value: number): string => {
    return `${value.toFixed(1)}%`;
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="200px">
        <CircularProgress />
      </Box>
    );
  }

  return (
    <Box>
      <Typography variant="h6" gutterBottom>
        Performance por Usuário
      </Typography>
      <TableContainer>
        <Table size="small">
          <TableHead>
            <TableRow sx={{ backgroundColor: '#f5f5f5' }}>
              <TableCell><strong>Usuário</strong></TableCell>
              <TableCell align="right"><strong>Leads</strong></TableCell>
              <TableCell align="right"><strong>Oportunidades</strong></TableCell>
              <TableCell align="right"><strong>Ganhas</strong></TableCell>
              <TableCell align="right"><strong>Taxa Conversão</strong></TableCell>
              <TableCell align="right"><strong>Receita Total</strong></TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {data.length === 0 ? (
              <TableRow>
                <TableCell colSpan={6} align="center">
                  Nenhum dado disponível
                </TableCell>
              </TableRow>
            ) : (
              data.map((row, index) => (
                <TableRow
                  key={index}
                  sx={{
                    '&:hover': { backgroundColor: '#f9f9f9' },
                    '&:nth-of-type(odd)': { backgroundColor: '#fafafa' }
                  }}
                >
                  <TableCell component="th" scope="row">
                    {row.userName}
                  </TableCell>
                  <TableCell align="right">{row.leadsCount}</TableCell>
                  <TableCell align="right">{row.opportunitiesCount}</TableCell>
                  <TableCell align="right" sx={{ color: '#2e7d32', fontWeight: 600 }}>
                    {row.opportunitiesWon}
                  </TableCell>
                  <TableCell align="right" sx={{ color: row.conversionRate >= 50 ? '#2e7d32' : '#ed6c02' }}>
                    {formatPercentage(row.conversionRate)}
                  </TableCell>
                  <TableCell align="right" sx={{ fontWeight: 600 }}>
                    {formatCurrency(row.totalRevenue)}
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </TableContainer>
    </Box>
  );
};
