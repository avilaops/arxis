import React, { useState, useEffect } from 'react';
import { Box, Typography, Paper, Button, Stack, TextField, Dialog, DialogTitle, DialogContent, DialogActions } from '@mui/material';
import { UploadIFC } from '../components/Viewer/UploadIFC';
import { Viewer3D } from '../components/Viewer/Viewer3D';
import { TimelineSlider } from '../components/Viewer/TimelineSlider';
import axios from 'axios';

export default function Viewer() {
  const [file, setFile] = useState<File | null>(null);
  const [fileUrl, setFileUrl] = useState<string | null>(null);
  const [modelId, setModelId] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);
  const [scheduleId, setScheduleId] = useState<string | null>(null);
  const [timelineEnabled, setTimelineEnabled] = useState(false);
  const [selectedDate, setSelectedDate] = useState(new Date());
  const [scheduleDialogOpen, setScheduleDialogOpen] = useState(false);
  const [inputScheduleId, setInputScheduleId] = useState('');

  const handleFileSelected = (f: File) => {
    setFile(f);
    setFileUrl(URL.createObjectURL(f));
  };

  const handleUploadToServer = async () => {
    if (!file) return;

    setLoading(true);
    try {
      const formData = new FormData();
      formData.append('file', file);
      formData.append('projectId', '00000000-0000-0000-0000-000000000000'); // TODO: Get from context
      formData.append('name', file.name);

      const response = await axios.post('/api/v1/fourd/bimmodels/upload', formData, {
        headers: { 'Content-Type': 'multipart/form-data' },
      });

      setModelId(response.data.id);
      setFileUrl(response.data.storagePath);
    } catch (error) {
      console.error('Upload failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleLoadSchedule = () => {
    if (inputScheduleId) {
      setScheduleId(inputScheduleId);
      setTimelineEnabled(true);
      setScheduleDialogOpen(false);
    }
  };

  const handleDateChange = async (date: Date) => {
    setSelectedDate(date);
    
    if (scheduleId) {
      try {
        const response = await axios.get('/api/v1/fourd/links/by-date', {
          params: { scheduleId, date: date.toISOString() },
        });
        
        // TODO: Pass active element IDs to Viewer3D for highlighting
        console.log('Active elements at date:', response.data.activeElements);
      } catch (error) {
        console.error('Failed to get elements by date:', error);
      }
    }
  };

  const getFileType = (): 'ifc' | 'glb' | 'gltf' => {
    if (!file) return 'ifc';
    const ext = file.name.toLowerCase();
    if (ext.endsWith('.ifc') || ext.endsWith('.ifczip')) return 'ifc';
    if (ext.endsWith('.glb')) return 'glb';
    if (ext.endsWith('.gltf')) return 'gltf';
    return 'ifc';
  };

  return (
    <Box sx={{ p: 4 }}>
      <Paper elevation={3} sx={{ p: 4, minHeight: '80vh', display: 'flex', flexDirection: 'column' }}>
        <Stack direction="row" justifyContent="space-between" alignItems="center" sx={{ mb: 3 }}>
          <Typography variant="h4">
            Visualizador 3D BIM (xeokit)
          </Typography>
          <Stack direction="row" spacing={2}>
            <UploadIFC onFileSelected={handleFileSelected} />
            <Button
              variant="outlined"
              onClick={() => setScheduleDialogOpen(true)}
              disabled={timelineEnabled}
            >
              Carregar Cronograma 4D
            </Button>
          </Stack>
        </Stack>

        {file && (
          <Stack direction="row" spacing={2} sx={{ mb: 2 }}>
            <Typography variant="body1" color="primary.main">
              Arquivo: {file.name}
            </Typography>
            <Button
              variant="contained"
              onClick={handleUploadToServer}
              disabled={loading}
              size="small"
            >
              {loading ? 'Enviando...' : 'Enviar para Servidor'}
            </Button>
          </Stack>
        )}

        {timelineEnabled && (
          <TimelineSlider
            minDate={new Date()} // TODO: Get from schedule start date
            maxDate={new Date(Date.now() + 30 * 24 * 60 * 60 * 1000)} // TODO: Get from schedule end date
            value={selectedDate}
            onChange={handleDateChange}
          />
        )}

        {fileUrl && file && (
          <Box sx={{ flex: 1, minHeight: 500, position: 'relative' }}>
            <Viewer3D
              fileUrl={fileUrl}
              fileType={getFileType()}
              onError={(err) => console.error('Viewer error:', err)}
              onLoad={() => console.log('Model loaded')}
            />
          </Box>
        )}

        {!file && (
          <Box
            sx={{
              flex: 1,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center',
              minHeight: 500,
            }}
          >
            <Typography variant="body1" color="text.secondary">
              Selecione um arquivo IFC, IFCZIP, GLB ou GLTF para visualizar
            </Typography>
          </Box>
        )}

        <Dialog open={scheduleDialogOpen} onClose={() => setScheduleDialogOpen(false)}>
          <DialogTitle>Carregar Cronograma 4D</DialogTitle>
          <DialogContent>
            <TextField
              autoFocus
              margin="dense"
              label="ID do Cronograma"
              fullWidth
              variant="outlined"
              value={inputScheduleId}
              onChange={(e) => setInputScheduleId(e.target.value)}
              placeholder="Cole o GUID do cronograma aqui..."
            />
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setScheduleDialogOpen(false)}>Cancelar</Button>
            <Button onClick={handleLoadSchedule} variant="contained" disabled={!inputScheduleId}>
              Carregar
            </Button>
          </DialogActions>
        </Dialog>
      </Paper>
    </Box>
  );
}
