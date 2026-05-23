import React from 'react';
import { Box, Slider, Typography, Paper } from '@mui/material';

interface TimelineSliderProps {
  minDate: Date;
  maxDate: Date;
  value: Date;
  onChange: (date: Date) => void;
  disabled?: boolean;
}

export function TimelineSlider({ minDate, maxDate, value, onChange, disabled = false }: TimelineSliderProps) {
  // Convert dates to timestamps for slider
  const minTimestamp = minDate.getTime();
  const maxTimestamp = maxDate.getTime();
  const valueTimestamp = value.getTime();

  // Convert timestamp back to date
  const handleSliderChange = (_: Event, newValue: number | number[]) => {
    const newTimestamp = Array.isArray(newValue) ? newValue[0] : newValue;
    onChange(new Date(newTimestamp));
  };

  // Format date for display
  const formatDate = (date: Date): string => {
    return date.toLocaleDateString('pt-BR', {
      day: '2-digit',
      month: '2-digit',
      year: 'numeric',
    });
  };

  // Calculate percentage for progress
  const progress = ((valueTimestamp - minTimestamp) / (maxTimestamp - minTimestamp)) * 100;

  return (
    <Paper elevation={2} sx={{ p: 3, mb: 2 }}>
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        <Typography variant="body2" color="text.secondary" sx={{ minWidth: 100 }}>
          {formatDate(minDate)}
        </Typography>
        <Box sx={{ flex: 1 }}>
          <Slider
            min={minTimestamp}
            max={maxTimestamp}
            value={valueTimestamp}
            onChange={handleSliderChange}
            disabled={disabled}
            step={86400000} // 1 day in milliseconds
            marks={[
              { value: minTimestamp, label: formatDate(minDate) },
              { value: maxTimestamp, label: formatDate(maxDate) },
            ]}
            valueLabelDisplay="auto"
            valueLabelFormat={(value) => formatDate(new Date(value))}
            sx={{
              '& .MuiSlider-thumb': {
                width: 20,
                height: 20,
              },
            }}
          />
        </Box>
        <Typography variant="body2" color="text.secondary" sx={{ minWidth: 100, textAlign: 'right' }}>
          {formatDate(maxDate)}
        </Typography>
      </Box>
      <Box sx={{ mt: 2, display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
        <Typography variant="body1" fontWeight="bold">
          Data Atual: {formatDate(value)}
        </Typography>
        <Typography variant="body2" color="text.secondary">
          Progresso: {progress.toFixed(1)}%
        </Typography>
      </Box>
    </Paper>
  );
}
