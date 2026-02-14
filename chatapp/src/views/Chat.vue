<template>
    <div class="flex h-screen bg-[#1e1e2e]">
        <Sidebar />
        <div class="flex flex-col flex-1 relative">
            <!-- Header with member list toggle -->
            <div
                v-if="activeChannel && activeChannel.type !== 'single'"
                class="flex items-center justify-between px-5 py-3 bg-[#313244] border-b border-[#45475a]"
            >
                <div class="flex items-center">
                    <h1 class="text-lg font-bold text-[#cdd6f4]">
                        # {{ activeChannel.name }}
                    </h1>
                </div>
                <button
                    @click="toggleMemberList"
                    class="flex items-center gap-2 px-3 py-2 text-[#9399b2] hover:text-[#cdd6f4] hover:bg-[#45475a] rounded-lg transition-colors duration-200"
                >
                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197M13 7a4 4 0 11-8 0 4 4 0 018 0z" />
                    </svg>
                    <span class="text-sm">Members</span>
                </button>
            </div>
            <MessageList class="flex-1 overflow-y-auto" />
            <MessageSend class="absolute bottom-0 left-0 w-full" />
        </div>
        <MemberList :isVisible="showMemberList" @close="toggleMemberList" />
    </div>
</template>

<script>
import Sidebar from "../components/Sidebar.vue";
import MessageList from "../components/MessageList.vue";
import MessageSend from "../components/MessageSend.vue";
import MemberList from "../components/MemberList.vue";

export default {
    components: {
        Sidebar,
        MessageList,
        MessageSend,
        MemberList,
    },
    data() {
        return {
            showMemberList: false,
        };
    },
    computed: {
        activeChannel() {
            return this.$store.state.activeChannel;
        },
    },
    methods: {
        toggleMemberList() {
            this.showMemberList = !this.showMemberList;
        },
    },
};
</script>
