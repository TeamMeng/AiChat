<template>
    <div class="flex items-center justify-center min-h-screen bg-gradient-to-br from-[#1e1e2e] to-[#181825] py-12">
        <div
            class="w-full max-w-md p-8 space-y-8 bg-gradient-to-br from-[#313244] to-[#45475a] rounded-2xl shadow-2xl border border-[#585b70]"
        >
            <!-- Header with Icon -->
            <div class="text-center space-y-4">
                <div class="flex justify-center">
                    <div class="w-16 h-16 bg-gradient-to-br from-[#a6e3a1] to-[#94e2d5] rounded-full flex items-center justify-center shadow-lg">
                        <svg class="w-8 h-8 text-[#1e1e2e]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z" />
                        </svg>
                    </div>
                </div>
                <h1 class="text-3xl font-bold text-[#cdd6f4]">
                    Create Your Account
                </h1>
                <p class="text-[#bac2de]">
                    Join us and start collaborating
                </p>
            </div>

            <!-- Error Message -->
            <div v-if="errorMessage" class="flex items-start gap-2 p-3 bg-[#f38ba8]/10 border border-[#f38ba8]/30 rounded-lg animate-shake">
                <svg class="w-5 h-5 text-[#f38ba8] flex-shrink-0 mt-0.5" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd" />
                </svg>
                <span class="text-[#f38ba8] text-sm">{{ errorMessage }}</span>
            </div>

            <form @submit.prevent="register" class="mt-8 space-y-5">
                <div class="space-y-2">
                    <label
                        for="fullName"
                        class="block text-sm font-medium text-[#bac2de]"
                        >Full Name</label
                    >
                    <input
                        type="text"
                        id="fullName"
                        v-model="fullName"
                        placeholder="Enter your full name"
                        required
                        class="mt-1 block w-full px-4 py-3 bg-[#1e1e2e] border border-[#45475a] rounded-lg text-[#cdd6f4] placeholder-[#6c7086] focus:outline-none focus:ring-2 focus:ring-[#a6e3a1] focus:border-transparent transition-all duration-200"
                    />
                </div>

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
                        class="mt-1 block w-full px-4 py-3 bg-[#1e1e2e] border border-[#45475a] rounded-lg text-[#cdd6f4] placeholder-[#6c7086] focus:outline-none focus:ring-2 focus:ring-[#a6e3a1] focus:border-transparent transition-all duration-200"
                    />
                </div>

                <div class="space-y-2">
                    <label
                        for="workspaceName"
                        class="block text-sm font-medium text-[#bac2de]"
                        >Workspace Name</label
                    >
                    <input
                        type="text"
                        id="workspaceName"
                        v-model="workspaceName"
                        placeholder="Enter your workspace name"
                        required
                        class="mt-1 block w-full px-4 py-3 bg-[#1e1e2e] border border-[#45475a] rounded-lg text-[#cdd6f4] placeholder-[#6c7086] focus:outline-none focus:ring-2 focus:ring-[#a6e3a1] focus:border-transparent transition-all duration-200"
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
                        class="mt-1 block w-full px-4 py-3 bg-[#1e1e2e] border border-[#45475a] rounded-lg text-[#cdd6f4] placeholder-[#6c7086] focus:outline-none focus:ring-2 focus:ring-[#a6e3a1] focus:border-transparent transition-all duration-200"
                    />
                </div>

                <button
                    type="submit"
                    :disabled="isLoading"
                    class="w-full flex justify-center items-center gap-2 py-3 px-4 border border-transparent rounded-lg shadow-sm text-sm font-medium text-[#1e1e2e] bg-gradient-to-r from-[#a6e3a1] to-[#94e2d5] hover:from-[#94e2d5] hover:to-[#89dceb] focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#a6e3a1] transition-all duration-200 transform hover:scale-[1.02] disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                >
                    <svg v-if="isLoading" class="animate-spin h-5 w-5 text-[#1e1e2e]" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                    </svg>
                    {{ isLoading ? 'Creating account...' : 'Register' }}
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
                Already have an account?
                <router-link
                    to="/login"
                    class="font-medium text-[#a6e3a1] hover:text-[#94e2d5] transition-colors duration-200"
                >
                    Login here
                </router-link>
            </p>
        </div>
    </div>
</template>

<script>
export default {
    data() {
        return {
            fullName: "",
            email: "",
            workspaceName: "",
            password: "",
            errorMessage: "",
            isLoading: false,
        };
    },
    methods: {
        async register() {
            this.errorMessage = "";
            this.isLoading = true;

            try {
                this.$store.dispatch("userRegister", {
                    email: this.email,
                    workspaceId: this.workspaceName,
                });

                const user = await this.$store.dispatch("signup", {
                    email: this.email,
                    fullname: this.fullName,
                    password: this.password,
                    workspace: this.workspaceName,
                });

                console.log("Signup successful, user:", user);
                this.$router.push("/"); // Redirect to chat after successful signup
            } catch (error) {
                console.error("Signup failed:", error);

                // Handle different error types
                if (error.response) {
                    if (error.response.status === 409) {
                        this.errorMessage = "This email is already registered. Please use a different email or login.";
                    } else if (error.response.status === 500) {
                        this.errorMessage = "Server error. Please try again later.";
                    } else {
                        this.errorMessage = error.response.data?.error || "Registration failed. Please try again.";
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
