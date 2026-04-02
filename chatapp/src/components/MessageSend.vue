<template>
    <div
        class="flex flex-col bg-gradient-to-t from-[#313244] to-[#2a2a3c] border-t border-[#45475a] relative bottom-0"
    >
        <div class="flex items-center gap-3 p-3">
            <button
                @click="triggerFileUpload"
                :disabled="isUploading"
                class="p-2.5 text-[#9399b2] hover:text-[#89b4fa] hover:bg-[#45475a] rounded-lg focus:outline-none transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed"
            >
                <svg
                    v-if="!isUploading"
                    xmlns="http://www.w3.org/2000/svg"
                    class="w-5 h-5"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"
                    />
                </svg>
                <svg v-else class="animate-spin h-5 w-5 text-[#89b4fa]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
            </button>
            <input
                type="file"
                ref="fileInput"
                @change="handleFileUpload"
                multiple
                accept="image/*"
                class="hidden"
            />
        </div>

        <div class="px-3 pb-2">
            <textarea
                v-model="message"
                @keyup.enter.exact="sendMessage"
                placeholder="Type a message..."
                class="w-full px-4 py-3 text-sm bg-[#1e1e2e] text-[#cdd6f4] placeholder-[#6c7086] border border-[#45475a] rounded-xl focus:outline-none focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent resize-none transition-all duration-200"
                rows="3"
            ></textarea>
        </div>

        <div v-if="files.length > 0" class="flex flex-wrap px-3 pb-2 gap-2">
            <div v-for="file in files" :key="file.path" class="relative group">
                <img
                    :src="file.fullUrl"
                    class="h-20 w-20 object-cover rounded-lg border border-[#45475a] group-hover:border-[#89b4fa] transition-all duration-200"
                    alt="Uploaded image"
                />
                <button
                    @click="removeFile(file)"
                    class="absolute -top-2 -right-2 bg-[#f38ba8] text-white rounded-full p-1 opacity-0 group-hover:opacity-100 transition-opacity duration-200"
                >
                    <svg class="w-3 h-3" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                    </svg>
                </button>
            </div>
        </div>

        <button
            @click="sendMessage"
            :disabled="isSending || message.trim() === ''"
            class="absolute bottom-4 right-4 p-3 text-[#1e1e2e] bg-gradient-to-r from-[#89b4fa] to-[#74c7ec] rounded-full hover:from-[#74c7ec] hover:to-[#89dceb] focus:outline-none transition-all duration-200 transform hover:scale-110 shadow-lg disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
        >
            <svg v-if="!isSending"
                xmlns="http://www.w3.org/2000/svg"
                class="w-5 h-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
            >
                <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M5 12h14M12 5l7 7-7 7"
                />
            </svg>
            <svg v-else class="animate-spin h-5 w-5 text-[#1e1e2e]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
        </button>
    </div>
</template>

<script>
export default {
    data() {
        return {
            message: "",
            files: [],
            isSending: false,
            isUploading: false,
        };
    },
    computed: {
        userId() {
            return this.$store.state.user.id;
        },
        activeChannelId() {
            const channel = this.$store.state.activeChannel;
            if (!channel) {
                return null;
            }
            return channel.id;
        },
    },
    methods: {
        async sendMessage() {
            if (this.message.trim() === "" || this.isSending) return;

            this.isSending = true;

            const payload = {
                chatId: this.activeChannelId,
                content: this.message,
                files: this.files.map((file) => file.path),
            };

            console.log("Sending message:", payload);

            this.$store.dispatch("messageSent", {
                chatId: payload.chatId,
                type: "text",
                size: payload.content.length,
                totalFiles: payload.files.length,
            });

            try {
                await this.$store.dispatch("sendMessage", payload);
                this.message = ""; // Clear the input after sending
                this.files = []; // Clear the files after sending
            } catch (error) {
                console.error("Failed to send message:", error);
                // TODO: Show error notification to user
            } finally {
                this.isSending = false;
            }
        },
        triggerFileUpload() {
            this.$refs.fileInput.click();
        },
        async handleFileUpload(event) {
            const files = Array.from(event.target.files);
            if (files.length === 0) return;

            this.isUploading = true;

            try {
                const uploadedFiles = await this.$store.dispatch(
                    "uploadFiles",
                    files,
                );
                this.files = [...this.files, ...uploadedFiles];
            } catch (error) {
                console.error("Failed to upload files:", error);
            } finally {
                this.isUploading = false;
            }
        },
        removeFile(file) {
            this.files = this.files.filter(f => f.path !== file.path);
        },
    },
};
</script>
