<template>
    <div class="flex h-screen bg-[#1e1e2e]">
        <Sidebar />
        <div class="flex flex-col flex-1 relative">
            <!-- Header -->
            <div
                v-if="activeChannel"
                class="flex items-center justify-between px-5 py-3 bg-[#313244] border-b border-[#45475a]"
            >
                <!-- Channel title -->
                <div class="flex items-center gap-2">
                    <template v-if="activeChannel.type === 'single'">
                        <div class="w-8 h-8 rounded-full bg-[#89b4fa] flex items-center justify-center text-[#1e1e2e] text-sm font-bold">
                            {{ dmRecipientInitials }}
                        </div>
                        <h1 class="text-lg font-bold text-[#cdd6f4]">{{ dmRecipientName }}</h1>
                    </template>
                    <template v-else>
                        <h1 class="text-lg font-bold text-[#cdd6f4]"># {{ activeChannel.name }}</h1>
                    </template>
                </div>

                <!-- Action buttons -->
                <div class="flex items-center gap-2">
                    <button
                        @click="toggleAgentList"
                        class="flex items-center gap-2 px-3 py-2 text-[#9399b2] hover:text-[#cdd6f4] hover:bg-[#45475a] rounded-lg transition-colors duration-200"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                        </svg>
                        <span class="text-sm">Agents</span>
                    </button>
                    <button
                        v-if="activeChannel.type !== 'single'"
                        @click="toggleMemberList"
                        class="flex items-center gap-2 px-3 py-2 text-[#9399b2] hover:text-[#cdd6f4] hover:bg-[#45475a] rounded-lg transition-colors duration-200"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                        </svg>
                        <span class="text-sm">Members</span>
                    </button>
                </div>
            </div>
            <MessageList class="flex-1 overflow-y-auto" />
            <MessageSend class="absolute bottom-0 left-0 w-full" />
        </div>
        <MemberList :isVisible="showMemberList" @close="toggleMemberList" />
        <AgentList :isVisible="showAgentList" @close="toggleAgentList" />
    </div>
</template>

<script>
import Sidebar from "../components/Sidebar.vue";
import MessageList from "../components/MessageList.vue";
import MessageSend from "../components/MessageSend.vue";
import MemberList from "../components/MemberList.vue";
import AgentList from "../components/AgentList.vue";

export default {
    components: {
        Sidebar,
        MessageList,
        MessageSend,
        MemberList,
        AgentList,
    },
    data() {
        return {
            showMemberList: false,
            showAgentList: false,
        };
    },
    computed: {
        activeChannel() {
            return this.$store.state.activeChannel;
        },
        dmRecipientName() {
            if (!this.activeChannel || this.activeChannel.type !== 'single') return '';
            const myId = this.$store.state.user?.id;
            const recipientId = this.activeChannel.members?.find((id) => id !== myId);
            const recipient = this.$store.state.users[recipientId];
            return recipient?.fullname || recipient?.email || 'Unknown';
        },
        dmRecipientInitials() {
            const name = this.dmRecipientName;
            if (!name) return '?';
            return name.split(' ').map((n) => n[0]).join('').slice(0, 2).toUpperCase();
        },
    },
    methods: {
        toggleMemberList() {
            this.showMemberList = !this.showMemberList;
        },
        toggleAgentList() {
            this.showAgentList = !this.showAgentList;
        },
    },
};
</script>
