<script setup lang="ts">
import TenantComponent from '@/components/TenantComponent.vue';
</script>

<template>
    <main>
        <TenantComponent @burn_submitted="this.$emit('burn_submitted')"/>
    </main>
</template>