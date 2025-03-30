<template>
    <div class="tenant-form-overlay">
        <div class="tenant-form">
            <h2>Create a tenant</h2>

            <form @submit.prevent="submitTenant">
              <div>
                <label for="name">Tenant name:</label>
                <input v-model="tenant_name" type="text" required />
              </div>
              <div>
                <label for="age">Tenant age:</label>
                <input v-model="tenant_age" type="number" required />
              </div>
              <div>
                 <label for="image">Tenant image</label>
                <input 
                    type="file" 
                    ref="tenant_image" 
                    accept="image/*" 
                    id="image" 
                    required 
                    @change="handleFileUpload"
                />
              </div>
              <div>
                 <label for="favorite_quote">Tenant quote:</label>
                <input v-model="tenant_quote" type="text" required />
              </div>
               <div>
                 <button type="submit">Create tenant!</button>
                 <button type="button" @click="$emit('close')">Cancel</button>
               </div>
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
            tenant_age: null as number | null,
            tenant_image: null as File | null,
            tenant_burn_count: 0,
            tenant_dishwasher_count: 0,
            tenant_quote: "",
            tenant_current_burn_count: 0,
            isResizingImage: false, //Track if image is still being resized
        };
    },
    methods: {
        async handleFileUpload(event: Event) {
            const input = event.target as HTMLInputElement;
            if (!input.files || input.files.length === 0) {
                return;
            }
            const file = input.files[0];
            this.isResizingImage = true;
            try {
                //Resize image with helper function. Size can ofc be changed
                const squareFile = await this.resizeImage(file, 300);
                this.tenant_image = squareFile;
            } catch (err) {
                console.error("Error resizing image:", err);
                alert("Failed to process the image. Please try a different image.");
            } finally {
                this.isResizingImage = false;
            }
        },
        resizeImage(file: File, canvasSize: number) {
            return new Promise((resolve, reject) => {
                const reader = new FileReader();
                reader.onload = (e) => {
                    if (!e.target?.result) {
                        reject(new Error("Failed to read image file."));
                        return;
                    }
                    const img = new Image();
                    img.onload = () => {
                        const canvas = document.createElement("canvas");
                        canvas.width = canvasSize;
                        canvas.height = canvasSize;
                        const ctx = canvas.getContext("2d");

                        if (!ctx) {
                            reject(new Error("Failed to create canvas context."));
                            return;
                        }
                        
                        const minSide = Math.min(img.width, img.height);
                        const startX = (img.width - minSide) / 2;
                        const startY = (img.height - minSide) / 2;

                        ctx.drawImage(img, startX, startY, minSide, minSide, 0, 0, canvasSize, canvasSize);

                        canvas.toBlob((blob) => {
                            if (!blob) {
                                reject(new Error("Failed to convert image to blob."));
                                return;
                            }
                            const newFile = new File([blob], file.name, { type: file.type });
                            resolve(newFile)
                        }, 
                        "image/png", 
                        1.0
                        );
                    }
                    img.onerror = (error) => reject(error);
                    img.src = e.target.result as string;
                };
                reader.onerror = reject;
                reader.readAsDataURL(file);
            });
        },
        async submitTenant() {
            try {
                if (!this.tenant_name.trim()) {
                    alert("Tenant name is required.");
                    return;
                }
                if (this.tenant_name.indexOf(' ') >= 0) {
                    alert("Tenant can only have one name!")
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
                if (!this.tenant_image) {
                    alert("Please select an image.");
                    return;
                }

                const allowedTypes = ["image/jpeg", "image/png", "image/jpg"];
                if (!allowedTypes.includes(this.tenant_image.type)) {
                    alert(`${this.tenant_image.type} is an invalid file type. Please upload a jpg/jpeg or png image`);
                    return;
                }
                if (this.tenant_image.size > 10 * 1024 * 1024) { // 10MB limit, might be too big...
                    alert("File size must be under 10MB.");
                    return;
                }

                if (this.isResizingImage) {
                    alert("Please wait while the image is being resized...");
                    return;
                }
                
                // Prepare FormData for image upload
                const formData = new FormData();
                formData.append("tenant_image", this.tenant_image);
                
                const imageResponse = await fetch("http://127.0.0.1:3001/tenant_image", {
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

                console.log(tenantData);
                

                const response = await fetch("http://127.0.0.1:3001/tenant", {
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

<style scoped>
.tenant-form-overlay {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
}

.tenant-form {
    background: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
    width: 300px;
  }
</style>