<template>
  <div v-if="tenants.length" class="all-tenants-div">
    <div v-for="tenant in tenants" :key="tenant.name" class="tenant-div">
      <h1> {{ tenant.name }} </h1>
      <img :src="tenant.image_url" alt="Tenant image">
      <p> {{ tenant.burn_count }} </p>
    </div>
  </div>
  <div v-else>
    <p>Error</p>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

interface Tenant {
  id: number,
  name: string,
  age: number,
  image_url: string,
  burn_count: number,
  dishwasher_count: number,
  favorite_quote: string
}

export default defineComponent ({
  name: "TenantComponent",
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
  .all-tenants-div {
    display: flex;
    gap: 20px;
    flex-wrap: nowrap;
    justify-content: space-evenly;
  }
  .tenant-div {
    flex: 0 0 calc(16.66% - 20px);
    background-color: lightcoral;
    padding: 20px;
    text-align: center;
    box-sizing: border-box; /* Include padding in width calculation */
  }
  .tenant-div img {
    width: 100%;
    height: 150px;
    object-fit: cover;
  }
</style>