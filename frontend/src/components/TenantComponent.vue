<template>
    <div>
        <div v-if="loading">Loading...</div>
        <div v-else-if="error"> {{ error }} </div>
        <div v-else>
            <p> {{ tenant?.name }} </p>
            <p> {{ tenant?.img }} </p>
            <p> {{ tenant?.burns }} </p>
        </div>
    </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

interface Tenant {
    name: string,
    imagePath: string,
    burns: number,
}

export default defineComponent ({
    name: "TenantComponent",
    data() {
        return {
            tentant: null as Tenant | null,
            loading: true,
            error: null as string | null,
        };
    },
    async created() {
        try {
            //Send request to backend and blah blah
            this.tentat.name = "Mr. Test";
            this.tenant.imagePath = "some/path/to/image";
            this.tenant.burns = 4;
        } catch (err: any) {
            this.error = err.message;
        } finally {
            this.loading = false;
        }
    }
});
</script>