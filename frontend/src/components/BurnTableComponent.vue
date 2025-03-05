<template>
    <div class="burn-table-overlay">
        <div v-for="tenant in tenants" :key="tenant.name" class="tenant-div">
            <div class="tenant-name"> 
                {{ tenant.name }} 
            </div>
            <div class="tenant-burns"> 
                {{ "🔥".repeat(tenant.current_burn_count ?? 0) }} 
            </div>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

interface Tenant {
  id: number,
  name: string,
  age: number,
  image_url: string,
  burn_count: number,
  dishwasher_count: number,
  favorite_quote: string
  weekly_chore: string,
  current_burn_count: number,
}

export default defineComponent({
    name: "BurnTableComponent",
    data() {
        return {
            tenants: [] as Tenant[],
        };
    },
    async created() {
    try {
      const response = await fetch("http://127.0.0.1:3001/tenants");

      if (!response.ok) {
        throw new Error("Network response was not ok: " + response.statusText);
      }

      const data: Tenant[] = await response.json(); // Type the fetched data as an array of Tenant

      // Assign the response data to tenants
      this.tenants = data;

    } catch (error) {
      console.error("Error fetching tenant data:", error);
    }
  },
});
</script>

<style scoped>
.burn-table-overlay {
  background-color: #fafafa;
  padding: 1rem;
  border-radius: 0.25rem;
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
  width: 100%;
}

.tenant-div {
  display: flex;
  align-items: center;
  justify-content: flex-start;
  transition: background-color 0.2s ease;
  padding: 0.5rem;
  border-radius: 0.25rem;
}

.tenant-div:hover {
  background-color: #f2f2f2;
}

.tenant-name {
  font-size: 1.1rem;
  font-weight: 600;
  margin-right: 1.5rem;
}

.tenant-burns {
  color: #FF5722;
  font-size: 1.5rem;
  user-select: none;
}
</style>
