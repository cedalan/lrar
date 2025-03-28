<template>
    <div class="burn-form-overlay">
      <div class="burn-form">
        <h2>Give a Burn to {{ receiver.name }}</h2>
  
        <form @submit.prevent="submitBurn">

          <div>
            <label for="giver_id">Who are you?:</label>
            <select v-model="giver_id" id="giver_id" required >
              <option disabled value="">select user</option>
              <option v-for="tenant in tenants" :key="tenant.id" :value="tenant.id">{{ tenant.name }}</option>
            </select>
          </div>>
          <div>
            <label for="reason">Reason:</label>
            <input v-model="reason" type="text" id="reason" required />
          </div>
          <div>
            <button type="submit">Submit Burn</button>
            <button type="button" @click="$emit('close')">Cancel</button>
          </div>

        </form>
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
    name: "BurnFormComponent",
    props: {
      receiver: {
        type: Object,
        required: true,
      },
    },
    data() {
      return {
        reason: "",
        giver_id: "",
        tenants: []
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
    methods: {
      async submitBurn() {
        console.log(this.receiver)
        const burnData = {
          reason: this.reason,
          receiver_id: this.receiver.id,
          giver_id: this.giver_id,
          created_at: new Date().toISOString().slice(0, 19), // Slice Milliseconds. Might be a better way to do this
        };

        console.log(burnData);
        
  
        try {
          const response = await fetch("http://localhost:3001/burn", {
            method: "POST",
            headers: {
              "Content-Type": "application/json",
            },
            body: JSON.stringify(burnData),
          });
  
          if (!response.ok) {
            throw new Error("Failed to submit burn");
          }
  
          alert("Burn submitted successfully!");
          this.$emit("burn_submitted")
          this.$emit("close");
        } catch (error) {
          console.error(error);
        }
      },
    },
  });
  </script>
  
  <style scoped>
  .burn-form-overlay {
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
  
  .burn-form {
    background: white;
    padding: 20px;
    border-radius: 8px;
    box-shadow: 0 4px 10px rgba(0, 0, 0, 0.3);
    width: 300px;
  }
  
  button {
    margin-top: 10px;
    margin-right: 5px;
  }
  </style>
  