// Sales API Service
const API_BASE_URL = '/api/sales';

class SalesService {
  private getAuthHeaders() {
    const token = localStorage.getItem('token');
    return {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    };
  }

  // Dashboard
  async getDashboard(): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/dashboard`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch dashboard data');
    }

    return response.json();
  }

  // Leads
  async getLeads(): Promise<any[]> {
    const response = await fetch(`${API_BASE_URL}/leads`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch leads');
    }

    return response.json();
  }

  async getLead(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/leads/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch lead');
    }

    return response.json();
  }

  async createLead(lead: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/leads`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(lead),
    });

    if (!response.ok) {
      throw new Error('Failed to create lead');
    }

    return response.json();
  }

  async updateLead(id: string, lead: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/leads/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(lead),
    });

    if (!response.ok) {
      throw new Error('Failed to update lead');
    }

    return response.json();
  }

  async deleteLead(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/leads/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete lead');
    }
  }

  async convertLeadToOpportunity(leadId: string, opportunity: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/leads/${leadId}/convert`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(opportunity),
    });

    if (!response.ok) {
      throw new Error('Failed to convert lead to opportunity');
    }

    return response.json();
  }

  // Contacts
  async getContacts(): Promise<any[]> {
    const response = await fetch(`${API_BASE_URL}/contacts`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch contacts');
    }

    return response.json();
  }

  async getContact(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/contacts/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch contact');
    }

    return response.json();
  }

  async createContact(contact: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/contacts`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(contact),
    });

    if (!response.ok) {
      throw new Error('Failed to create contact');
    }

    return response.json();
  }

  async updateContact(id: string, contact: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/contacts/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(contact),
    });

    if (!response.ok) {
      throw new Error('Failed to update contact');
    }

    return response.json();
  }

  async deleteContact(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/contacts/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete contact');
    }
  }

  // Opportunities
  async getOpportunities(): Promise<any[]> {
    const response = await fetch(`${API_BASE_URL}/opportunities`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch opportunities');
    }

    return response.json();
  }

  async getOpportunity(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/opportunities/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch opportunity');
    }

    return response.json();
  }

  async createOpportunity(opportunity: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/opportunities`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(opportunity),
    });

    if (!response.ok) {
      throw new Error('Failed to create opportunity');
    }

    return response.json();
  }

  async updateOpportunity(id: string, opportunity: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/opportunities/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(opportunity),
    });

    if (!response.ok) {
      throw new Error('Failed to update opportunity');
    }

    return response.json();
  }

  async deleteOpportunity(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/opportunities/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete opportunity');
    }
  }

  // Activities
  async getActivities(fromDate?: string, toDate?: string): Promise<any[]> {
    const params = new URLSearchParams();
    if (fromDate) params.append('fromDate', fromDate);
    if (toDate) params.append('toDate', toDate);

    const response = await fetch(`${API_BASE_URL}/activities?${params}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch activities');
    }

    return response.json();
  }

  async getActivity(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/activities/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch activity');
    }

    return response.json();
  }

  async createActivity(activity: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/activities`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(activity),
    });

    if (!response.ok) {
      throw new Error('Failed to create activity');
    }

    return response.json();
  }

  async updateActivity(id: string, activity: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/activities/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(activity),
    });

    if (!response.ok) {
      throw new Error('Failed to update activity');
    }

    return response.json();
  }

  async deleteActivity(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/activities/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete activity');
    }
  }

  // Cadences
  async getCadences(): Promise<any[]> {
    const response = await fetch(`${API_BASE_URL}/cadences`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch cadences');
    }

    return response.json();
  }

  async getCadence(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/cadences/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch cadence');
    }

    return response.json();
  }

  async createCadence(cadence: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/cadences`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(cadence),
    });

    if (!response.ok) {
      throw new Error('Failed to create cadence');
    }

    return response.json();
  }

  async updateCadence(id: string, cadence: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/cadences/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(cadence),
    });

    if (!response.ok) {
      throw new Error('Failed to update cadence');
    }

    return response.json();
  }

  async deleteCadence(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/cadences/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete cadence');
    }
  }

  // Templates
  async getTemplates(type?: string, stage?: string): Promise<any[]> {
    const params = new URLSearchParams();
    if (type) params.append('type', type);
    if (stage) params.append('stage', stage);

    const response = await fetch(`${API_BASE_URL}/templates?${params}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch templates');
    }

    return response.json();
  }

  async getTemplate(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/templates/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch template');
    }

    return response.json();
  }

  async createTemplate(template: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/templates`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(template),
    });

    if (!response.ok) {
      throw new Error('Failed to create template');
    }

    return response.json();
  }

  async updateTemplate(id: string, template: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/templates/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(template),
    });

    if (!response.ok) {
      throw new Error('Failed to update template');
    }

    return response.json();
  }

  async deleteTemplate(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/templates/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete template');
    }
  }

  // Scripts
  async getScripts(stage?: string): Promise<any[]> {
    const params = new URLSearchParams();
    if (stage) params.append('stage', stage);

    const response = await fetch(`${API_BASE_URL}/scripts?${params}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch scripts');
    }

    return response.json();
  }

  async getScript(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/scripts/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch script');
    }

    return response.json();
  }

  async createScript(script: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/scripts`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(script),
    });

    if (!response.ok) {
      throw new Error('Failed to create script');
    }

    return response.json();
  }

  async updateScript(id: string, script: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/scripts/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(script),
    });

    if (!response.ok) {
      throw new Error('Failed to update script');
    }

    return response.json();
  }

  async deleteScript(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/scripts/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete script');
    }
  }

  // Proposals
  async getProposals(opportunityId?: string): Promise<any[]> {
    const params = new URLSearchParams();
    if (opportunityId) params.append('opportunityId', opportunityId);

    const response = await fetch(`${API_BASE_URL}/proposals?${params}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch proposals');
    }

    return response.json();
  }

  async getProposal(id: string): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/proposals/${id}`, {
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to fetch proposal');
    }

    return response.json();
  }

  async createProposal(proposal: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/proposals`, {
      method: 'POST',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(proposal),
    });

    if (!response.ok) {
      throw new Error('Failed to create proposal');
    }

    return response.json();
  }

  async updateProposal(id: string, proposal: any): Promise<any> {
    const response = await fetch(`${API_BASE_URL}/proposals/${id}`, {
      method: 'PUT',
      headers: this.getAuthHeaders(),
      body: JSON.stringify(proposal),
    });

    if (!response.ok) {
      throw new Error('Failed to update proposal');
    }

    return response.json();
  }

  async deleteProposal(id: string): Promise<void> {
    const response = await fetch(`${API_BASE_URL}/proposals/${id}`, {
      method: 'DELETE',
      headers: this.getAuthHeaders(),
    });

    if (!response.ok) {
      throw new Error('Failed to delete proposal');
    }
  }
}

export const salesService = new SalesService();