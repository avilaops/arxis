import { useEffect } from 'react'
import { ThemeProvider, CssBaseline } from '@mui/material'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { AuthProvider, useAuth } from './context/AuthContext'
import { arxisTheme } from './theme/theme'
import { Layout } from './components/Layout/Layout'
import { ProtectedRoute } from './components/ProtectedRoute'
import { Login } from './pages/Login'
import DashboardNew from './pages/DashboardNew'
import { Projects } from './pages/Projects'
import { Tasks } from './pages/Tasks'
import { Issues } from './pages/Issues'
import { ModuleLanding } from './pages/ModuleLanding'
import { Marketplace } from './pages/Marketplace'
import { Costs } from './pages/Costs'
import { Analytics } from './pages/Analytics'
import { Integrations } from './pages/Integrations'
import { Field } from './pages/Field'
import { Stakeholders } from './pages/Stakeholders'
import { MarketingLanding } from './pages/MarketingLanding'
import CorporateLanding from './pages/CorporateLanding'
import UserManagement from './pages/UserManagement'
import SalesDashboard from './pages/SalesDashboard'
import SalesDashboardObra from './pages/SalesDashboardObra'
import LeadsList from './pages/LeadsList'
import { CrmDashboard } from './components/crm/CrmDashboard'
import { CrmNotifications } from './components/crm/CrmNotifications'
import { signalrService } from './services/signalrService'
import { activityModules } from './config/navigation'
import Viewer from './pages/Viewer'

// SignalR Connection Manager Component
function SignalRConnectionManager() {
  const { user } = useAuth()

  useEffect(() => {
    if (user?.token) {
      // Conectar ao SignalR
      signalrService.connect(user.token).then(() => {
        // Entrar em canais relevantes baseado no role do usuário
        signalrService.joinChannel('leads_channel')
        
        if (user.role === 'admin' || user.role === 'sales_manager') {
          signalrService.joinChannel('admin_channel')
        }
      })

      // Cleanup ao desmontar
      return () => {
        signalrService.disconnect()
      }
    }
  }, [user])

  return null
}


function App() {
  const modulesWithCustomPages: Record<string, JSX.Element> = {
    projects: <Projects />,
    tasks: <Tasks />,
    viewer: <Viewer />,
    issues: <Issues />,
    marketplace: <Marketplace />,
    costs: <Costs />,
    analytics: <Analytics />,
    integrations: <Integrations />,
    field: <Field />,
    stakeholders: <Stakeholders />,
    crm: <SalesDashboardObra />,
  }

  const crmSubRoutes: Record<string, JSX.Element> = {
    'crm/leads': <LeadsList />,
    'crm/analytics': <CrmDashboard />,
    // Adicionar outras sub-rotas do CRM aqui
  }

  return (
    <ThemeProvider theme={arxisTheme}>
      <CssBaseline />
      <AuthProvider>
        <SignalRConnectionManager />
        <BrowserRouter>
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route path="/landing" element={<MarketingLanding />} />
            <Route path="/corporate" element={<CorporateLanding />} />

            <Route
              path="/"
              element={
                <ProtectedRoute>
                  <>
                    <CrmNotifications />
                    <Layout />
                  </>
                </ProtectedRoute>
              }
            >
              <Route index element={<DashboardNew />} />
              <Route path="admin/users" element={<UserManagement />} />
              {activityModules
                .filter((module) => module.path !== '/')
                .map((module) => {
                  const relativePath = module.path.startsWith('/') ? module.path.slice(1) : module.path
                  const element = modulesWithCustomPages[module.id] ?? <ModuleLanding module={module} />

                  // Adicionar sub-rotas específicas do CRM
                  if (module.id === 'crm') {
                    return (
                      <Route key={module.id} path={relativePath}>
                        <Route index element={element} />
                        {Object.entries(crmSubRoutes).map(([subPath, subElement]) => (
                          <Route key={subPath} path={subPath.replace('crm/', '')} element={subElement} />
                        ))}
                      </Route>
                    )
                  }

                  return <Route key={module.id} path={relativePath} element={element} />
                })}
            </Route>

            <Route path="/viewer" element={<ProtectedRoute><Viewer /></ProtectedRoute>} />

            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </BrowserRouter>
      </AuthProvider>
    </ThemeProvider>
  )
}

export default App
