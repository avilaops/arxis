use axum::{
    extract::State,
    response::{Html, IntoResponse, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

use crate::{error::ConsoleError, state::ConsoleState};

/// Team Management UI HTML
const TEAM_MANAGEMENT_HTML: &str = r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Team Management & RBAC - AVL Console</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif;
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1600px;
            margin: 0 auto;
            background: white;
            border-radius: 20px;
            padding: 40px;
            box-shadow: 0 20px 60px rgba(0,0,0,0.3);
        }
        h1 {
            color: #f5576c;
            margin-bottom: 10px;
            font-size: 36px;
        }
        .subtitle {
            color: #666;
            margin-bottom: 30px;
            font-size: 16px;
        }
        .tabs {
            display: flex;
            gap: 10px;
            margin-bottom: 30px;
            border-bottom: 2px solid #e9ecef;
        }
        .tab {
            padding: 15px 30px;
            background: none;
            border: none;
            border-bottom: 3px solid transparent;
            cursor: pointer;
            font-weight: 600;
            color: #6c757d;
            transition: all 0.2s;
            font-size: 15px;
        }
        .tab.active {
            color: #f5576c;
            border-bottom-color: #f5576c;
        }
        .tab:hover {
            color: #f5576c;
        }
        .tab-content {
            display: none;
        }
        .tab-content.active {
            display: block;
        }
        .action-bar {
            display: flex;
            justify-content: space-between;
            align-items: center;
            margin-bottom: 25px;
        }
        .search-box {
            flex: 1;
            max-width: 400px;
            position: relative;
        }
        .search-box input {
            width: 100%;
            padding: 12px 40px 12px 15px;
            border: 2px solid #dee2e6;
            border-radius: 10px;
            font-size: 14px;
        }
        .search-box input:focus {
            outline: none;
            border-color: #f5576c;
        }
        .search-icon {
            position: absolute;
            right: 15px;
            top: 50%;
            transform: translateY(-50%);
            color: #adb5bd;
        }
        .btn {
            padding: 12px 24px;
            border: none;
            border-radius: 10px;
            font-weight: 600;
            cursor: pointer;
            transition: all 0.2s;
            font-size: 14px;
        }
        .btn-primary {
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            color: white;
        }
        .btn-primary:hover {
            transform: translateY(-2px);
            box-shadow: 0 6px 20px rgba(245, 87, 108, 0.4);
        }
        .btn-secondary {
            background: white;
            color: #f5576c;
            border: 2px solid #f5576c;
        }
        .btn-secondary:hover {
            background: #fff5f7;
        }
        .team-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
            gap: 20px;
        }
        .team-card {
            background: white;
            border: 2px solid #e9ecef;
            border-radius: 16px;
            padding: 25px;
            transition: all 0.2s;
            cursor: pointer;
        }
        .team-card:hover {
            border-color: #f5576c;
            box-shadow: 0 8px 24px rgba(245, 87, 108, 0.15);
            transform: translateY(-4px);
        }
        .team-header {
            display: flex;
            align-items: center;
            gap: 15px;
            margin-bottom: 15px;
        }
        .team-icon {
            width: 50px;
            height: 50px;
            border-radius: 12px;
            background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 24px;
        }
        .team-info {
            flex: 1;
        }
        .team-name {
            font-weight: 600;
            font-size: 18px;
            color: #495057;
        }
        .team-members {
            font-size: 13px;
            color: #6c757d;
        }
        .team-description {
            color: #6c757d;
            font-size: 14px;
            margin-bottom: 15px;
            line-height: 1.5;
        }
        .team-roles {
            display: flex;
            flex-wrap: wrap;
            gap: 6px;
        }
        .role-badge {
            padding: 4px 10px;
            border-radius: 6px;
            font-size: 11px;
            font-weight: 600;
            background: #f8f9fa;
            color: #495057;
        }
        .users-table {
            width: 100%;
            border-collapse: collapse;
            background: white;
        }
        .users-table thead {
            background: #f8f9fa;
        }
        .users-table th {
            padding: 15px;
            text-align: left;
            font-size: 13px;
            font-weight: 600;
            color: #495057;
            text-transform: uppercase;
            letter-spacing: 0.5px;
        }
        .users-table td {
            padding: 15px;
            border-bottom: 1px solid #e9ecef;
            font-size: 14px;
        }
        .users-table tbody tr:hover {
            background: #f8f9fa;
        }
        .user-avatar {
            width: 40px;
            height: 40px;
            border-radius: 50%;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            display: flex;
            align-items: center;
            justify-content: center;
            color: white;
            font-weight: 600;
            font-size: 16px;
        }
        .user-info {
            display: flex;
            align-items: center;
            gap: 12px;
        }
        .user-details {
            display: flex;
            flex-direction: column;
        }
        .user-name {
            font-weight: 600;
            color: #495057;
        }
        .user-email {
            font-size: 12px;
            color: #6c757d;
        }
        .status-badge {
            padding: 6px 12px;
            border-radius: 8px;
            font-size: 12px;
            font-weight: 600;
        }
        .status-active {
            background: #d4edda;
            color: #155724;
        }
        .status-inactive {
            background: #f8d7da;
            color: #721c24;
        }
        .permissions-grid {
            display: grid;
            grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
            gap: 15px;
        }
        .permission-card {
            background: #f8f9fa;
            border: 2px solid #e9ecef;
            border-radius: 12px;
            padding: 20px;
        }
        .permission-card.enabled {
            border-color: #f5576c;
            background: #fff5f7;
        }
        .permission-title {
            font-weight: 600;
            color: #495057;
            margin-bottom: 8px;
            display: flex;
            align-items: center;
            gap: 10px;
        }
        .permission-desc {
            font-size: 13px;
            color: #6c757d;
            margin-bottom: 12px;
        }
        .toggle-switch {
            position: relative;
            width: 50px;
            height: 26px;
        }
        .toggle-switch input {
            opacity: 0;
            width: 0;
            height: 0;
        }
        .toggle-slider {
            position: absolute;
            cursor: pointer;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: #ccc;
            transition: 0.4s;
            border-radius: 26px;
        }
        .toggle-slider:before {
            position: absolute;
            content: "";
            height: 18px;
            width: 18px;
            left: 4px;
            bottom: 4px;
            background-color: white;
            transition: 0.4s;
            border-radius: 50%;
        }
        .toggle-switch input:checked + .toggle-slider {
            background-color: #f5576c;
        }
        .toggle-switch input:checked + .toggle-slider:before {
            transform: translateX(24px);
        }
        .modal {
            display: none;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0,0,0,0.6);
            align-items: center;
            justify-content: center;
            z-index: 1000;
        }
        .modal.active {
            display: flex;
        }
        .modal-content {
            background: white;
            border-radius: 20px;
            padding: 40px;
            max-width: 600px;
            width: 90%;
            max-height: 90vh;
            overflow-y: auto;
        }
        .modal-header {
            margin-bottom: 25px;
        }
        .modal-title {
            font-size: 24px;
            font-weight: 600;
            color: #495057;
        }
        .form-group {
            margin-bottom: 20px;
        }
        .form-group label {
            display: block;
            font-weight: 600;
            color: #495057;
            margin-bottom: 8px;
            font-size: 14px;
        }
        .form-group input,
        .form-group select,
        .form-group textarea {
            width: 100%;
            padding: 12px;
            border: 2px solid #dee2e6;
            border-radius: 8px;
            font-size: 14px;
        }
        .form-group input:focus,
        .form-group select:focus,
        .form-group textarea:focus {
            outline: none;
            border-color: #f5576c;
        }
        .form-actions {
            display: flex;
            gap: 10px;
            justify-content: flex-end;
            margin-top: 30px;
        }
        .audit-log {
            background: #f8f9fa;
            border-radius: 12px;
            padding: 20px;
        }
        .log-entry {
            padding: 15px;
            border-bottom: 1px solid #e9ecef;
            display: flex;
            align-items: start;
            gap: 15px;
        }
        .log-entry:last-child {
            border-bottom: none;
        }
        .log-icon {
            font-size: 24px;
        }
        .log-content {
            flex: 1;
        }
        .log-action {
            font-weight: 600;
            color: #495057;
            margin-bottom: 4px;
        }
        .log-details {
            font-size: 13px;
            color: #6c757d;
        }
        .log-time {
            font-size: 12px;
            color: #adb5bd;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>👥 Team Management & RBAC</h1>
        <p class="subtitle">Enterprise-grade access control and team collaboration</p>

        <div class="tabs">
            <button class="tab active" onclick="switchTab('teams')">Teams</button>
            <button class="tab" onclick="switchTab('users')">Users</button>
            <button class="tab" onclick="switchTab('roles')">Roles & Permissions</button>
            <button class="tab" onclick="switchTab('audit')">Audit Log</button>
        </div>

        <!-- Teams Tab -->
        <div id="teams-tab" class="tab-content active">
            <div class="action-bar">
                <div class="search-box">
                    <input type="text" placeholder="Search teams..." onkeyup="searchTeams(this.value)">
                    <span class="search-icon">🔍</span>
                </div>
                <button class="btn btn-primary" onclick="openModal('newTeam')">➕ New Team</button>
            </div>
            <div class="team-grid" id="teamsGrid"></div>
        </div>

        <!-- Users Tab -->
        <div id="users-tab" class="tab-content">
            <div class="action-bar">
                <div class="search-box">
                    <input type="text" placeholder="Search users..." onkeyup="searchUsers(this.value)">
                    <span class="search-icon">🔍</span>
                </div>
                <button class="btn btn-primary" onclick="openModal('newUser')">➕ Invite User</button>
            </div>
            <table class="users-table">
                <thead>
                    <tr>
                        <th>User</th>
                        <th>Role</th>
                        <th>Teams</th>
                        <th>Status</th>
                        <th>Last Active</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody id="usersTable"></tbody>
            </table>
        </div>

        <!-- Roles Tab -->
        <div id="roles-tab" class="tab-content">
            <div class="action-bar">
                <h2 style="color: #495057; margin: 0;">Configure Role Permissions</h2>
                <select id="roleSelect" onchange="loadPermissions(this.value)" style="padding: 10px; border-radius: 8px; border: 2px solid #dee2e6;">
                    <option value="admin">Admin</option>
                    <option value="developer">Developer</option>
                    <option value="viewer">Viewer</option>
                </select>
            </div>
            <div class="permissions-grid" id="permissionsGrid"></div>
        </div>

        <!-- Audit Log Tab -->
        <div id="audit-tab" class="tab-content">
            <div class="audit-log" id="auditLog"></div>
        </div>
    </div>

    <!-- Modals -->
    <div id="newTeamModal" class="modal">
        <div class="modal-content">
            <div class="modal-header">
                <h2 class="modal-title">Create New Team</h2>
            </div>
            <form onsubmit="createTeam(event)">
                <div class="form-group">
                    <label>Team Name</label>
                    <input type="text" name="name" required placeholder="Engineering, Design, Marketing...">
                </div>
                <div class="form-group">
                    <label>Description</label>
                    <textarea name="description" rows="3" placeholder="Brief description of the team's purpose"></textarea>
                </div>
                <div class="form-group">
                    <label>Default Role</label>
                    <select name="default_role">
                        <option value="developer">Developer</option>
                        <option value="viewer">Viewer</option>
                    </select>
                </div>
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" onclick="closeModal('newTeam')">Cancel</button>
                    <button type="submit" class="btn btn-primary">Create Team</button>
                </div>
            </form>
        </div>
    </div>

    <div id="newUserModal" class="modal">
        <div class="modal-content">
            <div class="modal-header">
                <h2 class="modal-title">Invite User</h2>
            </div>
            <form onsubmit="inviteUser(event)">
                <div class="form-group">
                    <label>Email</label>
                    <input type="email" name="email" required placeholder="user@company.com">
                </div>
                <div class="form-group">
                    <label>Role</label>
                    <select name="role">
                        <option value="developer">Developer</option>
                        <option value="viewer">Viewer</option>
                        <option value="admin">Admin</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>Teams</label>
                    <select name="teams" multiple style="height: 100px;">
                        <option value="team_1">Engineering</option>
                        <option value="team_2">Design</option>
                        <option value="team_3">Marketing</option>
                    </select>
                </div>
                <div class="form-actions">
                    <button type="button" class="btn btn-secondary" onclick="closeModal('newUser')">Cancel</button>
                    <button type="submit" class="btn btn-primary">Send Invitation</button>
                </div>
            </form>
        </div>
    </div>

    <script>
        let currentTab = 'teams';

        async function loadTeams() {
            try {
                const response = await fetch('/teams/list');
                const data = await response.json();
                renderTeams(data.teams);
            } catch (error) {
                console.error('Failed to load teams:', error);
            }
        }

        async function loadUsers() {
            try {
                const response = await fetch('/teams/users');
                const data = await response.json();
                renderUsers(data.users);
            } catch (error) {
                console.error('Failed to load users:', error);
            }
        }

        async function loadAuditLog() {
            try {
                const response = await fetch('/teams/audit');
                const data = await response.json();
                renderAuditLog(data.logs);
            } catch (error) {
                console.error('Failed to load audit log:', error);
            }
        }

        function renderTeams(teams) {
            const grid = document.getElementById('teamsGrid');
            grid.innerHTML = teams.map(team => `
                <div class="team-card" onclick="viewTeam('${team.id}')">
                    <div class="team-header">
                        <div class="team-icon">${team.icon}</div>
                        <div class="team-info">
                            <div class="team-name">${team.name}</div>
                            <div class="team-members">${team.member_count} members</div>
                        </div>
                    </div>
                    <div class="team-description">${team.description}</div>
                    <div class="team-roles">
                        ${team.roles.map(role => `<span class="role-badge">${role}</span>`).join('')}
                    </div>
                </div>
            `).join('');
        }

        function renderUsers(users) {
            const table = document.getElementById('usersTable');
            table.innerHTML = users.map(user => `
                <tr>
                    <td>
                        <div class="user-info">
                            <div class="user-avatar">${user.name.charAt(0)}</div>
                            <div class="user-details">
                                <div class="user-name">${user.name}</div>
                                <div class="user-email">${user.email}</div>
                            </div>
                        </div>
                    </td>
                    <td><span class="role-badge">${user.role}</span></td>
                    <td>${user.teams.join(', ')}</td>
                    <td><span class="status-badge status-${user.status}">${user.status}</span></td>
                    <td>${user.last_active}</td>
                    <td>
                        <button class="btn-secondary" style="padding: 6px 12px; font-size: 12px;" onclick="editUser('${user.id}')">Edit</button>
                    </td>
                </tr>
            `).join('');
        }

        function renderAuditLog(logs) {
            const logDiv = document.getElementById('auditLog');
            logDiv.innerHTML = logs.map(log => `
                <div class="log-entry">
                    <div class="log-icon">${log.icon}</div>
                    <div class="log-content">
                        <div class="log-action">${log.action}</div>
                        <div class="log-details">${log.details}</div>
                        <div class="log-time">${log.time}</div>
                    </div>
                </div>
            `).join('');
        }

        function loadPermissions(role) {
            const permissions = {
                admin: [
                    {name: 'Manage Users', desc: 'Create, edit, and delete users', enabled: true},
                    {name: 'Manage Teams', desc: 'Create and configure teams', enabled: true},
                    {name: 'View Billing', desc: 'Access billing and invoices', enabled: true},
                    {name: 'Manage Database', desc: 'Full database access', enabled: true},
                    {name: 'Manage Storage', desc: 'Full storage access', enabled: true},
                    {name: 'View Logs', desc: 'Access system logs', enabled: true},
                ],
                developer: [
                    {name: 'Manage Users', desc: 'Create, edit, and delete users', enabled: false},
                    {name: 'Manage Teams', desc: 'Create and configure teams', enabled: false},
                    {name: 'View Billing', desc: 'Access billing and invoices', enabled: false},
                    {name: 'Manage Database', desc: 'Full database access', enabled: true},
                    {name: 'Manage Storage', desc: 'Full storage access', enabled: true},
                    {name: 'View Logs', desc: 'Access system logs', enabled: true},
                ],
                viewer: [
                    {name: 'Manage Users', desc: 'Create, edit, and delete users', enabled: false},
                    {name: 'Manage Teams', desc: 'Create and configure teams', enabled: false},
                    {name: 'View Billing', desc: 'Access billing and invoices', enabled: false},
                    {name: 'Manage Database', desc: 'Full database access', enabled: false},
                    {name: 'Manage Storage', desc: 'Full storage access', enabled: false},
                    {name: 'View Logs', desc: 'Access system logs', enabled: true},
                ]
            };

            const grid = document.getElementById('permissionsGrid');
            grid.innerHTML = permissions[role].map((perm, idx) => `
                <div class="permission-card ${perm.enabled ? 'enabled' : ''}">
                    <div class="permission-title">
                        <span>${perm.name}</span>
                        <label class="toggle-switch" style="margin-left: auto;">
                            <input type="checkbox" ${perm.enabled ? 'checked' : ''} onchange="togglePermission('${role}', ${idx}, this.checked)">
                            <span class="toggle-slider"></span>
                        </label>
                    </div>
                    <div class="permission-desc">${perm.desc}</div>
                </div>
            `).join('');
        }

        function switchTab(tab) {
            currentTab = tab;
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
            event.target.classList.add('active');
            document.getElementById(`${tab}-tab`).classList.add('active');

            if (tab === 'teams') loadTeams();
            else if (tab === 'users') loadUsers();
            else if (tab === 'roles') loadPermissions('admin');
            else if (tab === 'audit') loadAuditLog();
        }

        function openModal(modal) {
            document.getElementById(`${modal}Modal`).classList.add('active');
        }

        function closeModal(modal) {
            document.getElementById(`${modal}Modal`).classList.remove('active');
        }

        async function createTeam(e) {
            e.preventDefault();
            const formData = new FormData(e.target);
            const data = Object.fromEntries(formData);

            try {
                await fetch('/teams/create', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify(data)
                });
                closeModal('newTeam');
                loadTeams();
            } catch (error) {
                console.error('Failed to create team:', error);
            }
        }

        async function inviteUser(e) {
            e.preventDefault();
            const formData = new FormData(e.target);
            const data = Object.fromEntries(formData);

            try {
                await fetch('/teams/invite', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify(data)
                });
                closeModal('newUser');
                loadUsers();
            } catch (error) {
                console.error('Failed to invite user:', error);
            }
        }

        function searchTeams(query) {
            // Implement search
        }

        function searchUsers(query) {
            // Implement search
        }

        function viewTeam(id) {
            // Implement team details view
        }

        function editUser(id) {
            // Implement user edit
        }

        function togglePermission(role, idx, enabled) {
            // Implement permission toggle
            console.log(`Toggle ${role} permission ${idx}: ${enabled}`);
        }

        // Initialize
        loadTeams();
    </script>
</body>
</html>"#;

/// Role enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Developer,
    Viewer,
    Custom(String),
}

impl Role {
    pub fn default_permissions(&self) -> HashSet<Permission> {
        match self {
            Role::Admin => HashSet::from([
                Permission::ManageUsers,
                Permission::ManageTeams,
                Permission::ViewBilling,
                Permission::ManageDatabase,
                Permission::ManageStorage,
                Permission::ViewLogs,
                Permission::ManageSettings,
            ]),
            Role::Developer => HashSet::from([
                Permission::ManageDatabase,
                Permission::ManageStorage,
                Permission::ViewLogs,
            ]),
            Role::Viewer => HashSet::from([Permission::ViewLogs]),
            Role::Custom(_) => HashSet::new(),
        }
    }
}

/// Permission enum
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    ManageUsers,
    ManageTeams,
    ViewBilling,
    ManageDatabase,
    ManageStorage,
    ViewLogs,
    ManageSettings,
}

/// Team structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub member_count: usize,
    pub roles: Vec<String>,
    pub created_at: String,
}

/// User structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub teams: Vec<String>,
    pub status: String,
    pub last_active: String,
    pub permissions: HashSet<Permission>,
}

/// Audit log entry
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuditLogEntry {
    pub id: String,
    pub action: String,
    pub details: String,
    pub icon: String,
    pub user_id: String,
    pub time: String,
}

/// Teams list response
#[derive(Debug, Serialize)]
struct TeamsResponse {
    teams: Vec<Team>,
}

/// Users list response
#[derive(Debug, Serialize)]
struct UsersResponse {
    users: Vec<User>,
}

/// Audit log response
#[derive(Debug, Serialize)]
struct AuditLogResponse {
    logs: Vec<AuditLogEntry>,
}

/// Team management UI
async fn team_management_ui() -> impl IntoResponse {
    Html(TEAM_MANAGEMENT_HTML)
}

/// List all teams
async fn list_teams(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<TeamsResponse>, ConsoleError> {
    // Mock data - in production, load from database
    let teams = vec![
        Team {
            id: "team_1".to_string(),
            name: "Engineering".to_string(),
            description: "Core development team building AVL Platform features".to_string(),
            icon: "⚙️".to_string(),
            member_count: 12,
            roles: vec!["Admin".to_string(), "Developer".to_string()],
            created_at: "2024-01-15".to_string(),
        },
        Team {
            id: "team_2".to_string(),
            name: "Design".to_string(),
            description: "UI/UX designers crafting beautiful experiences".to_string(),
            icon: "🎨".to_string(),
            member_count: 5,
            roles: vec!["Developer".to_string(), "Viewer".to_string()],
            created_at: "2024-02-01".to_string(),
        },
        Team {
            id: "team_3".to_string(),
            name: "Marketing".to_string(),
            description: "Growth and customer success team".to_string(),
            icon: "📊".to_string(),
            member_count: 8,
            roles: vec!["Viewer".to_string()],
            created_at: "2024-02-15".to_string(),
        },
    ];

    Ok(Json(TeamsResponse { teams }))
}

/// List all users
async fn list_users(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<UsersResponse>, ConsoleError> {
    // Mock data - in production, load from database
    let users = vec![
        User {
            id: "user_1".to_string(),
            name: "Alice Johnson".to_string(),
            email: "alice@company.com".to_string(),
            role: Role::Admin,
            teams: vec!["Engineering".to_string()],
            status: "active".to_string(),
            last_active: "2 minutes ago".to_string(),
            permissions: Role::Admin.default_permissions(),
        },
        User {
            id: "user_2".to_string(),
            name: "Bob Smith".to_string(),
            email: "bob@company.com".to_string(),
            role: Role::Developer,
            teams: vec!["Engineering".to_string(), "Design".to_string()],
            status: "active".to_string(),
            last_active: "1 hour ago".to_string(),
            permissions: Role::Developer.default_permissions(),
        },
        User {
            id: "user_3".to_string(),
            name: "Carol White".to_string(),
            email: "carol@company.com".to_string(),
            role: Role::Viewer,
            teams: vec!["Marketing".to_string()],
            status: "active".to_string(),
            last_active: "3 hours ago".to_string(),
            permissions: Role::Viewer.default_permissions(),
        },
    ];

    Ok(Json(UsersResponse { users }))
}

/// Get audit log
async fn get_audit_log(
    State(_state): State<Arc<ConsoleState>>,
) -> Result<Json<AuditLogResponse>, ConsoleError> {
    // Mock data - in production, load from database
    let logs = vec![
        AuditLogEntry {
            id: "log_1".to_string(),
            action: "User Invited".to_string(),
            details: "alice@company.com invited bob@company.com to Engineering team".to_string(),
            icon: "✉️".to_string(),
            user_id: "user_1".to_string(),
            time: "2 hours ago".to_string(),
        },
        AuditLogEntry {
            id: "log_2".to_string(),
            action: "Permission Changed".to_string(),
            details: "bob@company.com role changed from Viewer to Developer".to_string(),
            icon: "🔐".to_string(),
            user_id: "user_1".to_string(),
            time: "5 hours ago".to_string(),
        },
        AuditLogEntry {
            id: "log_3".to_string(),
            action: "Team Created".to_string(),
            details: "New team 'Design' created with 5 members".to_string(),
            icon: "🎨".to_string(),
            user_id: "user_1".to_string(),
            time: "1 day ago".to_string(),
        },
    ];

    Ok(Json(AuditLogResponse { logs }))
}

/// Create a new team
async fn create_team(
    State(_state): State<Arc<ConsoleState>>,
    Json(payload): Json<Team>,
) -> Result<Json<Team>, ConsoleError> {
    // In production, save to database
    Ok(Json(payload))
}

/// Invite a user
async fn invite_user(
    State(_state): State<Arc<ConsoleState>>,
    Json(payload): Json<User>,
) -> Result<Json<User>, ConsoleError> {
    // In production, send invitation email and save to database
    Ok(Json(payload))
}

/// Check if user has permission
pub fn has_permission(user: &User, permission: &Permission) -> bool {
    user.permissions.contains(permission)
}

/// Create router for team management
pub fn router(state: Arc<ConsoleState>) -> Router {
    Router::new()
        .route("/", get(team_management_ui))
        .route("/list", get(list_teams))
        .route("/users", get(list_users))
        .route("/audit", get(get_audit_log))
        .route("/create", post(create_team))
        .route("/invite", post(invite_user))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_permissions() {
        let perms = Role::Admin.default_permissions();
        assert!(perms.contains(&Permission::ManageUsers));
        assert!(perms.contains(&Permission::ManageDatabase));
    }

    #[test]
    fn test_developer_permissions() {
        let perms = Role::Developer.default_permissions();
        assert!(!perms.contains(&Permission::ManageUsers));
        assert!(perms.contains(&Permission::ManageDatabase));
    }

    #[test]
    fn test_viewer_permissions() {
        let perms = Role::Viewer.default_permissions();
        assert!(!perms.contains(&Permission::ManageUsers));
        assert!(perms.contains(&Permission::ViewLogs));
    }
}
