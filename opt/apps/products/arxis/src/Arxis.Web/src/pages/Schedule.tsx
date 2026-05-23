import React, { useState, useEffect } from 'react';
import { Box, Typography, Paper, Button, Stack, TextField, Dialog, DialogTitle, DialogContent, DialogActions } from '@mui/material';
import { ViewList } from 'gantt-task-react';
import axios from 'axios';

interface Task {
  id: string;
  name: string;
  start: Date;
  end: Date;
  progress: number;
  type: 'task' | 'milestone';
  dependencies?: string[];
}

export default function Schedule() {
  const [tasks, setTasks] = useState<Task[]>([]);
  const [loading, setLoading] = useState(false);
  const [importDialogOpen, setImportDialogOpen] = useState(false);
  const [xmlContent, setXmlContent] = useState('');
  const [scheduleName, setScheduleName] = useState('');

  const handleImport = async () => {
    if (!xmlContent || !scheduleName) return;

    setLoading(true);
    try {
      const response = await axios.post('/api/v1/fourd/schedules/import', {
        name: scheduleName,
        projectId: '00000000-0000-0000-0000-000000000000', // TODO: Get from context
        xmlContent,
        baselineDate: new Date().toISOString(),
      });

      // Load activities from the imported schedule
      const activitiesResponse = await axios.get(`/api/v1/fourd/activities/schedule/${response.data.scheduleId}`);
      
      const ganttTasks: Task[] = activitiesResponse.data.map((activity: any) => ({
        id: activity.id,
        name: activity.name,
        start: new Date(activity.startDate),
        end: new Date(activity.finishDate),
        progress: activity.percentComplete || 0,
        type: activity.isMilestone ? 'milestone' : 'task',
      }));

      setTasks(ganttTasks);
      setImportDialogOpen(false);
      setXmlContent('');
      setScheduleName('');
    } catch (error) {
      console.error('Import failed:', error);
    } finally {
      setLoading(false);
    }
  };

  const handleTaskChange = (task: Task) => {
    const newTasks = tasks.map((t) => (t.id === task.id ? task : t));
    setTasks(newTasks);
  };

  const handleTaskDelete = (taskId: string) => {
    const newTasks = tasks.filter((t) => t.id !== taskId);
    setTasks(newTasks);
  };

  return (
    <Box sx={{ p: 4 }}>
      <Paper elevation={3} sx={{ p: 4, minHeight: '80vh', display: 'flex', flexDirection: 'column' }}>
        <Stack direction="row" justifyContent="space-between" alignItems="center" sx={{ mb: 3 }}>
          <Typography variant="h4">
            Cronograma 4D (Gantt)
          </Typography>
          <Button
            variant="contained"
            onClick={() => setImportDialogOpen(true)}
            disabled={loading}
          >
            Importar MS Project XML
          </Button>
        </Stack>

        {tasks.length > 0 ? (
          <Box sx={{ flex: 1, minHeight: 500 }}>
            <ViewList
              tasks={tasks}
              onTaskChange={handleTaskChange}
              onTaskDelete={handleTaskDelete}
              ganttHeight={500}
            />
          </Box>
        ) : (
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
              Nenhum cronograma carregado. Importe um arquivo MS Project XML para começar.
            </Typography>
          </Box>
        )}

        <Dialog open={importDialogOpen} onClose={() => setImportDialogOpen(false)} maxWidth="md" fullWidth>
          <DialogTitle>Importar Cronograma MS Project XML</DialogTitle>
          <DialogContent>
            <Stack spacing={2} sx={{ mt: 2 }}>
              <TextField
                label="Nome do Cronograma"
                fullWidth
                value={scheduleName}
                onChange={(e) => setScheduleName(e.target.value)}
              />
              <TextField
                label="Conteúdo XML"
                fullWidth
                multiline
                rows={10}
                value={xmlContent}
                onChange={(e) => setXmlContent(e.target.value)}
                placeholder="Cole o conteúdo XML do MS Project aqui..."
              />
            </Stack>
          </DialogContent>
          <DialogActions>
            <Button onClick={() => setImportDialogOpen(false)}>Cancelar</Button>
            <Button onClick={handleImport} variant="contained" disabled={loading || !xmlContent || !scheduleName}>
              {loading ? 'Importando...' : 'Importar'}
            </Button>
          </DialogActions>
        </Dialog>
      </Paper>
    </Box>
  );
}
