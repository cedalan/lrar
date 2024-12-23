<script setup>
    import TenantView from '@/views/TenantView.vue';
</script>
<template>
    <div id="everything">
        <h1 id="welcome-text">Welcome to LRAR - Lazy Roomate Avoidance Registry!</h1>
        <div id="tenant-view-container">
            <TenantView />
        </div>
    </div>
</template>

<style>
h1 {
    text-align: center;
}
#everything {
    display: flex;
    flex-direction: column; /* Stacks items vertically (h1 then content) */
    height: 100%; /* 100% of the viewport height */
    width: 100%; /* 100% of the viewport width */
    box-sizing: border-box;
    padding: 10px; /* Optional: adds padding inside the container */
}

#tenant-view-container {
    flex: 1;
}
</style>
