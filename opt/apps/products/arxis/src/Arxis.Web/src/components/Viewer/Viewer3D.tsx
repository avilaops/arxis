import React, { useEffect, useRef } from 'react';
import { Box, CircularProgress, Typography } from '@mui/material';
import { Viewer } from 'xeokit-sdk';

interface Viewer3DProps {
  fileUrl: string;
  fileType: 'ifc' | 'glb' | 'gltf';
  onError?: (error: Error) => void;
  onLoad?: () => void;
}

export function Viewer3D({ fileUrl, fileType, onError, onLoad }: Viewer3DProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const viewerRef = useRef<Viewer | null>(null);
  const [loading, setLoading] = React.useState(true);
  const [error, setError] = React.useState<string | null>(null);

  useEffect(() => {
    if (!canvasRef.current) return;

    let viewer: Viewer | null = null;

    const initViewer = async () => {
      try {
        setLoading(true);
        setError(null);

        // Create xeokit viewer
        viewer = new Viewer({
          canvasId: canvasRef.current.id,
          transparent: true,
        });

        viewerRef.current = viewer;

        // Configure camera and scene
        viewer.cameraControl.pivotElem = canvasRef.current;
        viewer.scene.canvas.canvas = canvasRef.current;

        // Load model based on file type
        if (fileType === 'ifc') {
          // Load IFC model
          const model = viewer.scene.loadIFCModel({
            src: fileUrl,
            edges: true,
          });

          model.on('loaded', () => {
            setLoading(false);
            viewer?.cameraFit();
            onLoad?.();
          });

          model.on('error', (err: Error) => {
            setLoading(false);
            setError(err.message);
            onError?.(err);
          });
        } else {
          // Load glTF/GLB model
          const model = viewer.scene.loadGLTFModel({
            src: fileUrl,
            edges: true,
          });

          model.on('loaded', () => {
            setLoading(false);
            viewer?.cameraFit();
            onLoad?.();
          });

          model.on('error', (err: Error) => {
            setLoading(false);
            setError(err.message);
            onError?.(err);
          });
        }
      } catch (err) {
        setLoading(false);
        setError(err instanceof Error ? err.message : 'Unknown error');
        onError?.(err instanceof Error ? err : new Error('Unknown error'));
      }
    };

    initViewer();

    // Cleanup
    return () => {
      if (viewer) {
        viewer.scene.clear();
        viewer.destroy();
      }
    };
  }, [fileUrl, fileType, onError, onLoad]);

  return (
    <Box sx={{ width: '100%', height: '100%', position: 'relative' }}>
      <canvas
        ref={canvasRef}
        id={`xeokit-canvas-${Date.now()}`}
        style={{ width: '100%', height: '100%', outline: 'none' }}
      />
      {loading && (
        <Box
          sx={{
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            backgroundColor: 'rgba(0, 0, 0, 0.5)',
            zIndex: 1000,
          }}
        >
          <Box sx={{ textAlign: 'center' }}>
            <CircularProgress size={60} />
            <Typography variant="body1" sx={{ mt: 2, color: 'white' }}>
              Carregando modelo 3D...
            </Typography>
          </Box>
        </Box>
      )}
      {error && (
        <Box
          sx={{
            position: 'absolute',
            top: 0,
            left: 0,
            right: 0,
            bottom: 0,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            backgroundColor: 'rgba(0, 0, 0, 0.7)',
            zIndex: 1000,
          }}
        >
          <Typography variant="body1" color="error" sx={{ textAlign: 'center', px: 2 }}>
            Erro ao carregar modelo: {error}
          </Typography>
        </Box>
      )}
    </Box>
  );
}
