<template>
    <div class="burn-form-overlay">
      <div class="burn-form">
        <h2>Give a Burn to {{ receiver.name }}</h2>
  
        <form @submit.prevent="submitBurn">
          <!-- User manually enters their giver_id -->
          <label for="giver_id">Your ID:</label>
          <input v-model="giver_id" type="number" id="giver_id" required />
  
          <label for="reason">Reason:</label>
          <input v-model="reason" type="text" id="reason" required />
  
          <button type="submit">Submit Burn</button>
          <button type="button" @click="$emit('close')">Cancel</button>
        </form>
      </div>
    </div>
  </template>
  
  <script lang="ts">
  import { defineComponent } from "vue";
  
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
        giver_id: null
      };
    },
    methods: {
      async submitBurn() {
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
  