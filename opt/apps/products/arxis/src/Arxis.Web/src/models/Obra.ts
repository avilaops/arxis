// ============== ENUMS ==============

export enum OpportunityStage {
  NovoLead = 'NovoLead',
  ContatoFeito = 'ContatoFeito',
  QualificacaoTecnica = 'QualificacaoTecnica',
  DiagnosticoVisita = 'DiagnosticoVisita',
  DemoNoProjeto = 'DemoNoProjeto',
  PropostaEnviada = 'PropostaEnviada',
  NegociacaoJuridico = 'NegociacaoJuridico',
  GanhoOnboarding = 'GanhoOnboarding',
  PerdidoSemTiming = 'PerdidoSemTiming'
}

export enum WorkType {
  Residencial = 'Residencial',
  Comercial = 'Comercial',
  Industrial = 'Industrial',
  Infraestrutura = 'Infraestrutura',
  Reforma = 'Reforma',
  Incorporacao = 'Incorporacao'
}

export enum WorkPhase {
  Orcamento = 'Orcamento',
  PreConstrucao = 'PreConstrucao',
  Mobilizacao = 'Mobilizacao',
  Execucao = 'Execucao',
  Acabamento = 'Acabamento',
  Entrega = 'Entrega',
  PosObra = 'PosObra'
}

export enum ContractModel {
  Empreitada = 'Empreitada',
  EmpreitadaGlobal = 'EmpreitadaGlobal',
  PrecoUnitario = 'PrecoUnitario',
  Administracao = 'Administracao',
  TurnKey = 'TurnKey',
  EPC = 'EPC'
}

export enum PainPoint {
  Atraso = 'Atraso',
  Retrabalho = 'Retrabalho',
  CustoAcima = 'CustoAcima',
  Medicao = 'Medicao',
  RDO = 'RDO',
  Comunicacao = 'Comunicacao',
  Documentos = 'Documentos',
  Fiscalizacao = 'Fiscalizacao',
  Qualidade = 'Qualidade',
  Seguranca = 'Seguranca',
  Fornecedores = 'Fornecedores',
  Maoobra = 'Maoobra'
}

export enum LeadOrigin {
  Portal = 'Portal',
  Trafego = 'Trafego',
  Indicacao = 'Indicacao',
  Outbound = 'Outbound',
  Evento = 'Evento',
  LinkedIn = 'LinkedIn',
  WebSite = 'WebSite',
  Instagram = 'Instagram'
}

export enum LeadTemperature {
  Frio = 'Frio',
  Morno = 'Morno',
  Quente = 'Quente'
}

export enum ProposalStatus {
  Rascunho = 'Rascunho',
  EmRevisao = 'EmRevisao',
  Enviada = 'Enviada',
  Visualizada = 'Visualizada',
  Negociando = 'Negociando',
  Aceita = 'Aceita',
  Rejeitada = 'Rejeitada',
  Expirada = 'Expirada'
}

// ============== INTERFACES ==============

export interface ObraQualificationDto {
  id: string;
  leadId?: string;
  opportunityId?: string;

  // Dados técnicos da obra
  obraNome?: string;
  tipoObra: WorkType;
  faseAtual: WorkPhase;

  // Complexidade
  metragemM2?: number;
  numeroPavimentos?: number;
  disciplinas: string[];
  equipeSize?: number;
  localizacaoCidade?: string;

  // Modelo de contratação
  modeloContratacao: ContractModel;
  orcamentoTotal?: number;
  prazoInicio?: string;
  prazoTermino?: string;

  // Dor principal
  dorPrincipal: PainPoint;
  dorDetalhes?: string;
  custoMensalDoProblema?: number;

  // Ferramentas atuais
  ferramentasAtuais: string[];
  processoAtualRDO?: string;
  processoAtualMedicao?: string;
  processoAtualCronograma?: string;

  // Stakeholders
  dono?: string;
  engenheiro?: string;
  residente?: string;
  planejador?: string;
  financeiro?: string;

  // Decisão
  decisor?: string;
  aprovadorFinanceiro?: string;
  prazoDecisao?: string;
  motivoPrazo?: string;

  // Budget e Fit
  budgetRange?: string;
  criterioSucesso?: string;

  // Fit Score (calculado)
  fitScore: number;
  fitReason?: string;
  cadenciaSugerida?: string;

  // Descoberta adicional
  eventoRecente?: string;
  concorrenteAtual?: string;
  objecoesAntecipar?: string;

  // Notas
  notasTecnicas?: string;
  notasComerciais?: string;

  dataPreenchimento?: string;
  createdAt: string;
}

export interface PropostaTecnicaDto {
  id: string;
  versao: string;
  titulo: string;
  status: ProposalStatus;
  opportunityId: string;

  // Módulos Arxis incluídos
  moduloRDO: boolean;
  moduloMedicoes: boolean;
  moduloCronograma4D: boolean;
  moduloDocumentos: boolean;
  moduloChecklists: boolean;
  moduloFotos: boolean;
  moduloResponsabilidades: boolean;
  moduloQualidade: boolean;
  moduloSeguranca: boolean;

  escopoDetalhado?: string;
  foraDeEscopo?: string;

  // Implantação
  planoImplantacao?: string;
  diasImplantacao: number;
  marcosImplantacao?: string;

  // Preço
  valorMensalidade: number;
  valorSetup?: number;
  usuariosIncluidos?: number;
  condicoesComerciais?: string;
  validadeAte?: string;

  // Anexos e cases
  anexos?: string;
  caseSimilar?: string;
  depoimentoCliente?: string;

  // Tracking
  dataEnvio?: string;
  dataVisualizacao?: string;
  dataUltimoFollowup?: string;

  fileUrl?: string;
  fileName?: string;

  createdAt: string;
}

export interface SalesInboxItemDto {
  id: string;
  tipo: 'lead' | 'opportunity';
  nome: string;
  empresa?: string;
  status: string;
  temperatura: LeadTemperature;

  proximoPasso?: string;
  proximoPassoEm?: string;

  diasParado: number;
  atrasado: boolean;
  venceHoje: boolean;

  valorEstimado?: number;
  fitScore: number;

  prioridade: 'alta' | 'media' | 'baixa';
  ultimaAtividade?: string;
  ultimaAtividadeEm?: string;
}

export interface SalesMetricsDto {
  // Speed-to-lead
  speedToLeadMinutosMedia: number;
  speedToLeadUltimas24h: number;

  // Deals sem próximo passo
  dealsTotal: number;
  dealsSemProximoPasso: number;
  percentualSemProximoPasso: number;

  // Dias parado por etapa
  diasParadoPorEtapa: Record<string, number>;

  // Win rate por origem
  winRatePorOrigem: Record<string, WinRateDto>;

  // Motivos de perda
  motivosPerdaContagem: Record<string, number>;

  // Pipeline health
  dealsAtrasados: number;
  dealsVenceHoje: number;
  dealsQuentes: number;
  valorTotalPipeline: number;
}

export interface WinRateDto {
  totalDeals: number;
  dealsGanhos: number;
  dealsPerdidos: number;
  winRate: number;
  valorMedioGanho: number;
}

export interface MotivoPerdaDto {
  id: string;
  opportunityId: string;
  categoriaPrincipal: string;
  detalhes?: string;
  concorrente?: string;
  precoOfertadoConcorrente?: number;

  foiPreco: boolean;
  foiTiming: boolean;
  foiSemFit: boolean;
  foiConcorrente: boolean;
  foiSemBudget: boolean;
  foiComplejidade: boolean;

  licoesAprendidas?: string;
  podeRetomar: boolean;
  dataRetomada?: string;

  createdAt: string;
}

export interface CadenceExecutionDto {
  id: string;
  leadId: string;
  cadenceId: string;
  cadenceName: string;

  dataInicio: string;
  dataFim?: string;
  ativa: boolean;
  status: string;

  proximoStepNumber?: number;
  proximaExecucaoEm?: string;

  logs: CadenceExecutionLogDto[];
}

export interface CadenceExecutionLogDto {
  id: string;
  stepNumber: number;
  programadoPara: string;
  executadoEm?: string;
  resultado?: string;
  notas?: string;
}

// ============== LABELS E HELPERS ==============

export const OpportunityStageLabels: Record<OpportunityStage, string> = {
  [OpportunityStage.NovoLead]: 'Novo Lead',
  [OpportunityStage.ContatoFeito]: 'Contato Feito',
  [OpportunityStage.QualificacaoTecnica]: 'Qualificação Técnica',
  [OpportunityStage.DiagnosticoVisita]: 'Diagnóstico / Visita',
  [OpportunityStage.DemoNoProjeto]: 'Demo no Projeto',
  [OpportunityStage.PropostaEnviada]: 'Proposta Enviada',
  [OpportunityStage.NegociacaoJuridico]: 'Negociação / Jurídico',
  [OpportunityStage.GanhoOnboarding]: 'Ganho (Onboarding)',
  [OpportunityStage.PerdidoSemTiming]: 'Perdido / Sem Timing'
};

export const WorkTypeLabels: Record<WorkType, string> = {
  [WorkType.Residencial]: 'Residencial',
  [WorkType.Comercial]: 'Comercial',
  [WorkType.Industrial]: 'Industrial',
  [WorkType.Infraestrutura]: 'Infraestrutura',
  [WorkType.Reforma]: 'Reforma',
  [WorkType.Incorporacao]: 'Incorporação'
};

export const WorkPhaseLabels: Record<WorkPhase, string> = {
  [WorkPhase.Orcamento]: 'Orçamento',
  [WorkPhase.PreConstrucao]: 'Pré-Construção',
  [WorkPhase.Mobilizacao]: 'Mobilização',
  [WorkPhase.Execucao]: 'Execução',
  [WorkPhase.Acabamento]: 'Acabamento',
  [WorkPhase.Entrega]: 'Entrega',
  [WorkPhase.PosObra]: 'Pós-Obra'
};

export const PainPointLabels: Record<PainPoint, string> = {
  [PainPoint.Atraso]: 'Atraso',
  [PainPoint.Retrabalho]: 'Retrabalho',
  [PainPoint.CustoAcima]: 'Custo Acima',
  [PainPoint.Medicao]: 'Medição',
  [PainPoint.RDO]: 'RDO',
  [PainPoint.Comunicacao]: 'Comunicação',
  [PainPoint.Documentos]: 'Documentos',
  [PainPoint.Fiscalizacao]: 'Fiscalização',
  [PainPoint.Qualidade]: 'Qualidade',
  [PainPoint.Seguranca]: 'Segurança',
  [PainPoint.Fornecedores]: 'Fornecedores',
  [PainPoint.Maoobra]: 'Mão de Obra'
};

export const LeadOriginLabels: Record<LeadOrigin, string> = {
  [LeadOrigin.Portal]: 'Portal',
  [LeadOrigin.Trafego]: 'Tráfego',
  [LeadOrigin.Indicacao]: 'Indicação',
  [LeadOrigin.Outbound]: 'Outbound',
  [LeadOrigin.Evento]: 'Evento',
  [LeadOrigin.LinkedIn]: 'LinkedIn',
  [LeadOrigin.WebSite]: 'Website',
  [LeadOrigin.Instagram]: 'Instagram'
};

export const LeadTemperatureLabels: Record<LeadTemperature, string> = {
  [LeadTemperature.Frio]: 'Frio',
  [LeadTemperature.Morno]: 'Morno',
  [LeadTemperature.Quente]: 'Quente'
};

export const LeadTemperatureColors: Record<LeadTemperature, string> = {
  [LeadTemperature.Frio]: '#4444ff',
  [LeadTemperature.Morno]: '#ffaa00',
  [LeadTemperature.Quente]: '#ff4444'
};

export const OpportunityStageColors: Record<OpportunityStage, string> = {
  [OpportunityStage.NovoLead]: '#9e9e9e',
  [OpportunityStage.ContatoFeito]: '#2196f3',
  [OpportunityStage.QualificacaoTecnica]: '#03a9f4',
  [OpportunityStage.DiagnosticoVisita]: '#00bcd4',
  [OpportunityStage.DemoNoProjeto]: '#009688',
  [OpportunityStage.PropostaEnviada]: '#ff9800',
  [OpportunityStage.NegociacaoJuridico]: '#ff5722',
  [OpportunityStage.GanhoOnboarding]: '#4caf50',
  [OpportunityStage.PerdidoSemTiming]: '#f44336'
};
