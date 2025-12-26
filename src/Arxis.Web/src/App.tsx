import { ThemeProvider, CssBaseline } from '@mui/material'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { AuthProvider } from './context/AuthContext'
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
import { activityModules } from './config/navigation'

function App() {
  const modulesWithCustomPages: Record<string, JSX.Element> = {
    projects: <Projects />,
    tasks: <Tasks />,
    issues: <Issues />,
    marketplace: <Marketplace />,
  }

  return (
    <ThemeProvider theme={arxisTheme}>
      <CssBaseline />
      <AuthProvider>
        <BrowserRouter>
          <Routes>
            <Route path="/login" element={<Login />} />

            <Route
              path="/"
              element={
                <ProtectedRoute>
                  <Layout />
                </ProtectedRoute>
              }
            >
              <Route index element={<DashboardNew />} />
              {activityModules
                .filter((module) => module.path !== '/')
                .map((module) => {
                  const relativePath = module.path.startsWith('/') ? module.path.slice(1) : module.path
                  const element = modulesWithCustomPages[module.id] ?? <ModuleLanding module={module} />

                  return <Route key={module.id} path={relativePath} element={element} />
                })}
            </Route>

            <Route path="*" element={<Navigate to="/" replace />} />
          </Routes>
        </BrowserRouter>
      </AuthProvider>
    </ThemeProvider>
  )
}

export default App
