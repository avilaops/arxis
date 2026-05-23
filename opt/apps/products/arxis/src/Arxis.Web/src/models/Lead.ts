export enum LeadStatus {
  New = 'New',
  Contacted = 'Contacted',
  Qualified = 'Qualified',
  Proposal = 'Proposal',
  Negotiation = 'Negotiation',
  ClosedWon = 'ClosedWon',
  ClosedLost = 'ClosedLost'
}

export enum LeadSource {
  Website = 'Website',
  Referral = 'Referral',
  SocialMedia = 'SocialMedia',
  Email = 'Email',
  Phone = 'Phone',
  TradeShow = 'TradeShow',
  Direct = 'Direct',
  Other = 'Other'
}

export enum LeadTemperature {
  Hot = 'Hot',
  Warm = 'Warm',
  Cold = 'Cold'
}

export interface LeadDto {
  id: string;
  firstName?: string;
  lastName?: string;
  title?: string;
  company?: string;
  email?: string;
  phone?: string;
  website?: string;
  address?: string;
  city?: string;
  state?: string;
  country?: string;
  postalCode?: string;
  status: LeadStatus;
  source: LeadSource;
  temperature: LeadTemperature;
  estimatedValue?: number;
  notes?: string;
  assignedToUserId?: string;
  assignedToUser?: {
    id: string;
    firstName: string;
    lastName: string;
    email: string;
  };
  createdAt: string;
  updatedAt: string;
  lastContactDate?: string;
  nextFollowUpDate?: string;
  tags?: string[];
  customFields?: Record<string, any>;
}

export interface CreateLeadRequest {
  firstName?: string;
  lastName?: string;
  title?: string;
  company?: string;
  email?: string;
  phone?: string;
  website?: string;
  address?: string;
  city?: string;
  state?: string;
  country?: string;
  postalCode?: string;
  status: LeadStatus;
  source: LeadSource;
  temperature: LeadTemperature;
  estimatedValue?: number;
  notes?: string;
  assignedToUserId?: string;
  lastContactDate?: string;
  nextFollowUpDate?: string;
  tags?: string[];
  customFields?: Record<string, any>;
}

export interface UpdateLeadRequest extends Partial<CreateLeadRequest> {
  id: string;
}

export interface LeadSummaryDto {
  totalLeads: number;
  newLeads: number;
  qualifiedLeads: number;
  convertedLeads: number;
  totalValue: number;
  averageValue: number;
  conversionRate: number;
}