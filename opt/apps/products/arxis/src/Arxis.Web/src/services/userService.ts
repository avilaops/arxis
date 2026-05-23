import axios from 'axios';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:5136/api';

export interface User {
  id: string;
  email: string;
  firstName: string;
  lastName: string;
  phone?: string;
  avatar?: string;
  language?: string;
  isActive: boolean;
  role: string;
  lastLoginAt?: string;
  createdAt: string;
}

export interface UpdateUserRequest {
  firstName?: string;
  lastName?: string;
  phone?: string;
  language?: string;
  role?: string;
  isActive?: boolean;
}

export interface ChangePasswordRequest {
  newPassword: string;
  confirmPassword: string;
}

export interface CreateUserRequest {
  email: string;
  firstName: string;
  lastName: string;
  password: string;
  phone?: string;
  language?: string;
  role?: string;
}

class UserService {
  private getAuthHeaders() {
    const token = localStorage.getItem('token');
    return {
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json',
      },
    };
  }

  async getUsers(): Promise<User[]> {
    const response = await axios.get(`${API_BASE_URL}/users`, this.getAuthHeaders());
    return response.data;
  }

  async getUser(id: string): Promise<User> {
    const response = await axios.get(`${API_BASE_URL}/users/${id}`, this.getAuthHeaders());
    return response.data;
  }

  async updateUser(id: string, userData: UpdateUserRequest): Promise<User> {
    const response = await axios.put(`${API_BASE_URL}/users/${id}`, userData, this.getAuthHeaders());
    return response.data;
  }

  async deleteUser(id: string): Promise<void> {
    await axios.delete(`${API_BASE_URL}/users/${id}`, this.getAuthHeaders());
  }

  async changePassword(id: string, passwordData: ChangePasswordRequest): Promise<void> {
    await axios.post(`${API_BASE_URL}/users/${id}/change-password`, passwordData, this.getAuthHeaders());
  }

  async toggleUserStatus(id: string): Promise<User> {
    const response = await axios.post(`${API_BASE_URL}/users/${id}/toggle-status`, {}, this.getAuthHeaders());
    return response.data;
  }

  async createUser(userData: CreateUserRequest): Promise<User> {
    const response = await axios.post(`${API_BASE_URL}/auth/register`, userData, this.getAuthHeaders());
    return response.data;
  }
}

export const userService = new UserService();