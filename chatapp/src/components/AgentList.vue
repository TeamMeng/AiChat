<template>
    <div
        v-if="isVisible"
        class="fixed inset-0 bg-[#11111b] bg-opacity-80 flex items-center justify-center z-50 backdrop-blur-sm transition-opacity duration-300"
        @click.self="$emit('close')"
    >
        <div class="bg-gradient-to-br from-[#1e1e2e] to-[#181825] rounded-2xl p-8 w-[500px] max-h-[80vh] border border-[#313244] shadow-2xl transform transition-all duration-300 scale-100">
            <!-- Header -->
            <div class="flex items-center justify-between mb-6">
                <div class="flex items-center gap-3">
                    <div class="w-10 h-10 bg-[#cba6f7] rounded-full flex items-center justify-center">
                        <svg class="w-5 h-5 text-[#1e1e2e]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                        </svg>
                    </div>
                    <h2 class="text-2xl font-bold text-[#cdd6f4]">Agents</h2>
                </div>
                <button
                    @click="$emit('close')"
                    class="text-[#9399b2] hover:text-[#cdd6f4] transition-colors duration-200"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>

            <!-- Agents List -->
            <div class="max-h-[400px] overflow-y-auto mb-6">
                <div v-if="agents.length === 0" class="text-center text-[#6c7086] py-8">
                    No agents in this channel yet.
                </div>
                <div
                    v-else
                    v-for="agent in agents"
                    :key="agent.id"
                    class="flex items-center justify-between p-3 bg-[#313244] rounded-lg mb-2 hover:bg-[#45475a] transition-colors duration-200"
                >
                    <div class="flex items-center gap-3">
                        <div class="w-10 h-10 bg-[#89b4fa] rounded-full flex items-center justify-center">
                            <svg class="w-5 h-5 text-[#1e1e2e]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                            </svg>
                        </div>
                        <div>
                            <div class="text-[#cdd6f4] font-medium">{{ agent.name }}</div>
                            <div class="text-[#9399b2] text-xs">{{ agent.type }} - {{ agent.model }}</div>
                        </div>
                    </div>
                    <button
                        @click="handleDeleteAgent(agent)"
                        class="text-[#9399b2] hover:text-[#f38ba8] transition-colors duration-200 p-2"
                        title="Delete agent"
                    >
                        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                        </svg>
                    </button>
                </div>
            </div>

            <!-- Add Agent Button -->
            <button
                @click="showCreateAgentForm = true"
                class="w-full py-3 bg-gradient-to-r from-[#cba6f7] to-[#94e2d5] hover:from-[#94e2d5] hover:to-[#89dceb] text-[#1e1e2e] rounded-lg font-medium transition-all duration-200 flex items-center justify-center gap-2"
            >
                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4" />
                </svg>
                Add Agent
            </button>

            <!-- Create Agent Form -->
            <div v-if="showCreateAgentForm" class="mt-6 p-4 bg-[#313244] rounded-lg">
                <h3 class="text-[#cdd6f4] font-medium mb-4">Create New Agent</h3>
                <form @submit.prevent="handleCreateAgent" class="space-y-4">
                    <div>
                        <label class="block text-sm text-[#bac2de] mb-1">Name</label>
                        <input
                            v-model="newAgent.name"
                            type="text"
                            required
                            class="w-full px-3 py-2 bg-[#45475a] border border-[#585b70] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#cba6f7] text-[#cdd6f4]"
                            placeholder="Agent name"
                        />
                    </div>
                    <div>
                        <label class="block text-sm text-[#bac2de] mb-1">Model</label>
                        <input
                            v-model="newAgent.model"
                            type="text"
                            required
                            class="w-full px-3 py-2 bg-[#45475a] border border-[#585b70] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#cba6f7] text-[#cdd6f4]"
                            placeholder="e.g., llama3.2"
                        />
                    </div>
                    <div>
                        <label class="block text-sm text-[#bac2de] mb-1">Prompt</label>
                        <textarea
                            v-model="newAgent.prompt"
                            required
                            rows="3"
                            class="w-full px-3 py-2 bg-[#45475a] border border-[#585b70] rounded-lg focus:outline-none focus:ring-2 focus:ring-[#cba6f7] text-[#cdd6f4] resize-none"
                            placeholder="Agent prompt..."
                        ></textarea>
                    </div>
                    <div v-if="errorMessage" class="text-[#f38ba8] text-sm">
                        {{ errorMessage }}
                    </div>
                    <div class="flex gap-2">
                        <button
                            type="button"
                            @click="showCreateAgentForm = false; errorMessage = ''"
                            class="flex-1 py-2 bg-[#45475a] hover:bg-[#585b70] text-[#cdd6f4] rounded-lg font-medium transition-colors duration-200"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            class="flex-1 py-2 bg-gradient-to-r from-[#cba6f7] to-[#94e2d5] hover:from-[#94e2d5] hover:to-[#89dceb] text-[#1e1e2e] rounded-lg font-medium transition-all duration-200"
                        >
                            Create
                        </button>
                    </div>
                </form>
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
    emits: ["close"],
    data() {
        return {
            showCreateAgentForm: false,
            newAgent: {
                name: "",
                model: "",
                prompt: "",
                type: "proxy",
                adapter: "ollama",
            },
            errorMessage: "",
        };
    },
    computed: {
        agents() {
            return this.$store.getters.getAgentsForActiveChannel;
        },
        activeChannelId() {
            const channel = this.$store.state.activeChannel;
            return channel ? channel.id : null;
        },
    },
    watch: {
        isVisible(newVal) {
            if (newVal) {
                this.fetchAgents();
            }
        },
    },
    methods: {
        async fetchAgents() {
            if (this.activeChannelId) {
                await this.$store.dispatch("fetchAgentsForChannel", this.activeChannelId);
            }
        },
        async handleCreateAgent() {
            if (!this.activeChannelId) return;

            this.errorMessage = "";
            try {
                await this.$store.dispatch("createAgent", {
                    chatId: this.activeChannelId,
                    agentData: this.newAgent,
                });
                this.showCreateAgentForm = false;
                this.newAgent = {
                    name: "",
                    model: "",
                    prompt: "",
                    type: "proxy",
                    adapter: "ollama",
                };
            } catch (error) {
                if (error.response && error.response.data) {
                    this.errorMessage = error.response.data.error || "Failed to create agent";
                } else {
                    this.errorMessage = "Failed to create agent";
                }
            }
        },
        async handleDeleteAgent(agent) {
            if (!this.activeChannelId) return;

            try {
                await this.$store.dispatch("deleteAgent", {
                    chatId: this.activeChannelId,
                    agentId: agent.id,
                });
            } catch (error) {
                console.error("Failed to delete agent:", error);
            }
        },
    },
};
</script>
