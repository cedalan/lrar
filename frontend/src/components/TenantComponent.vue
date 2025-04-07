<template>
  <div v-if="tenants.length" class="all-tenants-div">
    <div v-for="tenant in tenants" :key="tenant.name" class="tenant-div">
      <button id="tenant-delete-button" @click="deleteTenant(tenant)">X</button>
      <h2>{{ tenant.name }}, {{ tenant.age }} år</h2>
      <img :src="tenant.image_url" alt="Tenant image">
      <p>Tatt ut av oppvaskmaskinen {{ tenant.dishwasher_count }} ganger</p>
      <p>Burns: {{ tenant.current_burn_count }}</p>
      <p>Weekly chore: {{ tenant.weekly_chore.toLowerCase() }}</p>
      <p style="font-style:italic;">{{ tenant.favorite_quote }}</p>
      <button @click="increaseTenantDishwasherCount(tenant)" class="normal-tenant-button">Took out dishes</button>
      <button @click="openBurnForm(tenant)">Give Burn</button class="normal-tenant-button">
      <button @click="showBurnHistory(tenant)">Show Burns</button class="normal-tenant-button">
    </div>
  </div>
  <div v-else>
    <p>Error</p>
  </div>
  <BurnFormComponent 
    v-if="burnFormSelectedTenant"
    :receiver="burnFormSelectedTenant"
    @close="burnFormSelectedTenant = null"
    @burn_submitted="this.$emit('burn_submitted')"
  />
  <TenantBurnHistoryComponent
    v-if="burnHistorySelectedTenant"
    :tenant="burnHistorySelectedTenant"
    @close="burnHistorySelectedTenant = null"
  />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import BurnFormComponent from '@/components/BurnFormComponent.vue';
import TenantBurnHistoryComponent from '@/components/TenantBurnHistoryComponent.vue';

interface Tenant {
  id: number;
  name: string;
  age: number;
  image_url: string;
  burn_count: number;
  dishwasher_count: number;
  favorite_quote: string;
  weekly_chore: string;
  current_burn_count: number;
}

export default defineComponent({
  name: 'TenantComponent',
  components: {
    BurnFormComponent,
    TenantBurnHistoryComponent
  },
  data() {
    return {
      tenants: [] as Tenant[],
      burnFormSelectedTenant: null as Tenant | null,
      burnHistorySelectedTenant: null as Tenant | null
    };
  },
  async created() {
    try {
      const response = await fetch('http://127.0.0.1:3001/tenants');
      if (!response.ok) {
        throw new Error('Network response was not ok: ' + response.statusText);
      }
      const data: Tenant[] = await response.json();
      this.tenants = data;
    } catch (error) {
      console.error('Error fetching tenant data:', error);
    }
  },
  methods: {
    async increaseTenantDishwasherCount(tenant: Tenant) {
      const response = await fetch(`http://127.0.0.1:3001/dishwasher_count/${tenant.id}`, {
        method: "PATCH"
      });

      if (!response.ok) {
        throw new Error("Some error came up when increasing tenant dishwasher count. Error: " + response.statusText);
      }
    },
    async deleteTenant(tenant: Tenant) {
      const confirmation = prompt(
        "Do you really want to delete this tenant? It cannot be undone! Write tenant name in the box and press confirm if you want to delete. Else press cancel"
      );
      if (confirmation?.toLowerCase() !== tenant.name.toLowerCase()) {
        alert("Wrong name!")
        return
      };
      const response = await fetch(`http://127.0.0.1:3001/tenant/${tenant.id}`, {
        method: "DELETE"
      })

      if (!response.ok) {
        throw new Error("Some error came when attempting to delete tenant. Error: " + response.statusText);
      }

      alert("Tenant deleted successfully!");
    },
    openBurnForm(tenant: Tenant) {
      this.burnFormSelectedTenant = tenant;
    },
    showBurnHistory(tenant: Tenant) {
      this.burnHistorySelectedTenant = tenant;
    }
  }
});
</script>

<style scoped>
.all-tenants-div {
  display: flex;
  gap: 1.25rem; 
  flex-wrap: wrap;
  justify-content: center;
  padding: 1rem;
}

.tenant-div {
  flex: 1 1 calc(33% - 15px);
  max-width: 450px;
  background-color: lightcoral;
  padding: 1.25rem;
  text-align: center;
  box-sizing: border-box;
  border-radius: 10px;
  position: relative;
}

.tenant-div img {
  width: 100%; 
  max-width: 10rem; 
  aspect-ratio: 1 / 1;
  object-fit: cover;
  border-radius: 5px;
}

.normal-tenant-buttons {
  background-color: #fff;
  color: #333;
  border: none;
  padding: 0.75rem 1.25rem;
  margin-top: 0.5rem;
  margin-left: 0.1rem;
  margin-right: 0.1rem;
  cursor: pointer;
  border-radius: 5px;
  transition: background-color 0.2s ease-in-out;
}

.normal-tenant-buttons:hover {
  background-color: rgba(255, 255, 255, 0.8);
}
#tenant-delete-button {
  position: absolute;
  top: 10px;
  right: 10px;
  background-color: #ff4040;
  color: white;
  border: none;
  border-radius: 50%;
  width: 30px;
  height: 30px;
  font-weight: bold;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
}

#tenant-delete-button:hover {
  background-color: #cc0000;
}
</style>
