<template>
    <div class="tenant-form-overlay">
        <div class="tenant-form">
            <h2>Create a tenant</h2>

            <form @submit.prevet="submitTenant">
                <label for="name">Tenant name:</label>
                <input v-model="tenant_name" type="text" required />

                <label for="age">Tenant age:</label>
                <input v-model="tenant_age" type="number" required />

                <label for="image">Tenant image</label>
                <input type="file" name="tenant_image" accept="image/*" id="image" required />

                <label for="favorite_quote">Tenant quote:</label>
                <input v-model="tenant_quote" type="text" required />

                <button type="submit">Create tenant!</button>
                <button type="button" @click="$emit('close')">Cancel</button>
            </form>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
    name: "TenantFormComponent",
    data() {
        return {
            tenant_name: "",
            tenant_age: null,
            tenant_image: "",
            tenant_burn_count: 0,
            tenant_dishwasher_count: 0,
            tenant_quote: "",
            tenant_current_burn_count: 0,
        };
    },
    methods: {
        async submitTenant() {
            try {
                if (!this.tenant_name.trim()) {
                    alert("Tenant name is required.");
                    return;
                }
                if (!this.tenant_age || this.tenant_age <= 0) {
                    alert("Tenant age must be a positive number.");
                    return;
                }
                if (!this.tenant_quote.trim()) {
                    alert("Tenant quote is required.");
                    return;
                }
                if (!this.$refs.tenantImage || !this.$refs.tenantImage.files.length) {
                    alert("Please select an image.");
                    return;
                }

                // Prepare FormData for image upload
                const formData = new FormData();
                const file = this.$refs.tenantImage.files[0];

                const allowedTypes = ["image/jpeg", "image/png", "image/jpg"];
                if (!allowedTypes.includes(file.type)) {
                    alert("Invalid file type. Please upload a JPEG or PNG image.");
                    return;
                }
                if (file.size > 10 * 1024 * 1024) { // 10MB limit, might be too big...
                    alert("File size must be under 10MB.");
                    return;
                }

                formData.append("tenant_image", file);

                const imageResponse = await fetch("http://localhost:3001/tenant_image", {
                    method: "POST",
                    body: formData,
                });

                if (!imageResponse.ok) {
                    throw new Error("Failed to upload image. Please try again.");
                }

                const imageResult = await imageResponse.json();

                if (!imageResult.filename) {
                    throw new Error("Server did not return an image filename.");
                }

                // Construct tenant data
                const tenantData = {
                    name: this.tenant_name.trim(),
                    age: this.tenant_age,
                    image: imageResult.filename, // Filename returned from backend
                    burn_count: this.tenant_burn_count,
                    dishwasher_count: this.tenant_dishwasher_count,
                    favorite_quote: this.tenant_quote.trim(),
                    current_burn_count: this.tenant_current_burn_count,
                };

                const response = await fetch("http://localhost:3001/tenants", {
                    method: "POST",
                    headers: {
                        "Content-Type": "application/json",
                    },
                    body: JSON.stringify(tenantData),
                });

                if (!response.ok) {
                    throw new Error("Failed to submit tenant details.");
                }

                alert("Tenant successfully created!");
                this.$emit("close");

            } catch (error) {
                console.error("Error:", error);
                alert(error.message);
            }
        },
    },
})
</script>