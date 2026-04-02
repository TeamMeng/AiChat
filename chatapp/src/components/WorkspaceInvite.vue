<template>
  <div class="page-container">
    <div class="header">
      <div class="header-left">
        <button class="btn-back" @click="$router.back()">
          <svg class="back-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div class="icon-box purple">
          <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
          </svg>
        </div>
        <div class="header-text">
          <h3 class="title">Workspace Invitations</h3>
          <p class="subtitle">Manage and share invitations</p>
        </div>
      </div>
      <button class="btn-create" @click="showCreateDialog = true">
        <svg class="btn-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
        </svg>
        Create Invitation
      </button>
    </div>

    <!-- Create Invitation Dialog -->
    <div v-if="showCreateDialog" class="modal-overlay" @click.self="closeCreateDialog">
      <div class="modal">
        <div class="modal-header">
          <div class="modal-title-group">
            <div class="icon-box green">
              <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
              </svg>
            </div>
            <h4 class="modal-title">Create Invitation</h4>
          </div>
          <button class="btn-close" @click="closeCreateDialog">
            <svg class="close-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>
        <div class="form-group">
          <label class="label">Expires in (days)</label>
          <input
            v-model.number="newInvite.expires_in_days"
            type="number"
            placeholder="Leave empty for no expiration"
            class="input"
          />
        </div>
        <div class="form-group">
          <label class="label">Max uses</label>
          <input
            v-model.number="newInvite.max_uses"
            type="number"
            placeholder="Leave empty for unlimited"
            class="input"
          />
        </div>
        <div class="modal-actions">
          <button class="btn-cancel" @click="closeCreateDialog">Cancel</button>
          <button class="btn-confirm" @click="createInvitation">Create</button>
        </div>
      </div>
    </div>

    <!-- Invitations List -->
    <div class="invitations-list">
      <div v-if="loading" class="loading">
        <div class="spinner"></div>
      </div>
      <div v-else-if="invitations.length === 0" class="empty">
        <svg class="empty-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 10h18M7 15h1m4 0h1m-7 4h12a3 3 0 003-3V8a3 3 0 00-3-3H6a3 3 0 00-3 3v8a3 3 0 003 3z" />
        </svg>
        <p class="empty-text">No invitations yet. Create one to invite others!</p>
      </div>
      <div v-else class="invitations">
        <div
          v-for="invitation in invitations"
          :key="invitation.id"
          class="invitation-card"
          :class="{ inactive: !invitation.is_active }"
        >
          <div class="invitation-header">
            <span class="label-code">Code:</span>
            <span class="code">{{ invitation.invite_code }}</span>
            <button class="btn-copy" @click="copyCode(invitation.invite_code)">
              <svg class="copy-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
              Copy
            </button>
          </div>
          <div class="invitation-details">
            <div class="detail-item">
              <svg class="detail-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <span v-if="invitation.expires_at">{{ formatDate(invitation.expires_at) }}</span>
              <span v-else class="text-muted">No expiration</span>
            </div>
            <div class="detail-item">
              <svg class="detail-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
              <span v-if="invitation.max_uses">{{ invitation.used_count }}/{{ invitation.max_uses }}</span>
              <span v-else>{{ invitation.used_count }} (unlimited)</span>
            </div>
            <span class="badge" :class="invitation.is_active ? 'active' : 'inactive'">
              {{ invitation.is_active ? 'Active' : 'Inactive' }}
            </span>
          </div>
          <button
            v-if="invitation.is_active"
            class="btn-deactivate"
            @click="deactivateInvitation(invitation.id)"
          >
            <svg class="deactivate-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18.364 18.364A9 9 0 005.636 5.636m12.728 12.728A9 9 0 015.636 5.636m12.728 12.728L5.636 5.636" />
            </svg>
            Deactivate
          </button>
        </div>
      </div>
    </div>

    <!-- Join Workspace Section -->
    <div class="join-section">
      <div class="join-header">
        <div class="icon-box blue">
          <svg class="icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
          </svg>
        </div>
        <div class="join-text">
          <h4 class="join-title">Join a Workspace</h4>
          <p class="join-subtitle">Enter an invitation code to join</p>
        </div>
      </div>
      <div class="join-form">
        <input
          v-model="joinCode"
          type="text"
          placeholder="Enter invitation code"
          @keyup.enter="joinWorkspace"
          class="join-input"
        />
        <button class="btn-join" @click="joinWorkspace">
          <svg class="join-icon" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6" />
          </svg>
          Join
        </button>
      </div>
    </div>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  name: 'WorkspaceInvite',
  data() {
    return {
      invitations: [],
      loading: false,
      showCreateDialog: false,
      newInvite: {
        expires_in_days: null,
        max_uses: null
      },
      joinCode: ''
    };
  },
  mounted() {
    this.loadInvitations();
  },
  methods: {
    async loadInvitations() {
      this.loading = true;
      try {
        const token = localStorage.getItem('token');
        const response = await axios.get('http://localhost:6688/api/workspaces/invitations', {
          headers: { Authorization: `Bearer ${token}` }
        });
        this.invitations = response.data;
      } catch (error) {
        console.error('Failed to load invitations:', error);
      } finally {
        this.loading = false;
      }
    },
    async createInvitation() {
      try {
        const token = localStorage.getItem('token');
        const payload = {};
        if (this.newInvite.expires_in_days) {
          payload.expires_in_days = this.newInvite.expires_in_days;
        }
        if (this.newInvite.max_uses) {
          payload.max_uses = this.newInvite.max_uses;
        }

        await axios.post('http://localhost:6688/api/workspaces/invitations', payload, {
          headers: { Authorization: `Bearer ${token}` }
        });

        this.closeCreateDialog();
        this.loadInvitations();
      } catch (error) {
        console.error('Failed to create invitation:', error);
      }
    },
    async deactivateInvitation(id) {
      try {
        const token = localStorage.getItem('token');
        await axios.delete(`http://localhost:6688/api/workspaces/invitations/${id}`, {
          headers: { Authorization: `Bearer ${token}` }
        });

        this.loadInvitations();
      } catch (error) {
        console.error('Failed to deactivate invitation:', error);
      }
    },
    async joinWorkspace() {
      if (!this.joinCode.trim()) {
        return;
      }

      try {
        const token = localStorage.getItem('token');
        const response = await axios.post(
          'http://localhost:6688/api/workspaces/join',
          { invite_code: this.joinCode.trim() },
          { headers: { Authorization: `Bearer ${token}` } }
        );

        localStorage.setItem('token', response.data.token);
        this.joinCode = '';
        window.location.reload();
      } catch (error) {
        console.error('Failed to join workspace:', error);
      }
    },
    copyCode(code) {
      navigator.clipboard.writeText(code);
    },
    formatDate(dateString) {
      const date = new Date(dateString);
      return date.toLocaleDateString() + ' ' + date.toLocaleTimeString();
    },
    closeCreateDialog() {
      this.showCreateDialog = false;
      this.newInvite = {
        expires_in_days: null,
        max_uses: null
      };
    }
  }
};
</script>

<style scoped>
/* Page Container */
.page-container {
  min-height: 100vh;
  background-color: #1e1e2e;
  padding: 32px;
  box-sizing: border-box;
}

/* Header */
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 32px;
  padding-bottom: 24px;
  border-bottom: 1px solid #45475a;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.btn-back {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 40px;
  height: 40px;
  background: #313244;
  border: 1px solid #45475a;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-back:hover {
  background: #45475a;
  border-color: #585b70;
}

.back-icon {
  width: 20px;
  height: 20px;
  color: #cdd6f4;
}

.header-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.title {
  font-size: 24px;
  font-weight: 700;
  color: #cdd6f4;
  margin: 0;
}

.subtitle {
  font-size: 14px;
  color: #9399b2;
  margin: 0;
}

/* Icon Box */
.icon-box {
  width: 48px;
  height: 48px;
  border-radius: 12px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.icon-box.purple {
  background: linear-gradient(135deg, #cba6f7 0%, #89b4fa 100%);
}

.icon-box.green {
  background: linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);
}

.icon-box.blue {
  background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
}

.icon {
  width: 24px;
  height: 24px;
  color: #1e1e2e;
}

/* Create Button */
.btn-create {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 20px;
  background: linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);
  color: #1e1e2e;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-create:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(166, 227, 161, 0.4);
}

.btn-icon {
  width: 18px;
  height: 18px;
}

/* Modal */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(17, 17, 27, 0.85);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
  backdrop-filter: blur(8px);
}

.modal {
  background: linear-gradient(135deg, #1e1e2e 0%, #181825 100%);
  padding: 32px;
  border-radius: 20px;
  min-width: 480px;
  border: 1px solid #313244;
  box-shadow: 0 24px 64px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 28px;
}

.modal-title-group {
  display: flex;
  align-items: center;
  gap: 12px;
}

.modal-title {
  font-size: 20px;
  font-weight: 700;
  color: #cdd6f4;
  margin: 0;
}

.btn-close {
  background: none;
  border: none;
  cursor: pointer;
  padding: 8px;
  border-radius: 8px;
  transition: background 0.2s;
}

.btn-close:hover {
  background: #313244;
}

.close-icon {
  width: 24px;
  height: 24px;
  color: #9399b2;
}

/* Form */
.form-group {
  margin-bottom: 20px;
}

.label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  color: #bac2de;
  margin-bottom: 8px;
}

.input {
  width: 100%;
  padding: 14px 16px;
  background: #1e1e2e;
  border: 1px solid #45475a;
  border-radius: 10px;
  color: #cdd6f4;
  font-size: 14px;
  transition: all 0.2s;
  box-sizing: border-box;
}

.input::placeholder {
  color: #6c7086;
}

.input:focus {
  outline: none;
  border-color: #89b4fa;
  box-shadow: 0 0 0 3px rgba(137, 180, 250, 0.2);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 28px;
}

.btn-cancel {
  padding: 12px 20px;
  background: #45475a;
  color: #cdd6f4;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-cancel:hover {
  background: #585b70;
}

.btn-confirm {
  padding: 12px 24px;
  background: linear-gradient(135deg, #a6e3a1 0%, #94e2d5 100%);
  color: #1e1e2e;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-confirm:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(166, 227, 161, 0.4);
}

/* Invitations List */
.invitations-list {
  margin-bottom: 40px;
}

.loading {
  display: flex;
  justify-content: center;
  padding: 60px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid #45475a;
  border-top-color: #89b4fa;
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.empty {
  text-align: center;
  padding: 60px 20px;
}

.empty-icon {
  width: 64px;
  height: 64px;
  color: #6c7086;
  margin-bottom: 16px;
}

.empty-text {
  font-size: 16px;
  color: #6c7086;
}

.invitations {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* Invitation Card */
.invitation-card {
  background: linear-gradient(135deg, #313244 0%, #45475a 100%);
  border: 1px solid #585b70;
  border-radius: 16px;
  padding: 24px;
  transition: all 0.2s;
}

.invitation-card:hover {
  border-color: #89b4fa;
  transform: translateY(-2px);
  box-shadow: 0 12px 32px rgba(137, 180, 250, 0.15);
}

.invitation-card.inactive {
  opacity: 0.5;
}

.invitation-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 16px;
}

.label-code {
  font-size: 14px;
  color: #9399b2;
}

.code {
  font-family: 'JetBrains Mono', 'Fira Code', monospace;
  font-size: 16px;
  font-weight: 600;
  color: #89b4fa;
  background: #1e1e2e;
  padding: 8px 14px;
  border-radius: 8px;
  border: 1px solid #45475a;
}

.btn-copy {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: #89b4fa;
  color: #1e1e2e;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
  margin-left: auto;
}

.btn-copy:hover {
  background: #74c7ec;
  transform: scale(1.02);
}

.copy-icon {
  width: 16px;
  height: 16px;
}

.invitation-details {
  display: flex;
  align-items: center;
  gap: 24px;
  flex-wrap: wrap;
}

.detail-item {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
  color: #bac2de;
}

.detail-icon {
  width: 16px;
  height: 16px;
  color: #6c7086;
}

.text-muted {
  color: #6c7086;
}

.badge {
  padding: 6px 12px;
  border-radius: 20px;
  font-size: 12px;
  font-weight: 600;
}

.badge.active {
  background: rgba(166, 227, 161, 0.2);
  color: #a6e3a1;
}

.badge.inactive {
  background: rgba(243, 139, 168, 0.2);
  color: #f38ba8;
}

.btn-deactivate {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 8px 14px;
  background: rgba(243, 139, 168, 0.15);
  color: #f38ba8;
  border: none;
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  margin-top: 16px;
}

.btn-deactivate:hover {
  background: rgba(243, 139, 168, 0.25);
}

/* Join Section */
.join-section {
  background: linear-gradient(135deg, #313244 0%, #45475a 100%);
  border: 1px solid #585b70;
  border-radius: 20px;
  padding: 32px;
}

.join-header {
  display: flex;
  align-items: center;
  gap: 16px;
  margin-bottom: 24px;
}

.join-text {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.join-title {
  font-size: 18px;
  font-weight: 700;
  color: #cdd6f4;
  margin: 0;
}

.join-subtitle {
  font-size: 14px;
  color: #9399b2;
  margin: 0;
}

.join-form {
  display: flex;
  gap: 12px;
}

.join-input {
  flex: 1;
  padding: 14px 16px;
  background: #1e1e2e;
  border: 1px solid #45475a;
  border-radius: 10px;
  color: #cdd6f4;
  font-size: 14px;
  transition: all 0.2s;
  box-sizing: border-box;
}

.join-input::placeholder {
  color: #6c7086;
}

.join-input:focus {
  outline: none;
  border-color: #89b4fa;
  box-shadow: 0 0 0 3px rgba(137, 180, 250, 0.2);
}

.btn-join {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 14px 24px;
  background: linear-gradient(135deg, #89b4fa 0%, #74c7ec 100%);
  color: #1e1e2e;
  border: none;
  border-radius: 10px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-join:hover {
  transform: translateY(-2px);
  box-shadow: 0 8px 24px rgba(137, 180, 250, 0.4);
}

.join-icon {
  width: 18px;
  height: 18px;
}
</style>
