<template>
    <div>
        <div v-if="tenant">
            <h1> {{ tenant?.name }} </h1>
            <img :src="tenant?.image_path" alt="Tenant image">
            <p> {{ tenant?.burns }} </p>
            <p> {{ tenant?.image_path }}</p>
        </div>
        <div v-else>
            <p>Error</p>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

interface Tenant {
    name: string,
    image_path: string,
    burns: number,
}

export default defineComponent ({
    name: "TenantComponent",
    data() {
        return {
            tenant: null as unknown as Tenant,
        };
    },
    async created() {
    try {
      const response = await fetch("http://127.0.0.1:3001/tenants");

      if (!response.ok) {
        throw new Error("Network response was not ok: " + response.statusText);
      }

      const data: Tenant = await response.json(); // Type the fetched data as Tenant

      // Assign the response data to tenant
      this.tenant = data;

    } catch (error) {
      console.error("Error fetching tenant data:", error);
    }
  },
});
</script>