import React, { useEffect, useState } from 'react';
import {
  Snackbar,
  Alert,
  AlertTitle,
  Box,
  IconButton,
  Slide,
  SlideProps
} from '@mui/material';
import { Close } from '@mui/icons-material';
import { signalrService, CrmNotification } from '../../services/signalrService';

function SlideTransition(props: SlideProps) {
  return <Slide {...props} direction="left" />;
}

interface NotificationWithId extends CrmNotification {
  id: string;
}

export const CrmNotifications: React.FC = () => {
  const [notifications, setNotifications] = useState<NotificationWithId[]>([]);

  useEffect(() => {
    const handleNotification = (notification: CrmNotification) => {
      const notificationWithId: NotificationWithId = {
        ...notification,
        id: `${Date.now()}-${Math.random()}`
      };
      
      setNotifications((prev) => [notificationWithId, ...prev].slice(0, 5));

      // Auto-remover após 5 segundos
      setTimeout(() => {
        setNotifications((prev) => prev.filter((n) => n.id !== notificationWithId.id));
      }, 5000);
    };

    // Inscrever em todos os tipos de notificações
    signalrService.onNotification('all', handleNotification);

    return () => {
      signalrService.offNotification('all', handleNotification);
    };
  }, []);

  const handleClose = (id: string) => {
    setNotifications((prev) => prev.filter((n) => n.id !== id));
  };

  const getSeverity = (color: string): 'success' | 'info' | 'warning' | 'error' => {
    switch (color) {
      case 'success':
        return 'success';
      case 'error':
      case 'danger':
        return 'error';
      case 'warning':
        return 'warning';
      case 'info':
      default:
        return 'info';
    }
  };

  return (
    <Box
      sx={{
        position: 'fixed',
        top: 80,
        right: 20,
        zIndex: 9999,
        display: 'flex',
        flexDirection: 'column',
        gap: 1,
        maxWidth: 400
      }}
    >
      {notifications.map((notification, index) => (
        <Snackbar
          key={notification.id}
          open={true}
          anchorOrigin={{ vertical: 'top', horizontal: 'right' }}
          TransitionComponent={SlideTransition}
          sx={{
            position: 'relative',
            top: `${index * 90}px`,
            right: 0
          }}
        >
          <Alert
            severity={getSeverity(notification.color)}
            icon={<span style={{ fontSize: '20px' }}>{notification.icon}</span>}
            action={
              <IconButton
                size="small"
                color="inherit"
                onClick={() => handleClose(notification.id)}
              >
                <Close fontSize="small" />
              </IconButton>
            }
            sx={{
              width: '100%',
              maxWidth: 400,
              boxShadow: 3,
              '& .MuiAlert-message': {
                width: '100%'
              }
            }}
          >
            <AlertTitle sx={{ fontWeight: 600 }}>
              {notification.type === 'new_lead' && 'Novo Lead'}
              {notification.type === 'opportunity_won' && 'Oportunidade Ganha! 🎉'}
              {notification.type === 'opportunity_lost' && 'Oportunidade Perdida'}
              {notification.type === 'cadence_executed' && 'Cadência Executada'}
              {notification.type === 'activity_assigned' && 'Nova Atividade'}
              {notification.type === 'admin_notification' && 'Notificação Admin'}
              {notification.type === 'report_generated' && 'Relatório Gerado'}
            </AlertTitle>
            {notification.message}
          </Alert>
        </Snackbar>
      ))}
    </Box>
  );
};
