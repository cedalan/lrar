<script setup>
import {ref, computed} from "vue";

const COFFEE_TRAIN_SCHEDULED_ICON = '/src/public/coffeeTrainScheduled_small.png'
const COFFEE_TRAIN_NOT_SCHEDULED_ICON = '/src/public/coffeeTrainUnscheduled_small.png'

const coffeeTrainIsScheduled = ref(JSON.parse(localStorage.getItem('coffeeTrainIsScheduled') ?? 'false'));
// MVP = pressing coffee train button makes happy smiley train. Next step is to make a popup and stuff to show where
// and when the coffee train is departing from
function toggleCoffeeTrainScheduled() {
  coffeeTrainIsScheduled.value = !coffeeTrainIsScheduled.value;
  localStorage.setItem('coffeeTrainIsScheduled',JSON.stringify(coffeeTrainIsScheduled.value))
}


const coffeeTrainIconPath = computed(() =>
  {return coffeeTrainIsScheduled.value ? COFFEE_TRAIN_SCHEDULED_ICON : COFFEE_TRAIN_NOT_SCHEDULED_ICON}
);

</script>

<template>
<button @click="toggleCoffeeTrainScheduled"><img :src="coffeeTrainIconPath" alt="coffee train!" /></button>
</template>

<style scoped>

</style>