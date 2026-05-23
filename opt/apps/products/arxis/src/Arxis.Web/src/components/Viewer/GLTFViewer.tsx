import React, { useRef, useEffect, Suspense } from 'react';
import { Canvas } from '@react-three/fiber';
import { OrbitControls, useGLTF } from '@react-three/drei';
import { Box, Typography } from '@mui/material';

function Model({ url }: { url: string }) {
  const { scene } = useGLTF(url);
  return <primitive object={scene} />;
}

export const GLTFViewer: React.FC<{ fileUrl: string }> = ({ fileUrl }) => {
  return (
    <Box sx={{ width: '100%', height: 500, background: '#222', borderRadius: 2, mt: 2 }}>
      <Canvas camera={{ position: [2, 2, 2], fov: 50 }} style={{ width: '100%', height: '100%' }}>
        <ambientLight intensity={0.7} />
        <pointLight position={[10, 10, 10]} />
        <Suspense fallback={null}>
          <Model url={fileUrl} />
        </Suspense>
        <OrbitControls />
      </Canvas>
    </Box>
  );
};

// Necessário para o loader GLTF (stub para evitar erro de importação em SSR)
useGLTF.preload = (_path: string | string[]) => {};
