<template>
    <div id="everything">
        <h1 id="welcome-text">Welcome to LRAR - Lazy Roomate Avoidance Registry!</h1>
        <button @click="openSchema(true)">Add Tenant</button>
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

export default defineComponent({
    name: "HeaderComponent",
    components: {
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
    flex-direction: row;
}

button {
    position: absolute;
    top: 0;
    right: 0;

    background-color: #C2FBD7;
    border-radius: 50px;
    border-width: 0;
    box-shadow: rgba(25, 25, 25, 0.04) 0 0 1px 0, rgba(0, 0, 0, 0.1) 0 3px 4px 0;
    color: #008000;
    cursor: pointer;
    display: inline-block;
    font-family: Arial, sans-serif;
    font-size: 1em;
    height: 50px;
    padding: 0 25px;
}
</style>