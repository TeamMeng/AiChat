<template>
  <div class="workspace-invite">
    <div class="invite-header">
      <h3>Workspace Invitations</h3>
      <button @click="showCreateDialog = true" class="create-btn">
        Create Invitation
      </button>
    </div>

    <!-- Create Invitation Dialog -->
    <div v-if="showCreateDialog" class="dialog-overlay" @click="closeCreateDialog">
      <div class="dialog" @click.stop>
        <h4>Create Invitation</h4>
        <div class="form-group">
          <label>Expires in (days)</label>
          <input
            v-model.number="newInvite.expires_in_days"
            type="number"
            placeholder="Leave empty for no expiration"
          />
        </div>
        <div class="form-group">
          <label>Max uses</label>
          <input
            v-model.number="newInvite.max_uses"
            type="number"
            placeholder="Leave empty for unlimited"
          />
        </div>
        <div class="dialog-actions">
          <button @click="closeCreateDialog" class="cancel-btn">Cancel</button>
          <button @click="createInvitation" class="confirm-btn">Create</button>
        </div>
      </div>
    </div>

    <!-- Invitations List -->
    <div class="invitations-list">
      <div v-if="loading" class="loading">Loading invitations...</div>
      <div v-else-if="invitations.length === 0" class="empty">
        No invitations yet. Create one to invite others!
      </div>
      <div v-else>
        <div
          v-for="invitation in invitations"
          :key="invitation.id"
          class="invitation-item"
          :class="{ inactive: !invitation.is_active }"
        >
          <div class="invite-code">
            <strong>Code:</strong>
            <span class="code">{{ invitation.invite_code }}</span>
            <button @click="copyCode(invitation.invite_code)" class="copy-btn">
              Copy
            </button>
          </div>
          <div class="invite-details">
            <span v-if="invitation.expires_at">
              Expires: {{ formatDate(invitation.expires_at) }}
            </span>
            <span v-else>No expiration</span>
            <span v-if="invitation.max_uses">
              Uses: {{ invitation.used_count }}/{{ invitation.max_uses }}
            </span>
            <span v-else>
              Uses: {{ invitation.used_count }} (unlimited)
            </span>
            <span :class="invitation.is_active ? 'active' : 'inactive-badge'">
              {{ invitation.is_active ? 'Active' : 'Inactive' }}
            </span>
          </div>
          <button
            v-if="invitation.is_active"
            @click="deactivateInvitation(invitation.id)"
            class="deactivate-btn"
          >
            Deactivate
          </button>
        </div>
      </div>
    </div>

    <!-- Join Workspace Section -->
    <div class="join-section">
      <h4>Join a Workspace</h4>
      <div class="join-form">
        <input
          v-model="joinCode"
          type="text"
          placeholder="Enter invitation code"
          @keyup.enter="joinWorkspace"
        />
        <button @click="joinWorkspace" class="join-btn">Join</button>
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
        alert('Failed to load invitations');
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
        alert('Invitation created successfully!');
      } catch (error) {
        console.error('Failed to create invitation:', error);
        alert('Failed to create invitation');
      }
    },
    async deactivateInvitation(id) {
      if (!confirm('Are you sure you want to deactivate this invitation?')) {
        return;
      }

      try {
        const token = localStorage.getItem('token');
        await axios.delete(`http://localhost:6688/api/workspaces/invitations/${id}`, {
          headers: { Authorization: `Bearer ${token}` }
        });

        this.loadInvitations();
        alert('Invitation deactivated');
      } catch (error) {
        console.error('Failed to deactivate invitation:', error);
        alert('Failed to deactivate invitation');
      }
    },
    async joinWorkspace() {
      if (!this.joinCode.trim()) {
        alert('Please enter an invitation code');
        return;
      }

      try {
        const token = localStorage.getItem('token');
        const response = await axios.post(
          'http://localhost:6688/api/workspaces/join',
          { invite_code: this.joinCode.trim() },
          { headers: { Authorization: `Bearer ${token}` } }
        );

        // Update token with new workspace
        localStorage.setItem('token', response.data.token);

        alert(`Successfully joined workspace: ${response.data.workspace.name}`);
        this.joinCode = '';

        // Reload the page to refresh with new workspace
        window.location.reload();
      } catch (error) {
        console.error('Failed to join workspace:', error);
        const message = error.response?.data?.error || 'Failed to join workspace';
        alert(message);
      }
    },
    copyCode(code) {
      navigator.clipboard.writeText(code).then(() => {
        alert('Code copied to clipboard!');
      }).catch(() => {
        alert('Failed to copy code');
      });
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
.workspace-invite {
  padding: 20px;
  max-width: 800px;
  margin: 0 auto;
}

.invite-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
}

.invite-header h3 {
  margin: 0;
  color: #333;
}

.create-btn {
  padding: 8px 16px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.create-btn:hover {
  background-color: #45a049;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 1000;
}

.dialog {
  background: white;
  padding: 24px;
  border-radius: 8px;
  min-width: 400px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
}

.dialog h4 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #333;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  color: #666;
  font-size: 14px;
}

.form-group input {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  box-sizing: border-box;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
  margin-top: 20px;
}

.cancel-btn, .confirm-btn {
  padding: 8px 16px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-btn {
  background-color: #f0f0f0;
  color: #333;
}

.cancel-btn:hover {
  background-color: #e0e0e0;
}

.confirm-btn {
  background-color: #4CAF50;
  color: white;
}

.confirm-btn:hover {
  background-color: #45a049;
}

.invitations-list {
  margin-bottom: 40px;
}

.loading, .empty {
  text-align: center;
  padding: 40px;
  color: #999;
}

.invitation-item {
  background: white;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 12px;
}

.invitation-item.inactive {
  opacity: 0.6;
  background-color: #f9f9f9;
}

.invite-code {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.code {
  font-family: monospace;
  font-size: 16px;
  font-weight: bold;
  color: #2196F3;
  background-color: #f0f8ff;
  padding: 4px 8px;
  border-radius: 4px;
}

.copy-btn {
  padding: 4px 12px;
  background-color: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.copy-btn:hover {
  background-color: #1976D2;
}

.invite-details {
  display: flex;
  gap: 16px;
  font-size: 14px;
  color: #666;
  margin-bottom: 12px;
}

.active {
  color: #4CAF50;
  font-weight: bold;
}

.inactive-badge {
  color: #f44336;
  font-weight: bold;
}

.deactivate-btn {
  padding: 6px 12px;
  background-color: #f44336;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
}

.deactivate-btn:hover {
  background-color: #d32f2f;
}

.join-section {
  border-top: 2px solid #eee;
  padding-top: 20px;
}

.join-section h4 {
  margin-top: 0;
  margin-bottom: 16px;
  color: #333;
}

.join-form {
  display: flex;
  gap: 8px;
}

.join-form input {
  flex: 1;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 14px;
}

.join-btn {
  padding: 10px 24px;
  background-color: #2196F3;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.join-btn:hover {
  background-color: #1976D2;
}
</style>
