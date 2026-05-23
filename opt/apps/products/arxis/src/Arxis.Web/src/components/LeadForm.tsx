import React, { useState, useEffect } from 'react';
import {
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  Button,
  TextField,
  Grid,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Box,
  Typography,
  Chip,
  IconButton,
} from '@mui/material';
import {
  Close as CloseIcon,
  Add as AddIcon,
} from '@mui/icons-material';
import { LeadDto, CreateLeadRequest, UpdateLeadRequest, LeadStatus, LeadSource, LeadTemperature } from '../models/Lead';
import { salesService } from '../services/salesService';

interface LeadFormProps {
  open: boolean;
  onClose: () => void;
  onSave: () => void;
  editingLead?: LeadDto | null;
}

const LeadForm: React.FC<LeadFormProps> = ({ open, onClose, onSave, editingLead }) => {
  const [formData, setFormData] = useState<Partial<CreateLeadRequest>>({
    status: LeadStatus.New,
    source: LeadSource.Website,
    temperature: LeadTemperature.Warm,
  });
  const [loading, setLoading] = useState(false);
  const [newTag, setNewTag] = useState('');

  useEffect(() => {
    if (editingLead) {
      setFormData({
        firstName: editingLead.firstName || '',
        lastName: editingLead.lastName || '',
        title: editingLead.title || '',
        company: editingLead.company || '',
        email: editingLead.email || '',
        phone: editingLead.phone || '',
        website: editingLead.website || '',
        address: editingLead.address || '',
        city: editingLead.city || '',
        state: editingLead.state || '',
        country: editingLead.country || '',
        postalCode: editingLead.postalCode || '',
        status: editingLead.status,
        source: editingLead.source,
        temperature: editingLead.temperature,
        estimatedValue: editingLead.estimatedValue || 0,
        notes: editingLead.notes || '',
        assignedToUserId: editingLead.assignedToUserId || '',
        lastContactDate: editingLead.lastContactDate || '',
        nextFollowUpDate: editingLead.nextFollowUpDate || '',
        tags: editingLead.tags || [],
        customFields: editingLead.customFields || {},
      });
    } else {
      setFormData({
        status: LeadStatus.New,
        source: LeadSource.Website,
        temperature: LeadTemperature.Warm,
        tags: [],
        customFields: {},
      });
    }
  }, [editingLead, open]);

  const handleInputChange = (field: string, value: any) => {
    setFormData(prev => ({
      ...prev,
      [field]: value
    }));
  };

  const handleAddTag = () => {
    if (newTag.trim() && !formData.tags?.includes(newTag.trim())) {
      setFormData(prev => ({
        ...prev,
        tags: [...(prev.tags || []), newTag.trim()]
      }));
      setNewTag('');
    }
  };

  const handleRemoveTag = (tagToRemove: string) => {
    setFormData(prev => ({
      ...prev,
      tags: prev.tags?.filter(tag => tag !== tagToRemove) || []
    }));
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    setLoading(true);

    try {
      if (editingLead) {
        const updateData: UpdateLeadRequest = {
          id: editingLead.id,
          ...formData,
        };
        await salesService.updateLead(editingLead.id, updateData);
      } else {
        const createData: CreateLeadRequest = {
          ...formData,
          status: formData.status || LeadStatus.New,
          source: formData.source || LeadSource.Website,
          temperature: formData.temperature || LeadTemperature.Warm,
        } as CreateLeadRequest;
        await salesService.createLead(createData);
      }

      onSave();
      onClose();
    } catch (error) {
      console.error('Erro ao salvar lead:', error);
    } finally {
      setLoading(false);
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

  return (
    <Dialog open={open} onClose={onClose} maxWidth="md" fullWidth>
      <DialogTitle>
        <Box sx={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
          <Typography variant="h6">
            {editingLead ? 'Editar Lead' : 'Novo Lead'}
          </Typography>
          <IconButton onClick={onClose} size="small">
            <CloseIcon />
          </IconButton>
        </Box>
      </DialogTitle>

      <form onSubmit={handleSubmit}>
        <DialogContent>
          <Grid container spacing={3}>
            {/* Informações Pessoais */}
            <Grid item xs={12}>
              <Typography variant="h6" gutterBottom>
                Informações Pessoais
              </Typography>
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Nome"
                value={formData.firstName || ''}
                onChange={(e) => handleInputChange('firstName', e.target.value)}
                required
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Sobrenome"
                value={formData.lastName || ''}
                onChange={(e) => handleInputChange('lastName', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Cargo"
                value={formData.title || ''}
                onChange={(e) => handleInputChange('title', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Empresa"
                value={formData.company || ''}
                onChange={(e) => handleInputChange('company', e.target.value)}
              />
            </Grid>

            {/* Informações de Contato */}
            <Grid item xs={12}>
              <Typography variant="h6" gutterBottom>
                Informações de Contato
              </Typography>
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Email"
                type="email"
                value={formData.email || ''}
                onChange={(e) => handleInputChange('email', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Telefone"
                value={formData.phone || ''}
                onChange={(e) => handleInputChange('phone', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Website"
                value={formData.website || ''}
                onChange={(e) => handleInputChange('website', e.target.value)}
              />
            </Grid>

            {/* Endereço */}
            <Grid item xs={12}>
              <Typography variant="h6" gutterBottom>
                Endereço
              </Typography>
            </Grid>

            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Endereço"
                value={formData.address || ''}
                onChange={(e) => handleInputChange('address', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={4}>
              <TextField
                fullWidth
                label="Cidade"
                value={formData.city || ''}
                onChange={(e) => handleInputChange('city', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={4}>
              <TextField
                fullWidth
                label="Estado"
                value={formData.state || ''}
                onChange={(e) => handleInputChange('state', e.target.value)}
              />
            </Grid>

            <Grid item xs={12} md={4}>
              <TextField
                fullWidth
                label="CEP"
                value={formData.postalCode || ''}
                onChange={(e) => handleInputChange('postalCode', e.target.value)}
              />
            </Grid>

            {/* Informações de Vendas */}
            <Grid item xs={12}>
              <Typography variant="h6" gutterBottom>
                Informações de Vendas
              </Typography>
            </Grid>

            <Grid item xs={12} md={4}>
              <FormControl fullWidth>
                <InputLabel>Status</InputLabel>
                <Select
                  value={formData.status || LeadStatus.New}
                  label="Status"
                  onChange={(e) => handleInputChange('status', e.target.value)}
                >
                  {Object.values(LeadStatus).map((status) => (
                    <MenuItem key={status} value={status}>
                      {getStatusLabel(status)}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={4}>
              <FormControl fullWidth>
                <InputLabel>Origem</InputLabel>
                <Select
                  value={formData.source || LeadSource.Website}
                  label="Origem"
                  onChange={(e) => handleInputChange('source', e.target.value)}
                >
                  {Object.values(LeadSource).map((source) => (
                    <MenuItem key={source} value={source}>
                      {getSourceLabel(source)}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={4}>
              <FormControl fullWidth>
                <InputLabel>Temperatura</InputLabel>
                <Select
                  value={formData.temperature || LeadTemperature.Warm}
                  label="Temperatura"
                  onChange={(e) => handleInputChange('temperature', e.target.value)}
                >
                  {Object.values(LeadTemperature).map((temp) => (
                    <MenuItem key={temp} value={temp}>
                      {temp}
                    </MenuItem>
                  ))}
                </Select>
              </FormControl>
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Valor Estimado"
                type="number"
                value={formData.estimatedValue || ''}
                onChange={(e) => handleInputChange('estimatedValue', parseFloat(e.target.value) || 0)}
                InputProps={{
                  startAdornment: <Typography>R$</Typography>,
                }}
              />
            </Grid>

            {/* Datas */}
            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Último Contato"
                type="datetime-local"
                value={formData.lastContactDate ? new Date(formData.lastContactDate).toISOString().slice(0, 16) : ''}
                onChange={(e) => handleInputChange('lastContactDate', e.target.value)}
                InputLabelProps={{
                  shrink: true,
                }}
              />
            </Grid>

            <Grid item xs={12} md={6}>
              <TextField
                fullWidth
                label="Próximo Follow-up"
                type="datetime-local"
                value={formData.nextFollowUpDate ? new Date(formData.nextFollowUpDate).toISOString().slice(0, 16) : ''}
                onChange={(e) => handleInputChange('nextFollowUpDate', e.target.value)}
                InputLabelProps={{
                  shrink: true,
                }}
              />
            </Grid>

            {/* Tags */}
            <Grid item xs={12}>
              <Typography variant="h6" gutterBottom>
                Tags
              </Typography>
              <Box sx={{ display: 'flex', flexWrap: 'wrap', gap: 1, mb: 2 }}>
                {formData.tags?.map((tag) => (
                  <Chip
                    key={tag}
                    label={tag}
                    onDelete={() => handleRemoveTag(tag)}
                    size="small"
                  />
                ))}
              </Box>
              <Box sx={{ display: 'flex', gap: 1 }}>
                <TextField
                  fullWidth
                  placeholder="Adicionar tag..."
                  value={newTag}
                  onChange={(e) => setNewTag(e.target.value)}
                  onKeyPress={(e) => {
                    if (e.key === 'Enter') {
                      e.preventDefault();
                      handleAddTag();
                    }
                  }}
                />
                <Button
                  variant="outlined"
                  onClick={handleAddTag}
                  disabled={!newTag.trim()}
                >
                  <AddIcon />
                </Button>
              </Box>
            </Grid>

            {/* Notas */}
            <Grid item xs={12}>
              <TextField
                fullWidth
                label="Notas"
                multiline
                rows={4}
                value={formData.notes || ''}
                onChange={(e) => handleInputChange('notes', e.target.value)}
                placeholder="Adicione observações sobre este lead..."
              />
            </Grid>
          </Grid>
        </DialogContent>

        <DialogActions>
          <Button onClick={onClose} disabled={loading}>
            Cancelar
          </Button>
          <Button
            type="submit"
            variant="contained"
            disabled={loading}
          >
            {loading ? 'Salvando...' : (editingLead ? 'Atualizar' : 'Criar')}
          </Button>
        </DialogActions>
      </form>
    </Dialog>
  );
};

export default LeadForm;