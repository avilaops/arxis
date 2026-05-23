import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:5136/api';

export const exportService = {
  async downloadLeads(startDate?: string, endDate?: string): Promise<void> {
    const params = new URLSearchParams();
    if (startDate) params.append('startDate', startDate);
    if (endDate) params.append('endDate', endDate);

    try {
      const response = await axios.get(
        `${API_BASE_URL}/export/leads?${params}`,
        { responseType: 'blob' }
      );

      const blob = new Blob([response.data], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
      });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `Leads_${new Date().toISOString().split('T')[0]}.xlsx`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Erro ao baixar leads:', error);
      throw error;
    }
  },

  async downloadOpportunities(startDate?: string, endDate?: string): Promise<void> {
    const params = new URLSearchParams();
    if (startDate) params.append('startDate', startDate);
    if (endDate) params.append('endDate', endDate);

    try {
      const response = await axios.get(
        `${API_BASE_URL}/export/opportunities?${params}`,
        { responseType: 'blob' }
      );

      const blob = new Blob([response.data], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
      });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `Oportunidades_${new Date().toISOString().split('T')[0]}.xlsx`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Erro ao baixar oportunidades:', error);
      throw error;
    }
  },

  async downloadDashboard(startDate?: string, endDate?: string): Promise<void> {
    const params = new URLSearchParams();
    if (startDate) params.append('startDate', startDate);
    if (endDate) params.append('endDate', endDate);

    try {
      const response = await axios.get(
        `${API_BASE_URL}/export/dashboard?${params}`,
        { responseType: 'blob' }
      );

      const blob = new Blob([response.data], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
      });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `Dashboard_CRM_${new Date().toISOString().split('T')[0]}.xlsx`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Erro ao baixar dashboard:', error);
      throw error;
    }
  },

  async downloadPerformanceReport(startDate?: string, endDate?: string): Promise<void> {
    const params = new URLSearchParams();
    if (startDate) params.append('startDate', startDate);
    if (endDate) params.append('endDate', endDate);

    try {
      const response = await axios.get(
        `${API_BASE_URL}/export/performance-report?${params}`,
        { responseType: 'blob' }
      );

      const blob = new Blob([response.data], {
        type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet'
      });
      const url = window.URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = url;
      link.download = `Performance_${new Date().toISOString().split('T')[0]}.xlsx`;
      document.body.appendChild(link);
      link.click();
      document.body.removeChild(link);
      window.URL.revokeObjectURL(url);
    } catch (error) {
      console.error('Erro ao baixar relatório de performance:', error);
      throw error;
    }
  },

  async requestCustomReport(startDate: string, endDate: string, email?: string): Promise<void> {
    try {
      const params = new URLSearchParams();
      params.append('startDate', startDate);
      params.append('endDate', endDate);
      if (email) params.append('email', email);

      await axios.post(`${API_BASE_URL}/reports/custom?${params}`);
      return;
    } catch (error) {
      console.error('Erro ao solicitar relatório personalizado:', error);
      throw error;
    }
  },

  async triggerDailyReport(): Promise<void> {
    try {
      await axios.post(`${API_BASE_URL}/reports/daily/send`);
      return;
    } catch (error) {
      console.error('Erro ao disparar relatório diário:', error);
      throw error;
    }
  },

  async triggerWeeklyReport(): Promise<void> {
    try {
      await axios.post(`${API_BASE_URL}/reports/weekly/send`);
      return;
    } catch (error) {
      console.error('Erro ao disparar relatório semanal:', error);
      throw error;
    }
  },

  async triggerMonthlyReport(): Promise<void> {
    try {
      await axios.post(`${API_BASE_URL}/reports/monthly/send`);
      return;
    } catch (error) {
      console.error('Erro ao disparar relatório mensal:', error);
      throw error;
    }
  }
};
