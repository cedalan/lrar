<template>
  <div v-if="tenants.length" class="all-tenants-div">
    <div v-for="tenant in tenants" :key="tenant.name" class="tenant-div">
      <h2>{{ tenant.name }}, {{ tenant.age }} år</h2>
      <img :src="tenant.image_url" alt="Tenant image">
      <p>Tatt ut av oppvaskmaskinen {{ tenant.dishwasher_count }} ganger</p>
      <p>Burns: {{ tenant.current_burn_count }}</p>
      <p>Weekly chore: {{ tenant.weekly_chore.toLowerCase() }}</p>
      <p style="font-style:italic;">{{ tenant.favorite_quote }}</p>
      <button @click="increaseTenantDishwasherCount(tenant)">Took out dishes</button>
      <button @click="openBurnForm(tenant)">Give Burn</button>
      <button @click="showBurnHistory(tenant)">Show Burns</button>
    </div>
  </div>
  <div v-else>
    <p>Error</p>
  </div>
  <BurnFormComponent 
    v-if="burnFormSelectedTenant"
    :receiver="burnFormSelectedTenant"
    @close="burnFormSelectedTenant = null"
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
  gap: 20px;
  flex-wrap: wrap;
  justify-content: space-evenly;
}
.tenant-div {
  flex: 0 0 calc(33.33% - 20px);
  background-color: lightcoral;
  padding: 20px;
  text-align: center;
  box-sizing: border-box;
}
.tenant-div img {
  width: 200px;
  height: 200px;
  object-fit: cover;
}
</style>
