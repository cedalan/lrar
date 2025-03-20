<template>
    <div class="tenant-burn-history-overlay"> 
      <div class="tenant-burn-history">
        <h3>Burn History for {{ tenant.name }}</h3>
        <div v-if="loading">Loading burn data...</div>
        <div v-else-if="error">{{ error }}</div>
        <div v-else>
          <p v-if="burns.length === 0">No burns found for this tenant.</p>
          <ul v-else class="burn-list"> 
            <li
              v-for="burn in burns"
              :key="burn.id"
              class="burn-item"       
            >
              <p>Reason: {{ burn.reason }}</p>
              <p>From: {{ burn.giver_name }}</p>
              <p>Created: {{ burn.created_at }}</p>
            </li>
          </ul>
        </div>
        <button class="close-burn-history" @click="$emit('close')">Close</button>
      </div>
    </div>
  </template>
  
  <script>
  export default {
    name: 'TenantBurnHistoryComponent',
    props: {
      tenant: {
        type: Object,
        required: true
      }
    },
    data() {
      return {
        burns: [],
        loading: false,
        error: null
      };
    },
    mounted() {
      this.fetchBurns();
    },
    methods: {
      fetchBurns() {
        this.loading = true;
        fetch(`http://localhost:3001/tenants/${this.tenant.id}/burns`)
          .then(response => {
            if (!response.ok) {
              return response.json().then(data => {
                throw new Error(data.error || `HTTP error ${response.status}`);
              });
            }
            return response.json();
          })
          .then(result => {
            this.burns = result.data || [];
          })
          .catch(err => {
            this.error = err.message || 'Unknown error';
          })
          .finally(() => {
            this.loading = false;
          });
      }
    }
  };
  </script>
  
  <style scoped>
  .tenant-burn-history-overlay {  
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
  
  .tenant-burn-history { 
    background: white;
    padding: 20px;
    border-radius: 8px;
    width: 400px; 
    max-height: 80vh; 
    overflow-y: auto; 
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
  }
  
  .burn-list { 
    margin-top: 1rem;
    padding: 0; 
  }
  
  .burn-item {
    list-style-type: none;    
    padding: 0.5rem;
    margin-bottom: 0.5rem;
    border-bottom: 1px solid #eee;
  }
  
  .close-burn-history {
    margin-top: 10px;
    background: #ccc;
    border: none;
    padding: 0.5rem 1rem;
    cursor: pointer;
  }
  </style>