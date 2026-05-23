import React, { useRef } from 'react';
import { Box, Button, Typography, Paper } from '@mui/material';

interface UploadIFCProps {
  onFileSelected: (file: File) => void;
}

export const UploadIFC: React.FC<UploadIFCProps> = ({ onFileSelected }) => {
  const inputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      onFileSelected(e.target.files[0]);
    }
  };

  return (
    <Paper elevation={2} sx={{ p: 3, mb: 2, textAlign: 'center' }}>
      <Typography variant="h6" gutterBottom>
        Upload de arquivo IFC/GLTF
      </Typography>
      <input
        ref={inputRef}
        type="file"
        accept=".ifc,.glb,.gltf"
        style={{ display: 'none' }}
        onChange={handleFileChange}
      />
      <Button variant="contained" onClick={() => inputRef.current?.click()}>
        Selecionar arquivo
      </Button>
    </Paper>
  );
};
