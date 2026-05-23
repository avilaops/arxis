import React from 'react';
import { Box, Typography } from '@mui/material';

interface Viewer3DProps {
  fileUrl: string;
  fileType: 'ifc' | 'glb' | 'gltf';
  onError?: (error: Error) => void;
  onLoad?: () => void;
}

export function Viewer3D({ fileUrl, fileType, onError, onLoad }: Viewer3DProps) {
  // Placeholder component - xeokit-sdk not available in npm
  // TODO: Implement 3D viewer with alternative library

  React.useEffect(() => {
    console.log('Viewer3D placeholder - file:', fileUrl, 'type:', fileType);
    onLoad?.();
  }, [fileUrl, fileType, onLoad]);

  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        backgroundColor: '#f5f5f5',
        border: '2px dashed #ccc',
      }}
    >
      <Typography variant="body1" color="textSecondary" sx={{ textAlign: 'center', px: 2 }}>
        Visualizador 3D temporariamente desabilitado.<br />
        Arquivo: {fileUrl}
      </Typography>
    </Box>
  );
}
