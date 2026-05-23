import React, { useState, useEffect } from 'react';
import {
  Box,
  Typography,
  Paper,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Button,
  Chip,
  IconButton,
  TextField,
  InputAdornment,
  MenuItem,
  Select,
  FormControl,
  InputLabel,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Grid,
  Avatar,
  Tooltip,
  Fab,
} from '@mui/material';
import {
  Add as AddIcon,
  Search as SearchIcon,
  Edit as EditIcon,
  Delete as DeleteIcon,
  Phone as PhoneIcon,
  Email as EmailIcon,
  Business as BusinessIcon,
  Person as PersonIcon,
  FilterList as FilterIcon,
} from '@mui/icons-material';
import { LeadDto, LeadStatus, LeadSource, LeadTemperature } from '../models/Lead';
import { salesService } from '../services/salesService';
import LeadForm from '../components/LeadForm';

const LeadsList: React.FC = () => {
  const [leads, setLeads] = useState<LeadDto[]>([]);
  const [filteredLeads, setFilteredLeads] = useState<LeadDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState<LeadStatus | 'ALL'>('ALL');
  const [temperatureFilter, setTemperatureFilter] = useState<LeadTemperature | 'ALL'>('ALL');
  const [sourceFilter, setSourceFilter] = useState<LeadSource | 'ALL'>('ALL');
  const [openCreateDialog, setOpenCreateDialog] = useState(false);
  const [editingLead, setEditingLead] = useState<LeadDto | null>(null);

  useEffect(() => {
    loadLeads();
  }, []);

  useEffect(() => {
    filterLeads();
  }, [leads, searchTerm, statusFilter, temperatureFilter, sourceFilter]);

  const loadLeads = async () => {
    try {
      setLoading(true);
      const data = await salesService.getLeads();
      setLeads(data);
    } catch (error) {
      console.error('Erro ao carregar leads:', error);
    } finally {
      setLoading(false);
    }
  };

  const filterLeads = () => {
    let filtered = leads;

    if (searchTerm) {
      filtered = filtered.filter(lead =>
        lead.firstName?.toLowerCase().includes(searchTerm.toLowerCase()) ||
        lead.lastName?.toLowerCase().includes(searchTerm.toLowerCase()) ||
        lead.company?.toLowerCase().includes(searchTerm.toLowerCase()) ||
        lead.email?.toLowerCase().includes(searchTerm.toLowerCase()) ||
        lead.phone?.includes(searchTerm)
      );
    }

    if (statusFilter !== 'ALL') {
      filtered = filtered.filter(lead => lead.status === statusFilter);
    }

    if (temperatureFilter !== 'ALL') {
      filtered = filtered.filter(lead => lead.temperature === temperatureFilter);
    }

    if (sourceFilter !== 'ALL') {
      filtered = filtered.filter(lead => lead.source === sourceFilter);
    }

    setFilteredLeads(filtered);
  };

  const getStatusColor = (status: LeadStatus) => {
    switch (status) {
      case LeadStatus.New: return 'primary';
      case LeadStatus.Contacted: return 'info';
      case LeadStatus.Qualified: return 'success';
      case LeadStatus.Proposal: return 'warning';
      case LeadStatus.Negotiation: return 'secondary';
      case LeadStatus.ClosedWon: return 'success';
      case LeadStatus.ClosedLost: return 'error';
      default: return 'default';
    }
  };

  const getTemperatureColor = (temperature: LeadTemperature) => {
    switch (temperature) {
      case LeadTemperature.Hot: return '#ff4444';
      case LeadTemperature.Warm: return '#ffaa00';
      case LeadTemperature.Cold: return '#4444ff';
      default: return '#666666';
    }
  };

  const getStatusLabel = (status: LeadStatus) => {
    switch (status) {
      case LeadStatus.New: return 'Novo';
      case LeadStatus.Contacted: return 'Contactado';
      case LeadStatus.Qualified: return 'Qualificado';
      case LeadStatus.Proposal: return 'Proposta';
      case LeadStatus.Negotiation: return 'Negociação';
      case LeadStatus.ClosedWon: return 'Ganho';
      case LeadStatus.ClosedLost: return 'Perdido';
      default: return status;
    }
  };

  const getSourceLabel = (source: LeadSource) => {
    switch (source) {
      case LeadSource.Website: return 'Website';
      case LeadSource.Referral: return 'Indicação';
      case LeadSource.SocialMedia: return 'Redes Sociais';
      case LeadSource.Email: return 'Email';
      case LeadSource.Phone: return 'Telefone';
      case LeadSource.TradeShow: return 'Feira';
      case LeadSource.Direct: return 'Direto';
      case LeadSource.Other: return 'Outro';
      default: return source;
    }
  };

  const handleDeleteLead = async (id: string) => {
    if (window.confirm('Tem certeza que deseja excluir este lead?')) {
      try {
        await salesService.deleteLead(id);
        await loadLeads();
      } catch (error) {
        console.error('Erro ao excluir lead:', error);
      }
    }
  };

  const handleCreateLead = () => {
    setEditingLead(null);
    setOpenCreateDialog(true);
  };

  const handleEditLead = (lead: LeadDto) => {
    setEditingLead(lead);
    setOpenCreateDialog(true);
  };

  const handleSaveLead = async () => {
    await loadLeads();
  };

  const handleConvertToOpportunity = async (leadId: string) => {
    try {
      // Criar oportunidade básica com dados do lead
      const opportunityData = {
        title: `Oportunidade - ${leads.find(l => l.id === leadId)?.company || 'Lead'}`,
        value: leads.find(l => l.id === leadId)?.estimatedValue || 0,
        stage: 'Prospecting',
        probability: 25,
        expectedCloseDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString(), // 30 dias
        description: `Convertido do lead ${leadId}`,
      };

      await salesService.convertLeadToOpportunity(leadId, opportunityData);
      await loadLeads();
    } catch (error) {
      console.error('Erro ao converter lead para oportunidade:', error);
    }
  };

  return (
    <Box sx={{ p: 3 }}>
      <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', mb: 3 }}>
        <Typography variant="h4" component="h1" sx={{ fontWeight: 'bold' }}>
          Leads
        </Typography>
        <Button
          variant="contained"
          startIcon={<AddIcon />}
          onClick={handleCreateLead}
          sx={{ borderRadius: 2 }}
        >
          Novo Lead
        </Button>
      </Box>

      {/* Filtros */}
      <Paper sx={{ p: 2, mb: 3 }}>
        <Grid container spacing={2} alignItems="center">
          <Grid item xs={12} md={4}>
            <TextField
              fullWidth
              placeholder="Buscar leads..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
              InputProps={{
                startAdornment: (
                  <InputAdornment position="start">
                    <SearchIcon />
                  </InputAdornment>
                ),
              }}
            />
          </Grid>
          <Grid item xs={12} md={2}>
            <FormControl fullWidth>
              <InputLabel>Status</InputLabel>
              <Select
                value={statusFilter}
                label="Status"
                onChange={(e) => setStatusFilter(e.target.value as LeadStatus | 'ALL')}
              >
                <MenuItem value="ALL">Todos</MenuItem>
                {(Object.values(LeadStatus) as LeadStatus[]).map((status) => (
                  <MenuItem key={status} value={status}>
                    {getStatusLabel(status)}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Grid>
          <Grid item xs={12} md={2}>
            <FormControl fullWidth>
              <InputLabel>Temperatura</InputLabel>
              <Select
                value={temperatureFilter}
                label="Temperatura"
                onChange={(e) => setTemperatureFilter(e.target.value as LeadTemperature | 'ALL')}
              >
                <MenuItem value="ALL">Todas</MenuItem>
                {(Object.values(LeadTemperature) as LeadTemperature[]).map((temp) => (
                  <MenuItem key={temp} value={temp}>
                    {temp}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Grid>
          <Grid item xs={12} md={2}>
            <FormControl fullWidth>
              <InputLabel>Origem</InputLabel>
              <Select
                value={sourceFilter}
                label="Origem"
                onChange={(e) => setSourceFilter(e.target.value as LeadSource | 'ALL')}
              >
                <MenuItem value="ALL">Todas</MenuItem>
                {(Object.values(LeadSource) as LeadSource[]).map((source) => (
                  <MenuItem key={source} value={source}>
                    {getSourceLabel(source)}
                  </MenuItem>
                ))}
              </Select>
            </FormControl>
          </Grid>
          <Grid item xs={12} md={2}>
            <Button
              fullWidth
              variant="outlined"
              startIcon={<FilterIcon />}
              onClick={() => {
                setSearchTerm('');
                setStatusFilter('ALL');
                setTemperatureFilter('ALL');
                setSourceFilter('ALL');
              }}
            >
              Limpar
            </Button>
          </Grid>
        </Grid>
      </Paper>

      {/* Tabela */}
      <TableContainer component={Paper}>
        <Table>
          <TableHead>
            <TableRow>
              <TableCell>Lead</TableCell>
              <TableCell>Empresa</TableCell>
              <TableCell>Contato</TableCell>
              <TableCell>Status</TableCell>
              <TableCell>Temperatura</TableCell>
              <TableCell>Origem</TableCell>
              <TableCell>Valor</TableCell>
              <TableCell>Data</TableCell>
              <TableCell align="right">Ações</TableCell>
            </TableRow>
          </TableHead>
          <TableBody>
            {loading ? (
              <TableRow>
                <TableCell colSpan={9} align="center">
                  Carregando...
                </TableCell>
              </TableRow>
            ) : filteredLeads.length === 0 ? (
              <TableRow>
                <TableCell colSpan={9} align="center">
                  Nenhum lead encontrado
                </TableCell>
              </TableRow>
            ) : (
              filteredLeads.map((lead) => (
                <TableRow key={lead.id} hover>
                  <TableCell>
                    <Box sx={{ display: 'flex', alignItems: 'center' }}>
                      <Avatar sx={{ mr: 2, bgcolor: getTemperatureColor(lead.temperature) }}>
                        <PersonIcon />
                      </Avatar>
                      <Box>
                        <Typography variant="body2" sx={{ fontWeight: 'bold' }}>
                          {lead.firstName} {lead.lastName}
                        </Typography>
                        <Typography variant="caption" color="text.secondary">
                          {lead.title}
                        </Typography>
                      </Box>
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Box sx={{ display: 'flex', alignItems: 'center' }}>
                      <BusinessIcon sx={{ mr: 1, color: 'text.secondary' }} />
                      {lead.company}
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Box>
                      {lead.email && (
                        <Box sx={{ display: 'flex', alignItems: 'center', mb: 0.5 }}>
                          <EmailIcon sx={{ mr: 1, fontSize: 16, color: 'text.secondary' }} />
                          <Typography variant="body2">{lead.email}</Typography>
                        </Box>
                      )}
                      {lead.phone && (
                        <Box sx={{ display: 'flex', alignItems: 'center' }}>
                          <PhoneIcon sx={{ mr: 1, fontSize: 16, color: 'text.secondary' }} />
                          <Typography variant="body2">{lead.phone}</Typography>
                        </Box>
                      )}
                    </Box>
                  </TableCell>
                  <TableCell>
                    <Chip
                      label={getStatusLabel(lead.status)}
                      color={getStatusColor(lead.status)}
                      size="small"
                    />
                  </TableCell>
                  <TableCell>
                    <Box
                      sx={{
                        width: 12,
                        height: 12,
                        borderRadius: '50%',
                        bgcolor: getTemperatureColor(lead.temperature),
                        display: 'inline-block',
                        mr: 1
                      }}
                    />
                    {lead.temperature}
                  </TableCell>
                  <TableCell>{getSourceLabel(lead.source)}</TableCell>
                  <TableCell>
                    {lead.estimatedValue ? `R$ ${lead.estimatedValue.toLocaleString()}` : '-'}
                  </TableCell>
                  <TableCell>
                    {new Date(lead.createdAt).toLocaleDateString('pt-BR')}
                  </TableCell>
                  <TableCell align="right">
                    <Tooltip title="Editar">
                      <IconButton
                        size="small"
                        onClick={() => handleEditLead(lead)}
                      >
                        <EditIcon />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="Converter para Oportunidade">
                      <IconButton
                        size="small"
                        onClick={() => handleConvertToOpportunity(lead.id)}
                        disabled={lead.status === LeadStatus.ClosedWon || lead.status === LeadStatus.ClosedLost}
                      >
                        <BusinessIcon />
                      </IconButton>
                    </Tooltip>
                    <Tooltip title="Excluir">
                      <IconButton
                        size="small"
                        color="error"
                        onClick={() => handleDeleteLead(lead.id)}
                      >
                        <DeleteIcon />
                      </IconButton>
                    </Tooltip>
                  </TableCell>
                </TableRow>
              ))
            )}
          </TableBody>
        </Table>
      </TableContainer>

      {/* FAB para adicionar novo lead */}
      <Fab
        color="primary"
        aria-label="add"
        sx={{ position: 'fixed', bottom: 16, right: 16 }}
        onClick={handleCreateLead}
      >
        <AddIcon />
      </Fab>

      {/* Dialog para criar/editar lead */}
      <LeadForm
        open={openCreateDialog}
        onClose={() => setOpenCreateDialog(false)}
        onSave={handleSaveLead}
        editingLead={editingLead}
      />
    </Box>
  );
};

export default LeadsList;