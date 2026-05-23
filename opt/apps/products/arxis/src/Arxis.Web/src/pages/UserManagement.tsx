import React, { useEffect, useState } from 'react';
import {
  Box,
  Card,
  CardContent,
  Typography,
  Button,
  Table,
  TableBody,
  TableCell,
  TableContainer,
  TableHead,
  TableRow,
  Paper,
  Chip,
  IconButton,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  FormControl,
  InputLabel,
  Select,
  MenuItem,
  Alert,
  Snackbar,
  Avatar,
  Switch,
  FormControlLabel,
} from '@mui/material';
import {
  Edit,
  Delete,
  PersonAdd,
  Lock,
  Block,
  CheckCircle,
} from '@mui/icons-material';
import { userService, User, UpdateUserRequest, ChangePasswordRequest, CreateUserRequest } from '../services/userService';

const UserManagement: React.FC = () => {
  const [users, setUsers] = useState<User[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);

  // Dialog states
  const [editDialogOpen, setEditDialogOpen] = useState(false);
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [passwordDialogOpen, setPasswordDialogOpen] = useState(false);
  const [selectedUser, setSelectedUser] = useState<User | null>(null);

  // Form states
  const [editForm, setEditForm] = useState<UpdateUserRequest>({});
  const [createForm, setCreateForm] = useState<CreateUserRequest>({
    email: '',
    firstName: '',
    lastName: '',
    password: '',
    role: 'User',
  });
  const [passwordForm, setPasswordForm] = useState<ChangePasswordRequest>({
    newPassword: '',
    confirmPassword: '',
  });

  useEffect(() => {
    loadUsers();
  }, []);

  const loadUsers = async () => {
    try {
      setLoading(true);
      const userData = await userService.getUsers();
      setUsers(userData);
    } catch (err) {
      setError('Erro ao carregar usuários');
      console.error('Error loading users:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleEditUser = (user: User) => {
    setSelectedUser(user);
    setEditForm({
      firstName: user.firstName,
      lastName: user.lastName,
      phone: user.phone,
      language: user.language,
      role: user.role,
      isActive: user.isActive,
    });
    setEditDialogOpen(true);
  };

  const handleSaveEdit = async () => {
    if (!selectedUser) return;

    try {
      await userService.updateUser(selectedUser.id, editForm);
      setSuccess('Usuário atualizado com sucesso');
      setEditDialogOpen(false);
      loadUsers();
    } catch (err) {
      setError('Erro ao atualizar usuário');
      console.error('Error updating user:', err);
    }
  };

  const handleCreateUser = async () => {
    try {
      await userService.createUser(createForm);
      setSuccess('Usuário criado com sucesso');
      setCreateDialogOpen(false);
      setCreateForm({
        email: '',
        firstName: '',
        lastName: '',
        password: '',
        role: 'User',
      });
      loadUsers();
    } catch (err) {
      setError('Erro ao criar usuário');
      console.error('Error creating user:', err);
    }
  };

  const handleDeleteUser = async (userId: string) => {
    if (!window.confirm('Tem certeza que deseja excluir este usuário?')) return;

    try {
      await userService.deleteUser(userId);
      setSuccess('Usuário excluído com sucesso');
      loadUsers();
    } catch (err) {
      setError('Erro ao excluir usuário');
      console.error('Error deleting user:', err);
    }
  };

  const handleToggleStatus = async (userId: string) => {
    try {
      await userService.toggleUserStatus(userId);
      setSuccess('Status do usuário alterado com sucesso');
      loadUsers();
    } catch (err) {
      setError('Erro ao alterar status do usuário');
      console.error('Error toggling user status:', err);
    }
  };

  const handleChangePassword = async () => {
    if (!selectedUser) return;

    if (passwordForm.newPassword !== passwordForm.confirmPassword) {
      setError('As senhas não coincidem');
      return;
    }

    try {
      await userService.changePassword(selectedUser.id, passwordForm);
      setSuccess('Senha alterada com sucesso');
      setPasswordDialogOpen(false);
      setPasswordForm({ newPassword: '', confirmPassword: '' });
    } catch (err) {
      setError('Erro ao alterar senha');
      console.error('Error changing password:', err);
    }
  };

  const getRoleColor = (role: string) => {
    switch (role) {
      case 'Admin': return 'error';
      case 'Manager': return 'warning';
      case 'User': return 'primary';
      default: return 'default';
    }
  };

  const formatDate = (dateString?: string) => {
    if (!dateString) return 'Nunca';
    return new Date(dateString).toLocaleDateString('pt-BR');
  };

  if (loading) {
    return (
      <Box display="flex" justifyContent="center" alignItems="center" minHeight="400px">
        <Typography>Carregando usuários...</Typography>
      </Box>
    );
  }

  return (
    <Box>
      <Box display="flex" justifyContent="space-between" alignItems="center" mb={3}>
        <Typography variant="h4" component="h1">
          Gestão de Usuários
        </Typography>
        <Button
          variant="contained"
          startIcon={<PersonAdd />}
          onClick={() => setCreateDialogOpen(true)}
        >
          Novo Usuário
        </Button>
      </Box>

      <Card>
        <CardContent>
          <TableContainer component={Paper}>
            <Table>
              <TableHead>
                <TableRow>
                  <TableCell>Usuário</TableCell>
                  <TableCell>Email</TableCell>
                  <TableCell>Função</TableCell>
                  <TableCell>Status</TableCell>
                  <TableCell>Último Login</TableCell>
                  <TableCell>Criado em</TableCell>
                  <TableCell align="center">Ações</TableCell>
                </TableRow>
              </TableHead>
              <TableBody>
                {users.map((user) => (
                  <TableRow key={user.id}>
                    <TableCell>
                      <Box display="flex" alignItems="center" gap={1}>
                        <Avatar sx={{ width: 32, height: 32 }}>
                          {user.firstName?.[0]}{user.lastName?.[0]}
                        </Avatar>
                        <Typography>
                          {user.firstName} {user.lastName}
                        </Typography>
                      </Box>
                    </TableCell>
                    <TableCell>{user.email}</TableCell>
                    <TableCell>
                      <Chip
                        label={user.role}
                        color={getRoleColor(user.role)}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>
                      <Chip
                        label={user.isActive ? 'Ativo' : 'Inativo'}
                        color={user.isActive ? 'success' : 'error'}
                        size="small"
                      />
                    </TableCell>
                    <TableCell>{formatDate(user.lastLoginAt)}</TableCell>
                    <TableCell>{formatDate(user.createdAt)}</TableCell>
                    <TableCell align="center">
                      <IconButton
                        size="small"
                        onClick={() => handleEditUser(user)}
                        title="Editar"
                      >
                        <Edit />
                      </IconButton>
                      <IconButton
                        size="small"
                        onClick={() => {
                          setSelectedUser(user);
                          setPasswordDialogOpen(true);
                        }}
                        title="Alterar senha"
                      >
                        <Lock />
                      </IconButton>
                      <IconButton
                        size="small"
                        onClick={() => handleToggleStatus(user.id)}
                        title={user.isActive ? 'Desativar' : 'Ativar'}
                      >
                        {user.isActive ? <Block /> : <CheckCircle />}
                      </IconButton>
                      <IconButton
                        size="small"
                        color="error"
                        onClick={() => handleDeleteUser(user.id)}
                        title="Excluir"
                      >
                        <Delete />
                      </IconButton>
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          </TableContainer>
        </CardContent>
      </Card>

      {/* Edit User Dialog */}
      <Dialog open={editDialogOpen} onClose={() => setEditDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Editar Usuário</DialogTitle>
        <DialogContent>
          <Box component="form" sx={{ mt: 1 }}>
            <TextField
              fullWidth
              label="Nome"
              value={editForm.firstName || ''}
              onChange={(e) => setEditForm({ ...editForm, firstName: e.target.value })}
              margin="normal"
            />
            <TextField
              fullWidth
              label="Sobrenome"
              value={editForm.lastName || ''}
              onChange={(e) => setEditForm({ ...editForm, lastName: e.target.value })}
              margin="normal"
            />
            <TextField
              fullWidth
              label="Telefone"
              value={editForm.phone || ''}
              onChange={(e) => setEditForm({ ...editForm, phone: e.target.value })}
              margin="normal"
            />
            <FormControl fullWidth margin="normal">
              <InputLabel>Função</InputLabel>
              <Select
                value={editForm.role || 'User'}
                onChange={(e) => setEditForm({ ...editForm, role: e.target.value })}
              >
                <MenuItem value="User">Usuário</MenuItem>
                <MenuItem value="Manager">Gerente</MenuItem>
                <MenuItem value="Admin">Administrador</MenuItem>
              </Select>
            </FormControl>
            <FormControlLabel
              control={
                <Switch
                  checked={editForm.isActive || false}
                  onChange={(e) => setEditForm({ ...editForm, isActive: e.target.checked })}
                />
              }
              label="Ativo"
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setEditDialogOpen(false)}>Cancelar</Button>
          <Button onClick={handleSaveEdit} variant="contained">Salvar</Button>
        </DialogActions>
      </Dialog>

      {/* Create User Dialog */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Criar Novo Usuário</DialogTitle>
        <DialogContent>
          <Box component="form" sx={{ mt: 1 }}>
            <TextField
              fullWidth
              label="Email"
              type="email"
              value={createForm.email}
              onChange={(e) => setCreateForm({ ...createForm, email: e.target.value })}
              margin="normal"
              required
            />
            <TextField
              fullWidth
              label="Nome"
              value={createForm.firstName}
              onChange={(e) => setCreateForm({ ...createForm, firstName: e.target.value })}
              margin="normal"
              required
            />
            <TextField
              fullWidth
              label="Sobrenome"
              value={createForm.lastName}
              onChange={(e) => setCreateForm({ ...createForm, lastName: e.target.value })}
              margin="normal"
              required
            />
            <TextField
              fullWidth
              label="Senha"
              type="password"
              value={createForm.password}
              onChange={(e) => setCreateForm({ ...createForm, password: e.target.value })}
              margin="normal"
              required
            />
            <TextField
              fullWidth
              label="Telefone"
              value={createForm.phone || ''}
              onChange={(e) => setCreateForm({ ...createForm, phone: e.target.value })}
              margin="normal"
            />
            <FormControl fullWidth margin="normal">
              <InputLabel>Função</InputLabel>
              <Select
                value={createForm.role || 'User'}
                onChange={(e) => setCreateForm({ ...createForm, role: e.target.value })}
              >
                <MenuItem value="User">Usuário</MenuItem>
                <MenuItem value="Manager">Gerente</MenuItem>
                <MenuItem value="Admin">Administrador</MenuItem>
              </Select>
            </FormControl>
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>Cancelar</Button>
          <Button onClick={handleCreateUser} variant="contained">Criar</Button>
        </DialogActions>
      </Dialog>

      {/* Change Password Dialog */}
      <Dialog open={passwordDialogOpen} onClose={() => setPasswordDialogOpen(false)} maxWidth="sm" fullWidth>
        <DialogTitle>Alterar Senha</DialogTitle>
        <DialogContent>
          <Box component="form" sx={{ mt: 1 }}>
            <TextField
              fullWidth
              label="Nova Senha"
              type="password"
              value={passwordForm.newPassword}
              onChange={(e) => setPasswordForm({ ...passwordForm, newPassword: e.target.value })}
              margin="normal"
              required
            />
            <TextField
              fullWidth
              label="Confirmar Senha"
              type="password"
              value={passwordForm.confirmPassword}
              onChange={(e) => setPasswordForm({ ...passwordForm, confirmPassword: e.target.value })}
              margin="normal"
              required
            />
          </Box>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setPasswordDialogOpen(false)}>Cancelar</Button>
          <Button onClick={handleChangePassword} variant="contained">Alterar</Button>
        </DialogActions>
      </Dialog>

      {/* Snackbar for messages */}
      <Snackbar
        open={!!error}
        autoHideDuration={6000}
        onClose={() => setError(null)}
      >
        <Alert onClose={() => setError(null)} severity="error">
          {error}
        </Alert>
      </Snackbar>

      <Snackbar
        open={!!success}
        autoHideDuration={6000}
        onClose={() => setSuccess(null)}
      >
        <Alert onClose={() => setSuccess(null)} severity="success">
          {success}
        </Alert>
      </Snackbar>
    </Box>
  );
};

export default UserManagement;