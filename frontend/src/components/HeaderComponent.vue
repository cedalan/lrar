<template>
  <div id="coffee-train-btn">
    <CoffeeTrainComponent/>
  </div>
    <div id="everything">
        <h1 id="welcome-text">Welcome to LRAR - Lazy Roomate Avoidance Registry!</h1>
        <button id="add-tenant-btn" @click="openSchema(true)">Add Tenant</button>
    </div>
    <TenantFormComponent
      v-if="showTenantForm"
      :reciever="HeaderComponent"
      @close="showTenantForm = null"
    />
</template>

<script lang="ts">
import { defineComponent } from 'vue';
import TenantFormComponent from '@/components/TenantFormComponent.vue';
import CoffeeTrainComponent from '@/components/CoffeeTrainComponent.vue';

export default defineComponent({
    name: "HeaderComponent",
    components: {
      CoffeeTrainComponent,
        TenantFormComponent,
    },
    data() {
        return {
            showTenantForm: null,
        }
    },
    methods: {
        openSchema(show: boolean) {
            this.showTenantForm = show;
        }
    }
})
</script>

<style scoped>
#everything {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 20px;
    text-align: center;
}

#coffee-train-btn {
  position: absolute;
  top: 10px;
  left: 10px;

}

#add-tenant-btn {
    position: absolute;
    top: 10px;
    right: 10px;
    background-color: #C2FBD7;
    border-radius: 50px;
    border: none;
    box-shadow: rgba(25, 25, 25, 0.04) 0 0 1px 0, rgba(0, 0, 0, 0.1) 0 3px 4px 0;
    color: #008000;
    cursor: pointer;
    font-family: Arial, sans-serif;
    font-size: 1em;
    height: 50px;
    padding: 0 25px;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
}

#add-tenant-btn:hover {
    transform: scale(1.05);
    box-shadow: rgba(0, 0, 0, 0.2) 0 4px 8px;
}

</style>