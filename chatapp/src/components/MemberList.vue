<template>
    <div
        v-if="isVisible && isGroupChat"
        class="w-64 bg-[#313244] border-l border-[#45475a] flex flex-col h-screen p-4"
    >
        <!-- Header -->
        <div class="flex items-center justify-between mb-6">
            <h2 class="text-lg font-bold text-[#cdd6f4]">Members</h2>
            <div class="flex items-center gap-2">
                <button
                    @click="openInviteModal"
                    class="text-[#9399b2] hover:text-[#a6e3a1] transition-colors duration-200"
                    title="Invite members"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                    </svg>
                </button>
                <button
                    @click="$emit('close')"
                    class="text-[#9399b2] hover:text-[#cdd6f4] transition-colors duration-200"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </div>

        <!-- Member Count -->
        <div class="mb-4 text-sm text-[#9399b2]">
            {{ members.length }} {{ members.length === 1 ? 'member' : 'members' }}
        </div>

        <!-- Members List -->
        <div class="flex-1 overflow-y-auto">
            <div
                v-for="member in members"
                :key="member.id"
                class="flex items-center p-3 mb-2 rounded-lg hover:bg-[#45475a] transition-colors duration-200"
            >
                <img
                    :src="`https://ui-avatars.com/api/?name=${member.fullname.replace(' ', '+')}&background=89b4fa&color=1e1e2e`"
                    class="w-10 h-10 rounded-full mr-3 ring-2 ring-[#45475a]"
                    alt="Avatar"
                />
                <div class="flex-1">
                    <div class="text-[#cdd6f4] font-medium">{{ member.fullname }}</div>
                    <div class="text-xs text-[#9399b2]">{{ member.email }}</div>
                </div>
                <div
                    v-if="member.id === currentUserId"
                    class="text-xs bg-[#89b4fa] text-[#1e1e2e] px-2 py-1 rounded"
                >
                    You
                </div>
            </div>
        </div>

        <!-- Chat Management Actions -->
        <div class="mt-4 pt-4 border-t border-[#45475a] space-y-2">
            <button
                @click="openRenameModal"
                class="w-full flex items-center gap-2 px-3 py-2 text-[#cdd6f4] hover:bg-[#45475a] rounded-lg transition-colors duration-200"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z" />
                </svg>
                <span class="text-sm">Rename Group</span>
            </button>
            <button
                @click="confirmLeaveGroup"
                class="w-full flex items-center gap-2 px-3 py-2 text-[#f9e2af] hover:bg-[#45475a] rounded-lg transition-colors duration-200"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
                </svg>
                <span class="text-sm">Leave Group</span>
            </button>
            <button
                @click="confirmDeleteChat"
                class="w-full flex items-center gap-2 px-3 py-2 text-[#f38ba8] hover:bg-[#45475a] rounded-lg transition-colors duration-200"
            >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
                <span class="text-sm">Delete Chat</span>
            </button>
        </div>

        <!-- Invite Members Modal -->
        <div
            v-if="showInviteModal"
            class="fixed inset-0 bg-[#11111b] bg-opacity-80 flex items-center justify-center z-50 backdrop-blur-sm"
            @click.self="closeInviteModal"
        >
            <div class="bg-gradient-to-br from-[#1e1e2e] to-[#181825] rounded-2xl p-6 w-[400px] border border-[#313244] shadow-2xl">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-xl font-bold text-[#cdd6f4]">Invite Members</h3>
                    <button @click="closeInviteModal" class="text-[#9399b2] hover:text-[#cdd6f4]">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <div class="space-y-4">
                    <div class="max-h-64 overflow-y-auto bg-[#313244] border border-[#45475a] rounded-lg p-2">
                        <label
                            v-for="user in availableUsersToInvite"
                            :key="user.id"
                            class="flex items-center p-2 hover:bg-[#45475a] rounded cursor-pointer transition-colors duration-200"
                        >
                            <input
                                type="checkbox"
                                :value="user.id"
                                v-model="selectedUsersToInvite"
                                class="mr-3 text-[#a6e3a1] focus:ring-[#a6e3a1]"
                            />
                            <img
                                :src="`https://ui-avatars.com/api/?name=${user.fullname.replace(' ', '+')}`"
                                class="w-8 h-8 rounded-full mr-3"
                                alt="Avatar"
                            />
                            <span class="text-[#cdd6f4]">{{ user.fullname }}</span>
                        </label>
                        <div v-if="availableUsersToInvite.length === 0" class="text-center text-[#9399b2] py-4">
                            No users available to invite
                        </div>
                    </div>

                    <div v-if="inviteError" class="flex items-start gap-2 p-3 bg-[#f38ba8]/10 border border-[#f38ba8]/30 rounded-lg">
                        <span class="text-[#f38ba8] text-sm">{{ inviteError }}</span>
                    </div>

                    <div class="flex justify-end gap-3">
                        <button
                            @click="closeInviteModal"
                            class="px-4 py-2 bg-[#45475a] hover:bg-[#585b70] text-[#cdd6f4] rounded-lg transition-colors duration-200"
                        >
                            Cancel
                        </button>
                        <button
                            @click="handleInviteMembers"
                            :disabled="isInviting || selectedUsersToInvite.length === 0"
                            class="px-4 py-2 bg-gradient-to-r from-[#a6e3a1] to-[#94e2d5] text-[#1e1e2e] rounded-lg disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
                        >
                            {{ isInviting ? 'Inviting...' : 'Invite' }}
                        </button>
                    </div>
                </div>
            </div>
        </div>

        <!-- Rename Group Modal -->
        <div
            v-if="showRenameModal"
            class="fixed inset-0 bg-[#11111b] bg-opacity-80 flex items-center justify-center z-50 backdrop-blur-sm"
            @click.self="closeRenameModal"
        >
            <div class="bg-gradient-to-br from-[#1e1e2e] to-[#181825] rounded-2xl p-6 w-[400px] border border-[#313244] shadow-2xl">
                <div class="flex items-center justify-between mb-4">
                    <h3 class="text-xl font-bold text-[#cdd6f4]">Rename Group</h3>
                    <button @click="closeRenameModal" class="text-[#9399b2] hover:text-[#cdd6f4]">
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <div class="space-y-4">
                    <input
                        v-model="newGroupName"
                        type="text"
                        placeholder="Enter new group name"
                        class="w-full px-4 py-3 bg-[#313244] border border-[#45475a] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#89b4fa] text-[#cdd6f4] placeholder-[#6c7086]"
                    />

                    <div v-if="renameError" class="flex items-start gap-2 p-3 bg-[#f38ba8]/10 border border-[#f38ba8]/30 rounded-lg">
                        <span class="text-[#f38ba8] text-sm">{{ renameError }}</span>
                    </div>

                    <div class="flex justify-end gap-3">
                        <button
                            @click="closeRenameModal"
                            class="px-4 py-2 bg-[#45475a] hover:bg-[#585b70] text-[#cdd6f4] rounded-lg transition-colors duration-200"
                        >
                            Cancel
                        </button>
                        <button
                            @click="handleRenameGroup"
                            :disabled="isRenaming || !newGroupName.trim()"
                            class="px-4 py-2 bg-gradient-to-r from-[#89b4fa] to-[#74c7ec] text-[#1e1e2e] rounded-lg disabled:opacity-50 disabled:cursor-not-allowed transition-colors duration-200"
                        >
                            {{ isRenaming ? 'Renaming...' : 'Rename' }}
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    props: {
        isVisible: {
            type: Boolean,
            default: false,
        },
    },
    data() {
        return {
            showInviteModal: false,
            selectedUsersToInvite: [],
            inviteError: "",
            isInviting: false,
            showRenameModal: false,
            newGroupName: "",
            renameError: "",
            isRenaming: false,
        };
    },
    computed: {
        activeChannel() {
            return this.$store.state.activeChannel;
        },
        isGroupChat() {
            return this.activeChannel && this.activeChannel.type !== "single";
        },
        members() {
            if (!this.activeChannel || !this.activeChannel.members) {
                return [];
            }
            // Get member details from users store
            return this.activeChannel.members
                .map(memberId => this.$store.state.users[memberId])
                .filter(member => member); // Filter out undefined members
        },
        currentUserId() {
            return this.$store.state.user?.id;
        },
        availableUsersToInvite() {
            // Get all workspace users who are not already members
            const users = this.$store.state.users;
            const memberIds = this.activeChannel?.members || [];
            return Object.values(users).filter(user => !memberIds.includes(user.id));
        },
    },
    methods: {
        openInviteModal() {
            this.showInviteModal = true;
            this.selectedUsersToInvite = [];
            this.inviteError = "";
        },
        closeInviteModal() {
            this.showInviteModal = false;
            this.selectedUsersToInvite = [];
            this.inviteError = "";
        },
        async handleInviteMembers() {
            if (this.selectedUsersToInvite.length === 0) {
                this.inviteError = "Please select at least one user";
                return;
            }

            this.isInviting = true;
            this.inviteError = "";

            try {
                await this.$store.dispatch("addMembersToChat", {
                    chatId: this.activeChannel.id,
                    memberIds: this.selectedUsersToInvite,
                });
                this.closeInviteModal();
            } catch (error) {
                this.inviteError = error.response?.data?.error || "Failed to invite members";
            } finally {
                this.isInviting = false;
            }
        },
        openRenameModal() {
            this.showRenameModal = true;
            this.newGroupName = this.activeChannel?.name || "";
            this.renameError = "";
        },
        closeRenameModal() {
            this.showRenameModal = false;
            this.newGroupName = "";
            this.renameError = "";
        },
        async handleRenameGroup() {
            if (!this.newGroupName.trim()) {
                this.renameError = "Group name cannot be empty";
                return;
            }

            this.isRenaming = true;
            this.renameError = "";

            try {
                await this.$store.dispatch("renameChat", {
                    chatId: this.activeChannel.id,
                    name: this.newGroupName.trim(),
                });
                this.closeRenameModal();
            } catch (error) {
                this.renameError = error.response?.data?.error || "Failed to rename group";
            } finally {
                this.isRenaming = false;
            }
        },
        confirmLeaveGroup() {
            if (confirm("Are you sure you want to leave this group?")) {
                this.handleLeaveGroup();
            }
        },
        async handleLeaveGroup() {
            try {
                const currentChatId = this.activeChannel.id;

                await this.$store.dispatch("leaveChat", {
                    chatId: currentChatId,
                });

                this.$emit('close');

                // Get updated channels list after leaving
                const channels = this.$store.getters.getChannels;
                const singleChannels = this.$store.getters.getSingChannels;
                const allChannels = [...channels, ...singleChannels];

                // Find a different chat to redirect to (not the one we just left)
                const nextChat = allChannels.find(c => c.id !== currentChatId);

                if (nextChat) {
                    this.$router.push(`/chats/${nextChat.id}`);
                } else {
                    // No chats available, go to home
                    this.$router.push('/');
                }
            } catch (error) {
                alert(error.response?.data?.error || "Failed to leave group");
            }
        },
        confirmDeleteChat() {
            if (confirm("Are you sure you want to delete this chat? This action cannot be undone.")) {
                this.handleDeleteChat();
            }
        },
        async handleDeleteChat() {
            try {
                const currentChatId = this.activeChannel.id;

                await this.$store.dispatch("deleteChat", {
                    chatId: currentChatId,
                });

                this.$emit('close');

                // Get updated channels list after deleting
                const channels = this.$store.getters.getChannels;
                const singleChannels = this.$store.getters.getSingChannels;
                const allChannels = [...channels, ...singleChannels];

                // Find a different chat to redirect to (not the one we just deleted)
                const nextChat = allChannels.find(c => c.id !== currentChatId);

                if (nextChat) {
                    this.$router.push(`/chats/${nextChat.id}`);
                } else {
                    // No chats available, go to home
                    this.$router.push('/');
                }
            } catch (error) {
                alert(error.response?.data?.error || "Failed to delete chat");
            }
        },
    },
};
</script>
