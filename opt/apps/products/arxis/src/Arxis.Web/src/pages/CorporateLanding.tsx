import React, { useState } from 'react';
import { Link } from 'react-router-dom';
import emailService from '../services/emailService';
import './CorporateLanding.css';

interface LeadFormState {
  name: string;
  email: string;
  company: string;
  phone: string;
  role: string;
}

const initialFormState: LeadFormState = {
  name: '',
  email: '',
  company: '',
  phone: '',
  role: '',
};

export const CorporateLanding: React.FC = () => {
  const [formState, setFormState] = useState<LeadFormState>(initialFormState);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [statusMessage, setStatusMessage] = useState<string | null>(null);
  const [statusType, setStatusType] = useState<'success' | 'error' | null>(null);

  const handleChange = (event: React.ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
    const { name, value } = event.target;
    setFormState((prev) => ({
      ...prev,
      [name]: value,
    }));
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (!formState.email || !formState.name || !formState.company) {
      setStatusType('error');
      setStatusMessage('Preencha nome, email e empresa para continuar.');
      return;
    }

    try {
      setIsSubmitting(true);
      setStatusMessage(null);
      setStatusType(null);

      await emailService.sendWelcomeEmail(formState.email, formState.name);

      setStatusType('success');
      setStatusMessage('Recebemos seu contato. Nossa equipe comercial entrará em contato em até 24h úteis.');
      setFormState(initialFormState);
    } catch (error) {
      console.error('Falha ao enviar formulário corporativo', error);
      setStatusType('error');
      setStatusMessage('Não foi possível enviar seu contato. Tente novamente ou entre em contato pelo email comercial@arxis.com.br');
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="corporate-landing">
      <header className="corporate-hero">
        <div className="corporate-hero__content">
          <div className="corporate-hero__badge">
            <span className="badge-icon">🏢</span>
            <span>Solução Enterprise para Construção Civil</span>
          </div>
          <h1>Transforme a gestão de projetos em vantagem competitiva</h1>
          <p className="corporate-hero__subtitle">
            Plataforma corporativa que integra planejamento, execução e controle de obras em tempo real. 
            Reduza custos, mitigue riscos e aumente a produtividade com inteligência de dados e automação.
          </p>
          <div className="corporate-hero__metrics">
            <div className="hero-metric">
              <span className="hero-metric__value">R$ 2.4B+</span>
              <span className="hero-metric__label">em obras gerenciadas</span>
            </div>
            <div className="hero-metric">
              <span className="hero-metric__value">180+</span>
              <span className="hero-metric__label">empresas clientes</span>
            </div>
            <div className="hero-metric">
              <span className="hero-metric__value">99.9%</span>
              <span className="hero-metric__label">SLA garantido</span>
            </div>
          </div>
          <div className="corporate-hero__actions">
            <a className="cta-primary" href="#contact">Solicitar proposta comercial</a>
            <a className="cta-secondary" href="#demo">Agendar demonstração técnica</a>
          </div>
        </div>
        <div className="corporate-hero__visual">
          <div className="enterprise-dashboard">
            <div className="dashboard-header">
              <div className="dashboard-dots">
                <span></span>
                <span></span>
                <span></span>
              </div>
              <span className="dashboard-title">Arxis Enterprise Dashboard</span>
            </div>
            <div className="dashboard-content">
              <div className="dashboard-kpi">
                <span className="kpi-label">ROI Projetado</span>
                <span className="kpi-value kpi-value--positive">+34%</span>
              </div>
              <div className="dashboard-kpi">
                <span className="kpi-label">Eficiência Operacional</span>
                <span className="kpi-value">87%</span>
              </div>
              <div className="dashboard-chart">
                <div className="chart-bar chart-bar--1"></div>
                <div className="chart-bar chart-bar--2"></div>
                <div className="chart-bar chart-bar--3"></div>
                <div className="chart-bar chart-bar--4"></div>
                <div className="chart-bar chart-bar--5"></div>
              </div>
            </div>
          </div>
        </div>
      </header>

      <section className="corporate-trust" id="clientes">
        <h2>Empresas que confiam no Arxis</h2>
        <div className="trust-logos">
          <div className="trust-logo">Construtora Atlas</div>
          <div className="trust-logo">Grupo Odebrecht</div>
          <div className="trust-logo">MRV Engenharia</div>
          <div className="trust-logo">Tecnisa</div>
          <div className="trust-logo">Cyrela</div>
          <div className="trust-logo">Even</div>
        </div>
      </section>

      <section className="corporate-value" id="valor">
        <div className="section-header">
          <h2>Valor Corporativo Tangível</h2>
          <p>Resultados comprovados em grandes operações de construção civil</p>
        </div>
        <div className="value-grid">
          <article className="value-card">
            <div className="value-icon">📊</div>
            <h3>Visibilidade 360º</h3>
            <p>Dashboards executivos com KPIs em tempo real de todas as obras. Tomada de decisão baseada em dados, não em planilhas.</p>
            <ul>
              <li>Burn rate financeiro por projeto</li>
              <li>Progresso físico vs planejado</li>
              <li>Alertas de desvio de cronograma</li>
              <li>Analytics preditivo de riscos</li>
            </ul>
          </article>
          <article className="value-card">
            <div className="value-icon">⚡</div>
            <h3>Automação Inteligente</h3>
            <p>Workflows automatizados que eliminam tarefas manuais e reduzem erros humanos. Aprovações em minutos, não em dias.</p>
            <ul>
              <li>Fluxos de aprovação customizáveis</li>
              <li>Integração nativa ERP (TOTVS, SAP)</li>
              <li>RPA para conciliação de dados</li>
              <li>Notificações proativas de SLA</li>
            </ul>
          </article>
          <article className="value-card">
            <div className="value-icon">🔒</div>
            <h3>Governança e Compliance</h3>
            <p>Controle granular de permissões, audit trail completo e conformidade com normas brasileiras e internacionais.</p>
            <ul>
              <li>RBAC avançado por hierarquia</li>
              <li>Audit trail imutável</li>
              <li>Conformidade ISO 9001/14001</li>
              <li>SSO e MFA corporativo</li>
            </ul>
          </article>
        </div>
      </section>

      <section className="corporate-features" id="recursos">
        <div className="section-header">
          <h2>Recursos Enterprise</h2>
          <p>Funcionalidades desenvolvidas para grandes operações</p>
        </div>
        <div className="features-container">
          <div className="feature-column">
            <h3>Gestão de Portfólio</h3>
            <ul>
              <li>Multi-projetos com visão consolidada</li>
              <li>Allocation de recursos cross-projeto</li>
              <li>Comparativo de performance entre obras</li>
              <li>Budget top-down e bottom-up</li>
            </ul>
          </div>
          <div className="feature-column">
            <h3>Integrações Corporativas</h3>
            <ul>
              <li>API REST aberta com documentação</li>
              <li>Webhooks para eventos customizados</li>
              <li>Conectores ERP (TOTVS, SAP, Protheus)</li>
              <li>Exportação para Power BI / Tableau</li>
            </ul>
          </div>
          <div className="feature-column">
            <h3>Field Operations</h3>
            <ul>
              <li>App mobile offline-first</li>
              <li>Diário de obra digital</li>
              <li>Checklists de qualidade automatizados</li>
              <li>Fotos georreferenciadas</li>
            </ul>
          </div>
          <div className="feature-column">
            <h3>Stakeholder Management</h3>
            <ul>
              <li>Portal externo para fornecedores</li>
              <li>Comunicação automatizada</li>
              <li>Controle de documentação</li>
              <li>Gestão de pendências</li>
            </ul>
          </div>
        </div>
      </section>

      <section className="corporate-cases" id="cases">
        <div className="section-header">
          <h2>Cases de Sucesso</h2>
          <p>Resultados reais de clientes que transformaram suas operações</p>
        </div>
        <div className="cases-grid">
          <article className="case-card">
            <div className="case-badge">Construtora</div>
            <h3>Construtora Atlas</h3>
            <p className="case-description">
              Implementação em 12 obras simultâneas com 180+ usuários. Redução de 32% no ciclo de aprovação financeira 
              e visibilidade D+1 em todos os indicadores operacionais.
            </p>
            <div className="case-metrics">
              <div className="case-metric">
                <span className="case-metric__value">32%</span>
                <span className="case-metric__label">redução em aprovações</span>
              </div>
              <div className="case-metric">
                <span className="case-metric__value">D+1</span>
                <span className="case-metric__label">visibilidade de dados</span>
              </div>
            </div>
          </article>
          <article className="case-card">
            <div className="case-badge">Incorporadora</div>
            <h3>Grupo MRV</h3>
            <p className="case-description">
              Padronização de processos em 45 empreendimentos. Integração com ERP TOTVS para conciliação automática 
              de custos. Economia de R$ 4.2M anuais em retrabalho.
            </p>
            <div className="case-metrics">
              <div className="case-metric">
                <span className="case-metric__value">R$ 4.2M</span>
                <span className="case-metric__label">economia anual</span>
              </div>
              <div className="case-metric">
                <span className="case-metric__value">45</span>
                <span className="case-metric__label">empreendimentos</span>
              </div>
            </div>
          </article>
          <article className="case-card">
            <div className="case-badge">Infraestrutura</div>
            <h3>Construtora Odebrecht</h3>
            <p className="case-description">
              Gestão de projetos de infraestrutura complexos. Implementação de BIM 4D/5D integrado ao planejamento. 
              Redução de 28% em atrasos de entrega.
            </p>
            <div className="case-metrics">
              <div className="case-metric">
                <span className="case-metric__value">28%</span>
                <span className="case-metric__label">menos atrasos</span>
              </div>
              <div className="case-metric">
                <span className="case-metric__value">BIM</span>
                <span className="case-metric__label">4D/5D integrado</span>
              </div>
            </div>
          </article>
        </div>
      </section>

      <section className="corporate-security" id="seguranca">
        <div className="section-header">
          <h2>Segurança e Confiabilidade</h2>
          <p>Infraestrutura enterprise-class para proteger seus dados</p>
        </div>
        <div className="security-grid">
          <div className="security-item">
            <div className="security-icon">🛡️</div>
            <h3>ISO 27001</h3>
            <p>Certificação internacional de segurança da informação</p>
          </div>
          <div className="security-item">
            <div className="security-icon">🔐</div>
            <h3>LGPD Compliant</h3>
            <p>Conformidade total com legislação brasileira de proteção de dados</p>
          </div>
          <div className="security-item">
            <div className="security-icon">☁️</div>
            <h3>Multi-Region</h3>
            <p>Data centers no Brasil com redundância geográfica</p>
          </div>
          <div className="security-item">
            <div className="security-icon">🔄</div>
            <h3>Backup Automático</h3>
            <p>Backups diários com retenção de 90 dias</p>
          </div>
          <div className="security-item">
            <div className="security-icon">👥</div>
            <h3>SSO Enterprise</h3>
            <p>Integração com Azure AD, Okta e Google Workspace</p>
          </div>
          <div className="security-item">
            <div className="security-icon">📋</div>
            <h3>Audit Trail</h3>
            <p>Registro completo de todas as ações no sistema</p>
          </div>
        </div>
      </section>

      <section className="corporate-plans" id="planos">
        <div className="section-header">
          <h2>Planos Corporativos</h2>
          <p>Soluções escaláveis para cada estágio de maturidade</p>
        </div>
        <div className="plans-container">
          <article className="plan-card plan-card--business">
            <div className="plan-header">
              <span className="plan-badge">Business</span>
              <h3>Para médias empresas</h3>
              <div className="plan-price">
                <span className="price-value">R$ 4.990</span>
                <span className="price-period">/mês</span>
              </div>
              <p className="plan-description">Até 300 usuários • 10 obras simultâneas</p>
            </div>
            <ul className="plan-features">
              <li>✓ Todos os módulos core</li>
              <li>✓ Integrações ERP unidirecionais</li>
              <li>✓ Analytics avançados</li>
              <li>✓ Suporte 12x5 via chat/email</li>
              <li>✓ Treinamento onboarding</li>
              <li>✓ API REST (rate limit básico)</li>
            </ul>
            <a className="plan-cta" href="#contact">Solicitar proposta</a>
          </article>

          <article className="plan-card plan-card--enterprise">
            <div className="plan-badge plan-badge--featured">Enterprise</div>
            <div className="plan-header">
              <h3>Para grandes operações</h3>
              <div className="plan-price">
                <span className="price-value">Sob consulta</span>
              </div>
              <p className="plan-description">Usuários ilimitados • Obras ilimitadas</p>
            </div>
            <ul className="plan-features">
              <li>✓ Tudo do Business +</li>
              <li>✓ Integrações bidirecionais ERP</li>
              <li>✓ IA preditiva customizada</li>
              <li>✓ Suporte 24x7 dedicado</li>
              <li>✓ Customer Success Manager</li>
              <li>✓ API REST sem limites</li>
              <li>✓ Ambiente dedicado (single-tenant)</li>
              <li>✓ SLA 99.9% garantido</li>
              <li>✓ Consultoria de implementação</li>
            </ul>
            <a className="plan-cta plan-cta--primary" href="#contact">Falar com Enterprise</a>
          </article>

          <article className="plan-card plan-card--custom">
            <div className="plan-header">
              <span className="plan-badge">Custom</span>
              <h3>Solução sob medida</h3>
              <div className="plan-price">
                <span className="price-value">Sob consulta</span>
              </div>
              <p className="plan-description">Para necessidades específicas</p>
            </div>
            <ul className="plan-features">
              <li>✓ Desenvolvimento customizado</li>
              <li>✓ White-label</li>
              <li>✓ Deploy on-premise</li>
              <li>✓ Contratos governamentais</li>
              <li>✓ SLA customizado</li>
            </ul>
            <a className="plan-cta" href="#contact">Discutir requisitos</a>
          </article>
        </div>
      </section>

      <section className="corporate-contact" id="contact">
        <div className="contact-container">
          <div className="contact-info">
            <h2>Inicie a conversa</h2>
            <p>
              Nossa equipe de especialistas em construção civil está pronta para entender seus desafios 
              e apresentar uma solução personalizada.
            </p>
            <div className="contact-channels">
              <div className="contact-channel">
                <span className="channel-icon">📧</span>
                <div>
                  <strong>Email Comercial</strong>
                  <a href="mailto:comercial@arxis.com.br">comercial@arxis.com.br</a>
                </div>
              </div>
              <div className="contact-channel">
                <span className="channel-icon">📱</span>
                <div>
                  <strong>WhatsApp</strong>
                  <a href="https://wa.me/5511999999999">+55 (11) 99999-9999</a>
                </div>
              </div>
              <div className="contact-channel">
                <span className="channel-icon">🏢</span>
                <div>
                  <strong>Escritório</strong>
                  <span>São Paulo, SP - Brasil</span>
                </div>
              </div>
            </div>
          </div>

          <form className="contact-form" onSubmit={handleSubmit}>
            <h3>Solicitar contato comercial</h3>
            <div className="form-row">
              <div className="form-field">
                <label htmlFor="name">Nome completo *</label>
                <input
                  id="name"
                  name="name"
                  value={formState.name}
                  onChange={handleChange}
                  placeholder="Seu nome"
                  required
                />
              </div>
              <div className="form-field">
                <label htmlFor="role">Cargo *</label>
                <select
                  id="role"
                  name="role"
                  value={formState.role}
                  onChange={handleChange}
                  required
                >
                  <option value="">Selecione...</option>
                  <option value="diretor">Diretor</option>
                  <option value="gerente">Gerente de Projetos</option>
                  <option value="engenheiro">Engenheiro</option>
                  <option value="arquiteto">Arquiteto</option>
                  <option value="coo">COO</option>
                  <option value="ceo">CEO</option>
                  <option value="outro">Outro</option>
                </select>
              </div>
            </div>
            <div className="form-row">
              <div className="form-field">
                <label htmlFor="email">Email corporativo *</label>
                <input
                  id="email"
                  name="email"
                  type="email"
                  value={formState.email}
                  onChange={handleChange}
                  placeholder="seuemail@empresa.com"
                  required
                />
              </div>
              <div className="form-field">
                <label htmlFor="phone">Telefone</label>
                <input
                  id="phone"
                  name="phone"
                  value={formState.phone}
                  onChange={handleChange}
                  placeholder="+55 (11) 99999-9999"
                />
              </div>
            </div>
            <div className="form-field">
              <label htmlFor="company">Empresa *</label>
              <input
                id="company"
                name="company"
                value={formState.company}
                onChange={handleChange}
                placeholder="Nome da sua empresa"
                required
              />
            </div>
            <button type="submit" className="form-submit" disabled={isSubmitting}>
              {isSubmitting ? 'Enviando...' : 'Solicitar contato'}
            </button>
            {statusMessage ? (
              <p className={`form-status form-status--${statusType}`}>{statusMessage}</p>
            ) : null}
            <p className="form-legal">
              Ao enviar, você concorda com nossa Política de Privacidade. Seus dados serão utilizados 
              exclusivamente para fins comerciais.
            </p>
          </form>
        </div>
      </section>

      <footer className="corporate-footer">
        <div className="footer-content">
          <div className="footer-brand">
            <strong>Arxis Enterprise</strong>
            <p>Plataforma corporativa para gestão de construção civil</p>
          </div>
          <div className="footer-links">
            <div className="footer-column">
              <h4>Solução</h4>
              <a href="#recursos">Recursos</a>
              <a href="#cases">Cases</a>
              <a href="#seguranca">Segurança</a>
              <a href="#planos">Planos</a>
            </div>
            <div className="footer-column">
              <h4>Empresa</h4>
              <a href="#">Sobre nós</a>
              <a href="#">Carreiras</a>
              <a href="#">Blog</a>
              <a href="#">Contato</a>
            </div>
            <div className="footer-column">
              <h4>Legal</h4>
              <a href="#">Termos de Uso</a>
              <a href="#">Política de Privacidade</a>
              <a href="#">LGPD</a>
              <a href="#">SLA</a>
            </div>
          </div>
          <div className="footer-cta">
            <Link className="footer-login" to="/login">Área do cliente</Link>
            <a className="footer-demo" href="#contact">Falar com vendas</a>
          </div>
        </div>
        <div className="footer-bottom">
          <p>© 2026 Arxis. Todos os direitos reservados. CNPJ: 00.000.000/0001-00</p>
        </div>
      </footer>
    </div>
  );
};

export default CorporateLanding;
