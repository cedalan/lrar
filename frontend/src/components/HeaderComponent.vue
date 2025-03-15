<template>
    <div id="everything">
        <h1 id="welcome-text">Welcome to LRAR - Lazy Roomate Avoidance Registry!</h1>
        <button @click="openSchema(true)">+</button>
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