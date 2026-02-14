<template>
    <div class="w-64 bg-[#313244] text-[#cdd6f4] flex flex-col h-screen p-4 text-sm border-r border-[#45475a]">
        <div class="flex items-center justify-between mb-6">
            <div
                class="font-bold text-base truncate cursor-pointer"
                @click="toggleDropdown"
            >
                <span>{{ workspaceName }}</span>
                <button class="text-[#9399b2] ml-1">&nbsp;▼</button>
            </div>
            <div
                v-if="dropdownVisible"
                class="absolute top-12 left-0 w-48 bg-[#313244] border border-[#45475a] rounded-md shadow-lg z-10"
            >
                <ul class="py-1">
                    <li
                        @click="goToInvitations"
                        class="px-4 py-2 hover:bg-[#45475a] cursor-pointer transition-colors duration-200"
                    >
                        Workspace Invitations
                    </li>
                    <li
                        @click="openChangePasswordModal"
                        class="px-4 py-2 hover:bg-[#45475a] cursor-pointer transition-colors duration-200"
                    >
                        Change Password
                    </li>
                    <li
                        @click="logout"
                        class="px-4 py-2 hover:bg-[#45475a] cursor-pointer transition-colors duration-200"
                    >
                        Logout
                    </li>
                    <!-- Add more dropdown items here as needed -->
                </ul>
            </div>
            <button
                @click="openCreateChatModal"
                class="text-[#9399b2] text-xl hover:text-[#cdd6f4] transition-colors duration-200"
            >
                +
            </button>
        </div>

        <div class="mb-6">
            <h2 class="text-xs uppercase text-[#9399b2] mb-2">Channels</h2>
            <ul>
                <li
                    v-for="channel in channels"
                    :key="channel.id"
                    @click="selectChannel(channel.id)"
                    :class="[
                        'px-2 py-1 rounded cursor-pointer transition-colors duration-200',
                        { 'bg-[#89b4fa] text-[#1e1e2e]': channel.id === activeChannelId },
                        { 'hover:bg-[#45475a]': channel.id !== activeChannelId }
                    ]"
                >
                    # {{ channel.name }}
                </li>
            </ul>
        </div>

        <div>
            <h2 class="text-xs uppercase text-[#9399b2] mb-2">
                Direct Messages
            </h2>
            <ul>
                <li
                    v-for="channel in singleChannels"
                    :key="channel.id"
                    @click="selectChannel(channel.id)"
                    :class="[
                        'flex items-center px-2 py-1 rounded cursor-pointer transition-colors duration-200',
                        { 'bg-[#89b4fa] text-[#1e1e2e]': channel.id === activeChannelId },
                        { 'hover:bg-[#45475a]': channel.id !== activeChannelId }
                    ]"
                >
                    <img
                        :src="`https://ui-avatars.com/api/?name=${channel.recipient.fullname.replace(' ', '+')}`"
                        class="w-6 h-6 rounded-full mr-2"
                        alt="Avatar"
                    />
                    {{ channel.recipient.fullname }}
                </li>
            </ul>
        </div>

        <!-- Create Chat Modal -->
        <div
            v-if="showCreateChatModal"
            class="fixed inset-0 bg-[#11111b] bg-opacity-80 flex items-center justify-center z-50 backdrop-blur-sm transition-opacity duration-300"
            @click.self="closeCreateChatModal"
        >
            <div class="bg-gradient-to-br from-[#1e1e2e] to-[#181825] rounded-2xl p-8 w-[450px] border border-[#313244] shadow-2xl transform transition-all duration-300 scale-100">
                <!-- Header -->
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-[#a6e3a1] rounded-full flex items-center justify-center">
                            <svg class="w-5 h-5 text-[#1e1e2e]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                            </svg>
                        </div>
                        <h2 class="text-2xl font-bold text-[#cdd6f4]">Create Chat</h2>
                    </div>
                    <button
                        @click="closeCreateChatModal"
                        class="text-[#9399b2] hover:text-[#cdd6f4] transition-colors duration-200"
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <form @submit.prevent="handleCreateChat" class="space-y-5">
                    <!-- Chat Type -->
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-[#bac2de]">Chat Type</label>
                        <div class="flex gap-4">
                            <label class="flex items-center cursor-pointer">
                                <input
                                    type="radio"
                                    v-model="chatType"
                                    value="direct"
                                    class="mr-2 text-[#a6e3a1] focus:ring-[#a6e3a1]"
                                />
                                <span class="text-[#cdd6f4]">Direct Message</span>
                            </label>
                            <label class="flex items-center cursor-pointer">
                                <input
                                    type="radio"
                                    v-model="chatType"
                                    value="group"
                                    class="mr-2 text-[#a6e3a1] focus:ring-[#a6e3a1]"
                                />
                                <span class="text-[#cdd6f4]">Group Chat</span>
                            </label>
                        </div>
                    </div>

                    <!-- Chat Name (only for group chat) -->
                    <div v-if="chatType === 'group'" class="space-y-2">
                        <label class="block text-sm font-medium text-[#bac2de]">Chat Name</label>
                        <input
                            v-model="chatName"
                            type="text"
                            @input="checkChatNameExists"
                            class="w-full px-4 py-3 bg-[#313244] border border-[#45475a] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#a6e3a1] focus:border-transparent transition-all duration-200 text-[#cdd6f4] placeholder-[#6c7086]"
                            placeholder="Enter chat name (optional)"
                        />
                        <p v-if="chatNameWarning" class="text-[#f9e2af] text-xs mt-1">
                            {{ chatNameWarning }}
                        </p>
                    </div>

                    <!-- Select Users -->
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-[#bac2de]">
                            {{ chatType === 'direct' ? 'Select User' : 'Select Users' }}
                        </label>
                        <div class="max-h-48 overflow-y-auto bg-[#313244] border border-[#45475a] rounded-lg p-2">
                            <label
                                v-for="user in availableUsers"
                                :key="user.id"
                                class="flex items-center p-2 hover:bg-[#45475a] rounded cursor-pointer transition-colors duration-200"
                            >
                                <input
                                    type="checkbox"
                                    :value="user.id"
                                    v-model="selectedUsers"
                                    :disabled="chatType === 'direct' && selectedUsers.length >= 1 && !selectedUsers.includes(user.id)"
                                    class="mr-3 text-[#a6e3a1] focus:ring-[#a6e3a1]"
                                />
                                <img
                                    :src="`https://ui-avatars.com/api/?name=${user.fullname.replace(' ', '+')}`"
                                    class="w-8 h-8 rounded-full mr-3"
                                    alt="Avatar"
                                />
                                <span class="text-[#cdd6f4]">{{ user.fullname }}</span>
                            </label>
                        </div>
                    </div>

                    <!-- Error Message -->
                    <div v-if="createChatError" class="flex items-start gap-2 p-3 bg-[#f38ba8]/10 border border-[#f38ba8]/30 rounded-lg">
                        <svg class="w-5 h-5 text-[#f38ba8] flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                        </svg>
                        <span class="text-[#f38ba8] text-sm">{{ createChatError }}</span>
                    </div>

                    <!-- Success Message -->
                    <div v-if="createChatSuccess" class="flex items-start gap-2 p-3 bg-[#a6e3a1]/10 border border-[#a6e3a1]/30 rounded-lg">
                        <svg class="w-5 h-5 text-[#a6e3a1] flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                        </svg>
                        <span class="text-[#a6e3a1] text-sm">{{ createChatSuccess }}</span>
                    </div>

                    <!-- Buttons -->
                    <div class="flex justify-end gap-3 pt-2">
                        <button
                            type="button"
                            @click="closeCreateChatModal"
                            class="px-5 py-2.5 bg-[#45475a] hover:bg-[#585b70] text-[#cdd6f4] rounded-lg font-medium transition-all duration-200 hover:shadow-lg"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            class="px-5 py-2.5 bg-gradient-to-r from-[#a6e3a1] to-[#94e2d5] hover:from-[#94e2d5] hover:to-[#89dceb] text-[#1e1e2e] rounded-lg font-medium transition-all duration-200 hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                            :disabled="isCreatingChat || selectedUsers.length === 0"
                        >
                            <svg v-if="isCreatingChat" class="animate-spin h-4 w-4 text-[#1e1e2e]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            {{ isCreatingChat ? 'Creating...' : 'Create' }}
                        </button>
                    </div>
                </form>
            </div>
        </div>

        <!-- Change Password Modal -->
        <div
            v-if="showChangePasswordModal"
            class="fixed inset-0 bg-[#11111b] bg-opacity-80 flex items-center justify-center z-50 backdrop-blur-sm transition-opacity duration-300"
            @click.self="closeChangePasswordModal"
        >
            <div class="bg-gradient-to-br from-[#1e1e2e] to-[#181825] rounded-2xl p-8 w-[450px] border border-[#313244] shadow-2xl transform transition-all duration-300 scale-100">
                <!-- Header -->
                <div class="flex items-center justify-between mb-6">
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-[#89b4fa] rounded-full flex items-center justify-center">
                            <svg class="w-5 h-5 text-[#1e1e2e]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
                            </svg>
                        </div>
                        <h2 class="text-2xl font-bold text-[#cdd6f4]">Change Password</h2>
                    </div>
                    <button
                        @click="closeChangePasswordModal"
                        class="text-[#9399b2] hover:text-[#cdd6f4] transition-colors duration-200"
                    >
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                        </svg>
                    </button>
                </div>

                <form @submit.prevent="handleChangePassword" class="space-y-5">
                    <!-- Current Password -->
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-[#bac2de]">Current Password</label>
                        <div class="relative">
                            <input
                                v-model="oldPassword"
                                type="password"
                                class="w-full px-4 py-3 bg-[#313244] border border-[#45475a] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent transition-all duration-200 text-[#cdd6f4] placeholder-[#6c7086]"
                                placeholder="Enter current password"
                                required
                            />
                        </div>
                    </div>

                    <!-- New Password -->
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-[#bac2de]">New Password</label>
                        <div class="relative">
                            <input
                                v-model="newPassword"
                                type="password"
                                class="w-full px-4 py-3 bg-[#313244] border border-[#45475a] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent transition-all duration-200 text-[#cdd6f4] placeholder-[#6c7086]"
                                placeholder="Enter new password (min 6 characters)"
                                required
                            />
                        </div>
                    </div>

                    <!-- Confirm New Password -->
                    <div class="space-y-2">
                        <label class="block text-sm font-medium text-[#bac2de]">Confirm New Password</label>
                        <div class="relative">
                            <input
                                v-model="confirmPassword"
                                type="password"
                                class="w-full px-4 py-3 bg-[#313244] border border-[#45475a] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent transition-all duration-200 text-[#cdd6f4] placeholder-[#6c7086]"
                                placeholder="Confirm new password"
                                required
                            />
                        </div>
                    </div>

                    <!-- Error Message -->
                    <div v-if="errorMessage" class="flex items-start gap-2 p-3 bg-[#f38ba8]/10 border border-[#f38ba8]/30 rounded-lg">
                        <svg class="w-5 h-5 text-[#f38ba8] flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                        </svg>
                        <span class="text-[#f38ba8] text-sm">{{ errorMessage }}</span>
                    </div>

                    <!-- Success Message -->
                    <div v-if="successMessage" class="flex items-start gap-2 p-3 bg-[#a6e3a1]/10 border border-[#a6e3a1]/30 rounded-lg">
                        <svg class="w-5 h-5 text-[#a6e3a1] flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
                        </svg>
                        <span class="text-[#a6e3a1] text-sm">{{ successMessage }}</span>
                    </div>

                    <!-- Buttons -->
                    <div class="flex justify-end gap-3 pt-2">
                        <button
                            type="button"
                            @click="closeChangePasswordModal"
                            class="px-5 py-2.5 bg-[#45475a] hover:bg-[#585b70] text-[#cdd6f4] rounded-lg font-medium transition-all duration-200 hover:shadow-lg"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            class="px-5 py-2.5 bg-gradient-to-r from-[#89b4fa] to-[#74c7ec] hover:from-[#74c7ec] hover:to-[#89dceb] text-[#1e1e2e] rounded-lg font-medium transition-all duration-200 hover:shadow-lg disabled:opacity-50 disabled:cursor-not-allowed flex items-center gap-2"
                            :disabled="isSubmitting"
                        >
                            <svg v-if="isSubmitting" class="animate-spin h-4 w-4 text-[#1e1e2e]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                            </svg>
                            {{ isSubmitting ? 'Submitting...' : 'Confirm' }}
                        </button>
                    </div>
                </form>
            </div>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            dropdownVisible: false,
            showChangePasswordModal: false,
            oldPassword: "",
            newPassword: "",
            confirmPassword: "",
            errorMessage: "",
            successMessage: "",
            isSubmitting: false,
            showCreateChatModal: false,
            chatType: "direct",
            chatName: "",
            selectedUsers: [],
            createChatError: "",
            createChatSuccess: "",
            isCreatingChat: false,
            chatNameWarning: "",
        };
    },
    computed: {
        workspaceName() {
            return this.$store.getters.getWorkspace.name || "No Workspace";
        },
        channels() {
            return this.$store.getters.getChannels;
        },
        activeChannelId() {
            const channel = this.$store.state.activeChannel;
            if (!channel) {
                return null;
            }
            return channel.id;
        },
        singleChannels() {
            return this.$store.getters.getSingChannels;
        },
        availableUsers() {
            // Get all users except current user
            const users = this.$store.state.users;
            const currentUserId = this.$store.state.user.id;
            return Object.values(users).filter(user => user.id !== currentUserId);
        },
    },
    watch: {
        chatType(newType) {
            // Clear selections when switching chat type
            if (newType === "direct" && this.selectedUsers.length > 1) {
                // Keep only the first selected user for direct message
                this.selectedUsers = [this.selectedUsers[0]];
            }
        },
    },
    methods: {
        toggleDropdown() {
            this.dropdownVisible = !this.dropdownVisible;
        },
        openChangePasswordModal() {
            this.showChangePasswordModal = true;
            this.dropdownVisible = false;
            this.oldPassword = "";
            this.newPassword = "";
            this.confirmPassword = "";
            this.errorMessage = "";
            this.successMessage = "";
        },
        closeChangePasswordModal() {
            this.showChangePasswordModal = false;
            this.oldPassword = "";
            this.newPassword = "";
            this.confirmPassword = "";
            this.errorMessage = "";
            this.successMessage = "";
        },
        async handleChangePassword() {
            this.errorMessage = "";
            this.successMessage = "";

            // Validate passwords
            if (this.newPassword !== this.confirmPassword) {
                this.errorMessage = "New password and confirm password do not match";
                return;
            }

            if (this.newPassword.length < 6) {
                this.errorMessage = "New password must be at least 6 characters";
                return;
            }

            this.isSubmitting = true;

            try {
                await this.$store.dispatch("changePassword", {
                    oldPassword: this.oldPassword,
                    newPassword: this.newPassword,
                });
                this.successMessage = "Password changed successfully! Redirecting to login...";

                // Logout and redirect to login page after 1.5 seconds
                setTimeout(async () => {
                    this.closeChangePasswordModal();
                    this.$store.dispatch("userLogout");
                    this.$store.dispatch("logout");
                    await this.$router.push("/login");
                }, 1500);
            } catch (error) {
                if (error.response && error.response.data) {
                    this.errorMessage = error.response.data.error || "Failed to change password";
                } else {
                    this.errorMessage = "Failed to change password, please try again";
                }
            } finally {
                this.isSubmitting = false;
            }
        },
        async logout() {
            const from = `/chats/${this.activeChannelId}`;
            const to = "/logout";
            this.$store.dispatch("navigation", { from, to });
            this.$store.dispatch("userLogout");
            this.$store.dispatch("logout");
            await this.$router.push("/login");
        },
        goToInvitations() {
            this.dropdownVisible = false;
            this.$router.push("/invitations");
        },
        handleOutsideClick(event) {
            if (!this.$el.contains(event.target)) {
                this.dropdownVisible = false;
            }
        },
        addChannel() {
            const newChannel = {
                id: Date.now().toString(),
                name: `Channel ${this.channels.length + 1}`,
            };
            this.$store.dispatch("addChannel", newChannel);
        },
        selectChannel(channelId) {
            const from = `/chats/${this.activeChannelId}`;
            const to = `/chats/${channelId}`;
            this.$store.dispatch("navigation", { from, to });
            this.$store.dispatch("setActiveChannel", channelId);
        },
        openCreateChatModal() {
            this.showCreateChatModal = true;
            this.dropdownVisible = false;
            this.chatType = "direct";
            this.chatName = "";
            this.selectedUsers = [];
            this.createChatError = "";
            this.createChatSuccess = "";
        },
        closeCreateChatModal() {
            this.showCreateChatModal = false;
            this.chatType = "direct";
            this.chatName = "";
            this.selectedUsers = [];
            this.createChatError = "";
            this.createChatSuccess = "";
            this.chatNameWarning = "";
        },
        checkChatNameExists() {
            this.chatNameWarning = "";

            if (!this.chatName.trim()) {
                return;
            }

            // Check if a chat with this name already exists
            const existingChat = this.channels.find(
                channel => channel.name && channel.name.toLowerCase() === this.chatName.trim().toLowerCase()
            );

            if (existingChat) {
                this.chatNameWarning = "A chat with this name already exists";
            }
        },
        async handleCreateChat() {
            this.createChatError = "";
            this.createChatSuccess = "";

            // Validate selection
            if (this.selectedUsers.length === 0) {
                this.createChatError = "Please select at least one user";
                return;
            }

            if (this.chatType === "direct" && this.selectedUsers.length > 1) {
                this.createChatError = "Direct message can only have one recipient";
                return;
            }

            this.isCreatingChat = true;

            try {
                // Add current user to members list
                const members = [this.$store.state.user.id, ...this.selectedUsers];
                const isPublic = this.chatType === "group";

                const payload = {
                    members,
                    isPublic,
                };

                // Add name if provided for group chat
                if (this.chatType === "group" && this.chatName.trim()) {
                    payload.name = this.chatName.trim();
                }

                const newChat = await this.$store.dispatch("createChat", payload);

                this.createChatSuccess = "Chat created successfully!";

                // Close modal and select the new chat after a short delay
                setTimeout(() => {
                    this.closeCreateChatModal();
                    this.selectChannel(newChat.id);
                }, 1000);
            } catch (error) {
                if (error.response && error.response.data) {
                    this.createChatError = error.response.data.error || "Failed to create chat";
                } else {
                    this.createChatError = "Failed to create chat, please try again";
                }
            } finally {
                this.isCreatingChat = false;
            }
        },
    },
    mounted() {
        document.addEventListener("click", this.handleOutsideClick);
    },
    beforeDestroy() {
        document.removeEventListener("click", this.handleOutsideClick);
    },
};
</script>
