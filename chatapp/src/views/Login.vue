<template>
    <div class="flex items-center justify-center min-h-screen bg-gradient-to-br from-[#1e1e2e] to-[#181825]">
        <div
            class="w-full max-w-md p-8 space-y-8 bg-gradient-to-br from-[#313244] to-[#45475a] rounded-2xl shadow-2xl border border-[#585b70]"
        >
            <!-- Header with Icon -->
            <div class="text-center space-y-4">
                <div class="flex justify-center">
                    <div class="w-16 h-16 bg-gradient-to-br from-[#89b4fa] to-[#74c7ec] rounded-full flex items-center justify-center shadow-lg">
                        <svg class="w-8 h-8 text-[#1e1e2e]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                        </svg>
                    </div>
                </div>
                <h1 class="text-3xl font-bold text-[#cdd6f4]">
                    Welcome Back
                </h1>
                <p class="text-[#bac2de]">
                    Sign in to your account
                </p>
            </div>

            <!-- Error Message -->
            <div v-if="errorMessage" class="flex items-start gap-2 p-3 bg-[#f38ba8]/10 border border-[#f38ba8]/30 rounded-lg animate-shake">
                <svg class="w-5 h-5 text-[#f38ba8] flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                </svg>
                <span class="text-[#f38ba8] text-sm">{{ errorMessage }}</span>
            </div>

            <form @submit.prevent="login" class="mt-8 space-y-6">
                <div class="space-y-2">
                    <label
                        for="email"
                        class="block text-sm font-medium text-[#bac2de]"
                        >Email</label
                    >
                    <input
                        type="email"
                        id="email"
                        v-model="email"
                        placeholder="Enter your email"
                        required
                        class="mt-1 block w-full px-4 py-3 bg-[#1e1e2e] border border-[#45475a] rounded-lg text-[#cdd6f4] placeholder-[#6c7086] focus:outline-none focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent transition-all duration-200"
                    />
                </div>

                <div class="space-y-2">
                    <label
                        for="password"
                        class="block text-sm font-medium text-[#bac2de]"
                        >Password</label
                    >
                    <input
                        type="password"
                        id="password"
                        v-model="password"
                        placeholder="Enter your password"
                        required
                        class="mt-1 block w-full px-4 py-3 bg-[#1e1e2e] border border-[#45475a] rounded-lg text-[#cdd6f4] placeholder-[#6c7086] focus:outline-none focus:ring-2 focus:ring-[#89b4fa] focus:border-transparent transition-all duration-200"
                    />
                </div>

                <button
                    type="submit"
                    :disabled="isLoading"
                    class="w-full flex justify-center items-center gap-2 py-3 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-[#1e1e2e] bg-gradient-to-r from-[#89b4fa] to-[#74c7ec] hover:from-[#74c7ec] hover:to-[#89dceb] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#89b4fa] transition-all duration-200 transform hover:scale-[1.02] disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                >
                    <svg v-if="isLoading" class="animate-spin h-5 w-5 text-[#1e1e2e]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    {{ isLoading ? 'Logging in...' : 'Login' }}
                </button>
            </form>

            <div class="relative">
                <div class="absolute inset-0 flex items-center">
                    <div class="w-full border-t border-[#585b70]"></div>
                </div>
                <div class="relative flex justify-center text-sm">
                    <span class="px-2 bg-[#313244] text-[#9399b2]">or</span>
                </div>
            </div>

            <p class="text-center text-sm text-[#bac2de]">
                Don't have an account?
                <router-link
                    to="/register"
                    class="font-medium text-[#89b4fa] hover:text-[#74c7ec] transition-colors duration-200"
                >
                    Register here
                </router-link>
            </p>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            email: "",
            password: "",
            errorMessage: "",
            isLoading: false,
        };
    },
    methods: {
        async login() {
            this.errorMessage = "";
            this.isLoading = true;

            try {
                const user = await this.$store.dispatch("signin", {
                    email: this.email,
                    password: this.password,
                });

                this.$store.dispatch("userLogin", {
                    email: this.email,
                });

                console.log("Login successful, user:", user);
                this.$router.push("/"); // Redirect to chat after successful login
            } catch (error) {
                console.error("Login failed:", error);

                // Handle different error types
                if (error.response) {
                    if (error.response.status === 403) {
                        this.errorMessage = "Invalid email or password. Please try again.";
                    } else if (error.response.status === 500) {
                        this.errorMessage = "Server error. Please try again later.";
                    } else {
                        this.errorMessage = error.response.data?.error || "Login failed. Please try again.";
                    }
                } else if (error.request) {
                    this.errorMessage = "Cannot connect to server. Please check your connection.";
                } else {
                    this.errorMessage = "An unexpected error occurred. Please try again.";
                }
            } finally {
                this.isLoading = false;
            }
        },
    },
};
</script>

<style scoped>
@keyframes shake {
    0%, 100% { transform: translateX(0); }
    10%, 30%, 50%, 70%, 90% { transform: translateX(-5px); }
    20%, 40%, 60%, 80% { transform: translateX(5px); }
}

.animate-shake {
    animation: shake 0.5s;
}
</style>
